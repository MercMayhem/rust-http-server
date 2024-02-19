use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    num: u8,
    threads: Vec<JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(num: u8) -> ThreadPool {
        let mut threads: Vec<JoinHandle<()>> = vec![];

        let (sender, receiver) = channel::<Box<dyn FnOnce() + Send + 'static>>();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..num {
            let move_receiver = Arc::clone(&receiver);
            threads.push(thread::spawn(move || Self::thread_closure(move_receiver)));
        }

        ThreadPool {
            num,
            threads,
            sender,
        }
    }

    pub fn run<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.sender.send(Box::new(job));
    }

    fn thread_closure(receiver: Arc<Mutex<Receiver<Box<dyn FnOnce() + Send + 'static>>>>) {
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
