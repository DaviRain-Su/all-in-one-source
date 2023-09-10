use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub link: String,
    pub title: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub async fn content(&self) -> anyhow::Result<()> {
        let article_url = self.to_string();
        let response = reqwest::get(article_url).await?;
        let body = response.text().await?;
        println!("Body: {}", body);
        let document = Html::parse_document(&body);
        let selector = Selector::parse("div.detail-body").unwrap();

        if let Some(element) = document.select(&selector).next() {
            let title_and_contents_selector = Selector::parse("h3,p").unwrap();

            let title_and_contents = element.select(&title_and_contents_selector);

            for title_and_content in title_and_contents {
                // let title = title_and_content;
                let content = title_and_content.text().collect::<String>();
                println!("Content : {}", content);
            }
        }

        Ok(())
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

#[tokio::test]
async fn test_article_content() {
    let article = Article {
        link: "1ad7d23c-2392-4cce-9dc7-4bebcb3d51a5".to_string(),
        title: "【Rust日报】2023-09-09 Arroyo v0.5，高效地将流式数据传输到 S3".to_string(),
    };
    article.content().await.unwrap();
}
