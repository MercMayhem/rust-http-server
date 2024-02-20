use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(num: u8) -> ThreadPool {
        let mut threads: Vec<Option<JoinHandle<()>>> = vec![];

        let (sender, receiver) = channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..num {
            let move_receiver = Arc::clone(&receiver);
            threads.push(Some(thread::spawn(move || Self::thread_closure(move_receiver))));
        }

        ThreadPool {
            threads,
            sender: Some(sender),
        }
    }

    pub fn run<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.as_ref().map(|sender| sender.send(Box::new(job)).ok());
    }

    fn thread_closure(receiver: Arc<Mutex<Receiver<Job>>>) {
        loop {
            let locked_receiver = receiver.lock().unwrap();
            let job = locked_receiver.recv();

            match job {
                Ok(task) => task(),
                Err(_) => break,
            }
        }
        ()
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        drop(self.sender.take());
        
        for h in &mut self.threads{
            let handle = h.take().unwrap();
            handle.join().unwrap();
        }
    }
}
