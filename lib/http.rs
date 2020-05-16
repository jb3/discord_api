use reqwest::{Client, Error};

use crate::models::Invite;

const API_BASE: &str = "https://discord.com/api/v7";

/// Constructs a new client with a User-Agent compliant with the Discord API reference
fn create_client() -> Client {
    debug!("Creating new HTTP client");
    Client::builder()
        .user_agent("DiscordBot (https://github.com/jos-b/discord, 1.0)")
        .build()
        .expect("Could not build HTTP client")
}

/// Builds a route using the constant `API_BASE` to the specified path
fn build_route(path: &str) -> String {
    debug!("Building route to: {}", path);
    format!("{}{}", API_BASE, path)
}

/// Fetches an invite from the Discord API.
///
/// # Examples
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use discord_api::get_invite;
///
/// let guild_invite = get_invite("python").await.expect("Could not get invite, request error");
///
/// println!("Guild invite for: {}", guild_invite.guild.expect("No guild found!").name);
/// # }
/// ```
///
/// # Errors
///
/// In the event that the Discord API returns an non-200 response a reqwest error will be returned.
pub async fn get_invite<T: std::fmt::Display>(code: T) -> Result<Invite, Error> {
    info!("Starting request to get invite: {}", code);
    let invite: Invite = create_client()
        .get(&build_route(&format!("/invites/{}", code)))
        .send()
        .await?
        .json()
        .await?;

    Ok(invite)
}
