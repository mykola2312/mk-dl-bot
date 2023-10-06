use anyhow::{Result, anyhow};
use std::{future::Future, marker::PhantomData, sync::Mutex, thread::JoinHandle, collections::HashMap};

pub trait JobData<T>: Sized + Send + From<T> where T: Sized + Send {
    fn empty() -> Self;
    fn new(value: T) -> Self;
    fn get(&self) -> Option<&T>;
}

struct JobDataValue<T> {
    value: Option<T>
}

//#![derive(Send)]
impl<T> JobData<T> for JobDataValue<T> where T: Sized + Send {
    fn empty() -> Self {
        Self { value: None}
    }

    fn new(value: T) -> Self {
        Self::from(value)
    }

    fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> From<T> for JobDataValue<T> {
    fn from(value: T) -> Self {
        Self { value: Some(value) }
    }
}

impl<T> Into<Option<T>> for JobDataValue<T> {
    fn into(self) -> Option<T> {
        self.value
    }
}

pub enum JobStatus {
    Waiting,
    Pending,
    Done(bool)
}

//#[async_trait]
pub struct Job<In, Out, Handler: 'static> {
    handle: &'static Handler,
    input: JobDataValue<In>,
    output: JobDataValue<Out>,
    status: JobStatus
}

impl<In, Out, Handler> Job<In, Out, Handler>
where
    In: Sized + Send,
    Out: Sized + Send,
    Handler: Fn(&mut Self) -> Box<dyn Future<Output = u32>> + 'static
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


type JobId = i32;
pub struct JobQueue<In, Out, Handler: 'static>
{
    jobs: Mutex<HashMap<JobId, Job::<In, Out, Handler>>>,
    workers: Vec<JoinHandle<()>>,
    worker_limit: usize,
    job_id_counter: JobId
}

impl<In, Out, Handler> JobQueue<In, Out, Handler> 
where
    In: Sized + Send,
    Out: Sized + Send,
    Handler: Fn(&mut Job<In, Out, Handler>) -> Box<dyn Future<Output = u32>> + Send
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
