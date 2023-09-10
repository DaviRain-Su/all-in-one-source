use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Message {
    pub title: String,
    pub link: String,
    pub contents: Vec<String>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for content in self.contents.iter() {
            writeln!(f, "{}", content)?;
        }
        writeln!(f,)
    }
}

impl Message {
    pub fn new(title: String, link: String, contents: Vec<String>) -> Self {
        Self {
            title,
            link,
            contents,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_empty() && self.contents.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Messages {
    pub messages: Vec<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }
}
