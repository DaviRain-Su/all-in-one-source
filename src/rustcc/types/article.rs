use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub link: String,
    pub title: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = format!("https://rustcc.cn/article?id={}", self.link);
        write!(f, "{}", url)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleList {
    pub article_list: Vec<Article>,
}

impl ArticleList {
    pub fn push(&mut self, article: Article) {
        self.article_list.push(article);
    }
}
