// see
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().expect("").expect("");

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", html_hello()),
        _ => ("HTTP/1.1 404 NOT FOUND", html_404()),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn html_hello() -> &'static str {
    include_str!("hello.html")
}
fn html_404() -> &'static str {
    include_str!("404.html")
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<crossbeam_channel::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // self.sender.cl
        drop(self.sender.take());
        for w in &mut self.workers {
            println!("shutting down worker {}", w.id);
            if let Some(t) = w.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = crossbeam_channel::unbounded();
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().expect("").send(job).expect("");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
// --snip--
impl Worker {
    fn new(id: usize, receiver: crossbeam_channel::Receiver<Job>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.recv() {
                println!("Worker {id} got a job; executing.");
                job();
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
