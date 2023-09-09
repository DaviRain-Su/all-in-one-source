pub mod api;
pub mod display_notion;
pub mod hellath_check;
pub mod logic;
pub mod types;

use logic::process_task_range;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file = File::create("output.json").unwrap();
    let file1 = Arc::new(Mutex::new(file));

    file1.lock().unwrap().write_all("[".as_bytes()).unwrap();

    let cpu_count = num_cpus::get();
    let task_count = api::total_count().await?; // Total tasks to be processed
    let tasks_per_thread = task_count / cpu_count;

    let mut tasks = vec![];

    for i in 0..cpu_count {
        let start = i * tasks_per_thread;
        let end = if i == cpu_count - 1 {
            task_count - 1
        } else {
            (i + 1) * tasks_per_thread - 1
        };

        let file = file1.clone();

        let task = tokio::spawn(async move {
            process_task_range(start, end, file).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    file1.lock().unwrap().write_all("]".as_bytes()).unwrap();

    Ok(())
}
