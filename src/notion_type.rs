use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotionPage {
    pub title: String,
    pub properties: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub title: String,
    pub link: String,
    pub paragraphs: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            link: "".to_string(),
            paragraphs: "".to_string(),
        }
    }
}

pub fn parse_pages(title: String, html: &str) -> NotionPage {
    let mut properties = vec![];
    let mut default_message = Message::default();

    let document = Html::parse_document(html);
    let selector = Selector::parse(".entry.themeform").unwrap();

    for entry_element in document.select(&selector) {
        let title_element = entry_element
            .select(&Selector::parse("strong").unwrap())
            .next();
        let link_element = entry_element.select(&Selector::parse("a").unwrap()).next();

        if let Some(title) = title_element.map(|e| e.text().collect::<String>()) {
            // println!("{}", title);
            if title.contains("3. ") || title.contains("2. ") || title.contains("1. ") {
                // println!("{}", title);
                let title = title.replace("1. ", "");
                let title = title.replace("2. ", "");
                let title = title.replace("3. ", "");

                default_message.title = title;
            }
            // default_message.title = title;
        }

        if let Some(link) = link_element.map(|e| e.value().attr("href").unwrap()) {
            // println!("{}", link);

            default_message.link = link.to_string();
        }

        let paragraphs_slector = Selector::parse("p").unwrap();
        let paragraphs = entry_element.select(&paragraphs_slector).skip(1);

        for paragraph_element in paragraphs {
            let text = paragraph_element.text().collect::<String>();

            let title_element = paragraph_element
                .select(&Selector::parse("strong").unwrap())
                .next();

            if let Some(title) = title_element.map(|e| e.text().collect::<String>()) {
                if !title.contains("Web3极客日报") && title.contains("3. ")
                    || title.contains("2. ")
                    || title.contains("1. ")
                {
                    // println!("{}", title);
                    let title = title.replace("1. ", "");
                    let title = title.replace("2. ", "");
                    let title = title.replace("3. ", "");

                    default_message.title = title;

                    // println!("{}", title);
                    // default_message.title = title;
                }
            }

            let link_element = paragraph_element
                .select(&Selector::parse("a").unwrap())
                .next();

            if let Some(link) = link_element.map(|e| e.value().attr("href").unwrap()) {
                // println!("{}", link);
                default_message.link = link.to_string();
            } else if !text.is_empty()
                && !text.contains("Web3极客日报")
                && !text.contains("https://rebase.network")
                && !text.contains("rebase_network")
            {
                // println!("{}", text);
                let text = text.replace("\n\u{a0}", "");
                default_message.paragraphs = text;
                // 检查是否存在相同的页面
                let is_duplicate = properties.iter().any(|msg: &Message| {
                    msg.title == default_message.title && msg.link == default_message.link
                });
                if !is_duplicate {
                    properties.push(default_message.clone());
                }
            } else {
                println!();
            }
        }

        println!();
    }

    // Return the extracted pages
    NotionPage { title, properties }
}

pub async fn create_notion_page(
    client: &Client,
    token: &str,
    _database_id: &str,
    page: &NotionPage,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.notion.com/v1/pages".to_string();

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let response = client.post(&url).headers(headers).json(page).send().await?;

    if response.status().is_success() {
        println!("Success!");
    } else {
        println!("Error!");
    }

    // Handle the response here
    // You can check the status code and the response body to ensure the page was created successfully

    Ok(())
}
