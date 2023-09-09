use crate::display_notion;
use crate::notion_type::*;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn process_task_range(
    start: u64,
    end: u64,
    file: Arc<Mutex<File>>,
    numbers: Arc<Mutex<Vec<u64>>>,
) {
    for id in start..=end {
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

                        file.lock()
                            .unwrap()
                            .write_all(json_data.to_string().as_bytes())
                            .unwrap();
                        file.lock().unwrap().write_all(",".as_bytes()).unwrap();
                    }
                }
            }
        }
    }
}
