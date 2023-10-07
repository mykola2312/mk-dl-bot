use anyhow::{Result, anyhow};
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock, MutexGuard};
use std::thread;
use std::{future::Future, thread::JoinHandle, collections::HashMap};
use std::sync::{Mutex, mpsc};
use mpsc::{Sender, Receiver};

type JobId = i32;
type JobData<In, Out> = (In, Box<dyn Fn(In) -> Out + Send>);

enum WorkerMessage<In, Out> {
    PollJob,
    JobRun((JobId, JobData<In, Out>)),
    JobDone((JobId, Out))
}

pub struct JobQueue<In, Out> {
    jobs: HashMap<JobId, JobData<In, Out>>,
    chan_tx: Sender<WorkerMessage<In, Out>>,
    chan_rx: Receiver<WorkerMessage<In, Out>>,
    workers: Vec<JoinHandle<()>>,
    max_workers: usize,
    job_counter: JobId
}

impl<In, Out> JobQueue<In, Out>
where
    In: Sized + Send,
    Out: Sized + Send
{
    pub fn new(max_workers: usize) -> Self {
        let (chan_tx, chan_rx) = channel::<WorkerMessage<In, Out>>();
        Self {
            jobs: HashMap::new(),
            chan_tx,
            chan_rx,
            workers: Vec::new(),
            max_workers,
            job_counter: 0
        }
    }

    pub fn add_worker(&mut self) {
        let (tx, rx) = channel::<WorkerMessage<In, Out>>();
        self.workers.push(thread::spawn(move || {
            
        }));
    }
}