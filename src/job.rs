use anyhow::{Result, anyhow};
use std::sync::{Arc, RwLock, MutexGuard};
use std::thread;
use std::{future::Future, thread::JoinHandle, collections::HashMap};
use std::sync::{Mutex, mpsc};
use mpsc::{Sender, Receiver};

pub trait JobData<T>
where
    Self: Sized,
    T: Sized 
{
    fn empty() -> Self;
    fn new(value: T) -> Self;
    fn get(&self) -> Option<&T>;
}

struct JobDataValue<T> where T: Send {
    value: Arc<Option<T>>
}

//#![derive(Send)]
impl<T> JobData<T> for JobDataValue<T>
where 
    T: Sized + Send
{
    fn empty() -> Self {
        Self { value: Arc::new(None)}
    }

    fn new(value: T) -> Self {
        Self { value: Arc::new(Some(value)) }
    }

    fn get(&self) -> Option<&T> {
        match self.value.as_ref() {
            Some(val) => Some(val),
            None => None
        }
    }
}

pub trait JobRun
where Self: Sync
{
    fn run_job(&mut self) -> JobStatus;
}

#[derive(Clone)]
pub enum JobStatus {
    Waiting,
    Pending,
    Done(bool)
}

pub struct Job<In, Out, Handler> 
where
    In: Send,
    Out: Send,
    Handler: Send + 'static
{
    handle: &'static Handler,
    input: JobDataValue<In>,
    output: JobDataValue<Out>,
    status: JobStatus
}

impl<In, Out, Handler> Job<In, Out, Handler>
where
    In: Sized + Send,
    Out: Sized + Send,
    Handler: Fn(&mut Self) -> Box<dyn Future<Output = u32>> + Send + 'static
{
    pub fn new(handle: &'static Handler, data: In) -> Self {
        Self {
            handle: handle,
            input: JobDataValue::new(data),
            output: JobDataValue::empty(),
            status: JobStatus::Waiting,
        }
    }

    async fn run(&mut self) -> Result<&Out> {
        self.output = JobData::empty();
        
        let _ = Box::pin((self.handle)(self));

        if let JobStatus::Done(ret) = self.status {
            if ret {
                Ok(self.output.get().unwrap())
            } else {
                Err(anyhow!("job exit with failure"))
            }
        } else {
            Err(anyhow!(""))
        }
    }
}

impl<In, Out, Handler> JobRun for Job<In, Out, Handler>
where
    Self: Sync,
    In: Sized + Send,
    Out: Sized + Send,
    Handler: Fn(&mut Self) -> Box<dyn Future<Output = u32>> + Send + 'static
{
    fn run_job(&mut self) -> JobStatus {
        self.output = JobData::empty();
        let _ = Box::pin((self.handle)(self));

        self.status.clone()
    }
}

type JobRef<'a> = Arc::<dyn JobRun + 'a>;
type JobId = i32;

pub struct JobControl {

}

impl JobControl
where
    Self: Sync
{

}

pub struct JobWorker {
    join_handle: Option<JoinHandle<()>>,
    rx_chan: RwLock<Receiver::<JobControl>>,
    tx_chan: Mutex<Sender::<JobControl>>,
    current_job: Option<JobId>
}

impl JobWorker {
    pub fn new(rx_chan: Receiver<JobControl>, tx_chan: Sender<JobControl>) -> Self {
        JobWorker {
            join_handle: None,
            rx_chan: RwLock::new(rx_chan),
            tx_chan: Mutex::new(tx_chan),
            current_job: None
        }
    }

    pub fn start(mut self) {
        let join_handle= thread::spawn(move || {
            self.run();
        });
    }

    fn run(&mut self) {

    }
}

pub struct JobQueue<In, Out, Handler: 'static>
where
    In: Send,
    Out: Send,
    Handler: Send + 'static
{
    jobs: Mutex<HashMap<JobId, Job::<In, Out, Handler>>>,
    workers: Vec<JobWorker>,
    worker_limit: usize,
    job_id_counter: JobId
}

impl<In, Out, Handler> JobQueue<In, Out, Handler> 
where
    In: Sized + Send,
    Out: Sized + Send,
    Handler: Fn(&mut Job<In, Out, Handler>) -> Box<dyn Future<Output = u32>> + Send + Sync + 'static
{
    pub fn new(worker_limit: usize) -> Self {
        Self {
            jobs: Mutex::new(HashMap::new()),
            workers: Vec::with_capacity(worker_limit),
            worker_limit,
            job_id_counter: 0
        }
    }

    pub fn add_job(&mut self, handle: &'static Handler, input: In) -> JobId {
        let mut jobs = self.jobs.lock().expect("lock");
        jobs.insert(self.job_id_counter, Job::new(handle, input));
        
        let id = self.job_id_counter;
        self.job_id_counter += 1;
        
        id
    }
} 
