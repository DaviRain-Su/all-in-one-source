use crate::notion_type::*;
use serde_json::{json, Value};

pub fn display(notion_page: &NotionPage) -> Vec<Value> {
    let mut properties = Vec::new();

    for message in notion_page.properties.iter() {
        let title = json!({
            "type": "title",
            "title": [
                {
                    "type": "text",
                    "text": {
                        "content": message.title
                    }
                }
            ]
        });

        let link = json!({
            "type": "rich_text",
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": message.link
                    }
                }
            ]
        });

        let intro = json!({
            "type": "rich_text",
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": message.paragraphs
                    }
                }
            ]
        });

        let message_json = json!({
            "Title": title,
            "Link": link,
            "Intro": intro
        });
        properties.push(message_json)
    }

    properties
}
