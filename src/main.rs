use std::thread;
use std::sync::{Arc, mpsc, Mutex};
use std::time::Duration;

struct Task {
    id: i32,
    payload: String,
}

struct Worker {
    id: i32,
}

impl Worker {
    fn process_task(&self, task:Task) -> String {
        format!("Worker {} processed task {} with payload {}", self.id, task.id, task.payload)
    }
}

fn create_task(id: i32, payload: String) -> Task {
    Task {
        id,
        payload,
    }
}

fn create_worker(id: i32) -> Worker {
    Worker {
        id,
    }
}

fn main() {
    let tasks: Vec<Task> = vec![
        create_task(1, "Task 1".to_string()),
        create_task(2, "Task 2".to_string()),
        create_task(3, "Task 3".to_string()),
        create_task(4, "Task 4".to_string()),
        create_task(5, "Task 5".to_string()),
        create_task(6, "Task 6".to_string()),
        create_task(7, "Task 7".to_string()),
        create_task(8, "Task 8".to_string()),
        create_task(9, "Task 9".to_string()),
        create_task(10, "Task 10".to_string()),
    ];

    let workers: Vec<Worker> = vec![
        create_worker(1),
        create_worker(2),
        create_worker(3),
        create_worker(4),
        create_worker(5),
        create_worker(6),
        create_worker(7),
        create_worker(8),
        create_worker(9),
        create_worker(10),
    ];

    let (tx, rx) = mpsc::channel();
    let (txResult, rxResult) = mpsc::channel();
    let rec = Arc::new(Mutex::new(rx));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];


    for worker in workers {
        let thread_tx = tx.clone();
        let thread_result = txResult.clone();
        let n_rec = Arc::clone(&rec);
        let handle = thread::spawn(move || {
            while let Ok(task) = n_rec.lock().unwrap().recv() {
                let result = worker.process_task(task);
                thread_result.send(result).unwrap();
            }
        });
        // thread::spawn(move || {
        //     while let Ok(task) = thread_tx.recv() {
        //         let result = worker.process_task(task);
        //         thread_result.send(result).unwrap();
        //     }
        // });
        handles.push(handle);
    }

    for task in tasks {
        tx.send(task).unwrap();
    }

    let timeout = Duration::from_secs(5);

    for _ in 0..handles.len() {
        match rxResult.recv_timeout(timeout) {
            Ok(message) => println!("{}", message),
            Err(_) => println!("Worker timeout"),
        }
    }
}