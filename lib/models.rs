//! Module containing various structures received from the Discord API.
//!
//! Structures here can be created manually though it is advised to fetch actual data
//! using the methods exposed by the http module (which are rexported to discord_api)

use serde::Deserialize;

/// A struct representing a Discord guild invite, fetched through [`discord_api::get_invite`](../http/fn.get_invite.html)
#[derive(Deserialize, Debug)]
pub struct Invite {
    /// The Discord guild the invite points to
    pub guild: Option<InviteGuild>,
}

/// A struct representing a guild that an invite points to
#[derive(Deserialize, Debug)]
pub struct InviteGuild {
    /// The name of the Guild
    pub name: String,
    /// A list of feature flags the guild has
    pub features: Vec<String>,
    /// The configured welcome screen for public guilds
    pub welcome_screen: Option<WelcomeScreen>,
}

/// The welcome screen presented to a user when they join the guild
#[derive(Deserialize, Debug)]
pub struct WelcomeScreen {
    /// A description of the server displayed in the welcome screen
    pub description: Option<String>,
    /// A list of channels which are displayed in the welcome screen
    pub welcome_channels: Vec<WelcomeChannel>,
}

/// A channel listed in the welcome screen
#[derive(Deserialize, Debug)]
pub struct WelcomeChannel {
    /// The ID of the channel
    pub channel_id: String,
    /// The description of the channel
    pub description: String,
    /// The emoji ID of the channel (None if not a custom emote)
    pub emoji_id: Option<String>,
    /// The emoji, if unicode it is the unicode character else it is the name of the custom emote
    pub emoji_name: String,
}
