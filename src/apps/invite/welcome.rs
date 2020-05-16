use clap::{App, Arg, ArgMatches};

use crate::http;

use colored::Colorize;

pub fn get_app() -> App<'static> {
    App::new("welcome")
        .about("Fetch the welcome screen of a guild from an invite")
        .version("1.0")
        .arg(Arg::with_name("invite")
            .takes_value(true)
            .index(1)
            .about("The invite to fetch the welcome screen from")
            .required(true)
        )
}

pub async fn handle_input(matches: &ArgMatches) {
    info!("Finding guild welcome screen");
    if let Some(matches) = matches.subcommand_matches("welcome") {
        let code = matches.value_of("invite").unwrap();

        let resp = http::get_invite(code).await;

        if let Ok(data) = resp {
            info!("Confirming existance of guild");
            if let Some(guild) = data.get("guild") {
                let guild_name = guild.get("name").unwrap().as_str().unwrap();

                info!("Confirming existence of welcome screen");
                if let Some(welcome) = guild.get("welcome_screen") {
                    println!("\n{}\n", guild_name.bold().underline().cyan());

                    let description = welcome.get("description").unwrap();

                    debug!("Fetching description");
                    let description_text = if let serde_json::value::Value::Null = description {
                        debug!("No description set.");
                        "No description set"
                    } else {
                        debug!("Found description");
                        description.as_str().unwrap()
                    };

                    println!("{}\n", description_text.italic());

                    for channel in welcome.get("welcome_channels").unwrap().as_array().unwrap() {
                        let chan_id = channel.get("channel_id").unwrap().as_str().unwrap();
                        let description = channel.get("description").unwrap().as_str().unwrap();

                        let emoji = if let serde_json::value::Value::Null = channel.get("emoji_id").unwrap() {
                            String::from(channel.get("emoji_name").unwrap().as_str().unwrap())
                        } else {
                            format!("CUSTOM EMOTE: {}", channel.get("emoji_id").unwrap().as_str().unwrap())
                        };

                        println!("{} - {} - {}", emoji, chan_id.dimmed(), description.bold());
                    }
                } else {
                    error!("Could not fetch welcome screen");
                }
            } else {
                error!("Could not fetch guild");
            }
        } else {
            error!("Invalid response from Discord: {:?}", resp);
        };
    }
}
