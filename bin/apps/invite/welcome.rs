use clap::{App, Arg, ArgMatches};

use discord_api::get_invite;

use colored::Colorize;

pub fn get_app() -> App<'static> {
    App::new("welcome")
        .about("Fetch the welcome screen of a guild from an invite")
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

        let resp = get_invite(code).await;

        if let Ok(data) = resp {
            info!("Confirming existance of guild");
            if let Some(guild) = data.guild {
                info!("Checking if guild {} has welcome screen", guild.name);

                if let Some(welcome) = guild.welcome_screen {
                    println!("\n{}\n", guild.name.bold().underline().cyan());

                    debug!("Fetching description");
                    let description_text = if welcome.description.is_none() {
                        debug!("No description set.");
                        String::from("No description set")
                    } else {
                        debug!("Found description");
                        welcome.description.unwrap()
                    };

                    println!("{}\n", description_text.italic());

                    for channel in welcome.welcome_channels {
                        let chan_id = channel.channel_id;
                        let description = channel.description;

                        println!("{} - {} - {}", channel.emoji_name, chan_id.dimmed(), description.bold());
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
