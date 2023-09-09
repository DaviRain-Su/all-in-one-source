pub mod display_notion;
pub mod logic;
pub mod notion_type;

use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use logic::process_task_range;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("output.json").unwrap();
    let mut file2 = File::create("numbers.txt").unwrap();
    let file1 = Arc::new(Mutex::new(file));

    // file.write_all("[".as_bytes()).unwrap();
    file1.lock().unwrap().write_all("[".as_bytes()).unwrap();

    let cpu_count = num_cpus::get();
    let task_count = 3431; // Total tasks to be processed
    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];
    let numbers = Arc::new(Mutex::new(vec![]));

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };

        let file = file1.clone();
        let numbers = numbers.clone();

        let task = tokio::spawn(async move {
            process_task_range(start, end, file, numbers).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    file1.lock().unwrap().write_all("]".as_bytes()).unwrap();
    let numbers = numbers.lock().unwrap();
    for number in numbers.iter() {
        file2.write_all(number.to_string().as_bytes()).unwrap();
        file2.write_all("\n".as_bytes()).unwrap();
    }

    Ok(())
}
