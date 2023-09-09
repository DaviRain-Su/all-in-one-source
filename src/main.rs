pub mod display_notion;
pub mod logic;
pub mod notion_type;

use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use notion_type::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("output.json").unwrap();
    let mut file2 = File::create("numbers.txt").unwrap();
    let file1 = Arc::new(Mutex::new(file));

    // file.write_all("[".as_bytes()).unwrap();
    file1.lock().unwrap().write_all("[".as_bytes()).unwrap();

    let mut tasks = vec![];

    let numbers = Arc::new(Mutex::new(vec![]));

    for id in 0..=3430 {
        // for id in 3430u64..=3430 {
        let file = file1.clone();
        let numbers = numbers.clone();
        let task = tokio::spawn(async move {
            let html = reqwest::get(format!("http://old.rebase.network/posts/{}", id)).await;
            if let Ok(html) = html {
                if let Ok(ret) = html.text().await {
                    numbers.lock().unwrap().push(id);
                    let title = format!("Web3极客日报-{}", id);
                    let ret = parse_pages(title, &ret);
                    if !ret.properties.is_empty() {
                        let json_v = display_notion::display(&ret);
                        for msg in json_v.iter() {
                            println!("{}", serde_json::to_string_pretty(&msg).unwrap());
                            let json_data = msg.to_string();
                            // println!("{}", json_data);

                            file.lock()
                                .unwrap()
                                .write_all(json_data.to_string().as_bytes())
                                .unwrap();
                            file.lock().unwrap().write_all(",".as_bytes()).unwrap();
                        }
                    }
                }
            };
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
