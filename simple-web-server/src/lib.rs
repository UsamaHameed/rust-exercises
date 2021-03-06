use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

impl ThreadPool {
	/// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);
		
		let (sender, receiver) = mpsc::channel();
		let mut workers = Vec::with_capacity(size);
		
		let receiver = Arc::new(Mutex::new(receiver));

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F) 
		where F: FnOnce() + Send + 'static 
	{
		let job = Box::new(f);
		self.sender.send(job).unwrap();
	}
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || {
			while let Ok(job) = receiver.lock().unwrap().recv() {
				println!("Worker {} got a job, executing...", id);
				thread::sleep_ms(5000);
				job();
			};
		});

		Worker { id, thread }
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;