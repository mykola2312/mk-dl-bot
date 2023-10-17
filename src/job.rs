use anyhow::{anyhow, Result};
use mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;
use std::sync::{mpsc, Mutex};
use std::sync::{Arc, MutexGuard, RwLock};
use std::thread;
use std::{collections::HashMap, future::Future, thread::JoinHandle};

type JobId = i32;
type JobData<In, Out> = (In, Arc<dyn Fn(In) -> Out + Send + Sync + 'static>);

#[derive(Clone)]
enum WorkerMessage<In, Out> {
    PollJob,
    JobRun((JobId, JobData<In, Out>)),
    JobDone((JobId, Out)),
}

type WorkerChannel<In, Out> = (
    Sender<WorkerMessage<In, Out>>,
    Receiver<WorkerMessage<In, Out>>,
);
pub struct JobQueue<In, Out> {
    jobs: HashMap<JobId, JobData<In, Out>>,
    workers: Vec<(WorkerChannel<In, Out>, JoinHandle<()>)>,
    max_workers: usize,
    job_counter: JobId,
}

impl<In, Out> JobQueue<In, Out>
where
    In: Sized + Send + 'static,
    Out: Sized + Send + 'static,
{
    pub fn new(max_workers: usize) -> Self {
        Self {
            jobs: HashMap::new(),
            workers: Vec::new(),
            max_workers,
            job_counter: 0,
        }
    }

    fn new_worker_channel() -> (WorkerChannel<In, Out>, WorkerChannel<In, Out>) {
        let mut queue_chan: WorkerChannel<In, Out> = channel();
        let mut worker_chan = (queue_chan.0.clone(), queue_chan.1);

        (queue_chan, worker_chan)
    }

    pub fn add_worker(&mut self) {
        let ( queue_chan, mut worker_chan) = Self::new_worker_channel();
        self.workers.push((
            queue_chan,
            thread::spawn(move || {
                let mut chan = worker_chan;
                chan.0.send(WorkerMessage::PollJob).expect("send")
            }),
        ));
    }
}
  