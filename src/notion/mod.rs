use notion::NotionApi;

const NOTION_API_KEY: &str = "secret_PsuNwSQtXp7Tw1YeRLA3vAJxell24OvFCBJ8ZdvNrAF";
const NOTION_PAGE_ID: &str = "634047dd0bac4604b0c69a2daabb6d8a";

pub async fn create_database() -> anyhow::Result<()> {
    let notion_client = NotionApi::new(NOTION_API_KEY.to_string())?;

    // notion_client.create_page(page)
    Ok(())
}
