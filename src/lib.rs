use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub mod db_mock;
pub mod db_object;
pub mod db_object_enum;
mod utils;
use db_object_enum::DataObjectEnum;
use utils::*;

use serde::{Deserialize, Serialize};

pub fn run_server(address: &str, db: Arc<Mutex<DataObjectEnum>>) {
    let listener = TcpListener::bind(address).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let database = Arc::clone(&db);

        pool.execute(|| {
            handle_connection(stream, database);
        });
    }
}

fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<DataObjectEnum>>) {
    let mut buf_reader = BufReader::new(&stream);
    let mut request = String::new();

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        if line == "\r\n" {
            break;
        }
        request.push_str(&line);
    }

    let content_length = request
        .lines()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split(" ").nth(1))
        .and_then(|len| len.parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).unwrap();
    let data = String::from_utf8(body).unwrap();

    let request_line = request.split("\r\n").next().unwrap();
    let mut request_data = request_line.split(" ");
    let method = request_data.next().unwrap();
    let path = request_data.next().unwrap();
    let controller = UserController::new(db);

    let (code, contents) = match (method, path) {
        ("GET", "/users") => (Some(200), controller.show_users()),
        ("GET", path) if path.starts_with("/users/") => {
            let id = path.trim_start_matches("/users/");
            if let Ok(user_id) = id.parse::<u32>() {
                (Some(200), controller.show_user(user_id))
            } else {
                (None, Err(Errors::UserError(400)))
            }
        }
        ("POST", "/users") => {
            println!("{}", data);
            match serde_json::from_str::<HashMap<String, String>>(data.as_str()) {
                Ok(user) => {
                    let id = controller.add_user(user, None);
                    (Some(201), id)
                }
                Err(_) => (None, Err(Errors::UserError(400))),
            }
        }
        ("PATCH", path) if path.starts_with("/users/") => {
            let id = path.trim_start_matches("/users/");
            if let Ok(user_id) = id.parse::<u32>() {
                match serde_json::from_str::<HashMap<String, String>>(data.as_str()) {
                    Ok(user) => (Some(204), controller.change_user_data(user_id, user)),
                    Err(_) => (None, Err(Errors::UserError(400))),
                }
            } else {
                (None, Err(Errors::UserError(400)))
            }
        }
        ("DELETE", path) if path.starts_with("/users/") => {
            let id = path.trim_start_matches("/users/");
            if let Ok(user_id) = id.parse::<u32>() {
                (Some(204), controller.delete_user(user_id))
            } else {
                (None, Err(Errors::UserError(400)))
            }
        }
        _ => (None, Err(Errors::UserError(404))),
    };

    let (status_line, contents) = match contents {
        Ok(data) => (format!("HTTP/1.1 {}", code.unwrap()), data),
        Err(Errors::ServerError(code)) => (
            format!("HTTP/1.1 {}", code),
            "Internal serve error".to_string(),
        ),
        Err(Errors::UserError(code)) => (
            format!("HTTP/1.1 {}", code),
            match code {
                400 => "Invalid input".to_string(),
                404 => "Not found".to_string(),
                _ => "User error".to_string(),
            },
        ),
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub lastname: String,
    pub birth_year: u16,
    pub group: UserGroup,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum UserGroup {
    User,
    Premium,
    Admin,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
