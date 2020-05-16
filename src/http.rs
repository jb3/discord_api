use reqwest::{Error, Client};

const API_BASE: &str = "https://discord.com/api/v7";

fn create_client() -> Client {
    debug!("Creating new HTTP client");
    Client::builder()
        .user_agent("DiscordBot (https://github.com/jos-b/discord, 1.0)")
        .build().expect("Could not build HTTP client")
}

fn build_route(path: &str) -> String {
    debug!("Building route to: {}", path);
    format!("{}{}", API_BASE, path)
}

pub async fn get_invite(code: &str) -> Result<serde_json::Value, Error> {
    info!("Starting request to get invite: {}", code);
    let content: serde_json::Value = create_client()
        .get(&build_route(&format!("/invites/{}", code)))
        .send()
        .await?
        .json()
        .await?;

    Ok(content)
}
