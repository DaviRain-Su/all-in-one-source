use scraper::{Html, Selector};

pub async fn get_section() -> anyhow::Result<()> {
    let response =
        reqwest::get("https://rustcc.cn/section?id=f4703117-7e6b-4caf-aa22-a3ad3db6898f").await?;

    let html = response.text().await?;

    println!("body: {}", html);

    // 使用scraper解析HTML
    let document = Html::parse_document(&html);

    // 创建一个选择器来选择所有的<a>标签
    let selector = Selector::parse("a").unwrap();

    // 找到所有的<a>标签并打印链接和文本内容
    for link in document.select(&selector) {
        let href = link.value().attr("href").unwrap_or("");
        let text = link.text().collect::<String>();
        if !text.contains("泰晓科技")
            || !text.contains("Ruby China")
            || !text.contains("电鸭远程社区")
            || !text.contains("IPFS中文社区")
            || !text.contains("鸣谢")
            || !text.contains("迅达云")
            || !text.contains("赛贝")
            || !text.contains("LongHash")
            || !text.contains("Forustm")
            || !text.contains("Rusoda")
            || !text.contains("Sapper")
            || !text.contains("ICP备")
        {
            println!("链接：{}", href);
            println!("文本内容：{}", text);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_get_section() {
    let r = get_section().await.unwrap();
}
