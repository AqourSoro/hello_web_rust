use std::
{
    sync::
    {
        mpsc, Arc, Mutex
    }, 
    thread
};


struct Worker
{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker 
{
    fn new(id: usize, reveiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(||{ reveiver; });

        Worker { id, thread }
    }    
}

//struct  Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool
{
    // threads: Vec<thread::JoinHandle<()>> // JoinHandle will execute the mission immediately, not suitiable for Pool.
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}


impl ThreadPool 
{
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size:usize) -> ThreadPool
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        //let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size
        {
            //create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool
        {
            //threads
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F:FnOnce() + Send + 'static
    {

    }
}