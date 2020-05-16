use clap::{App, Arg, ArgMatches};

use discord_api::get_invite;

use colored::Colorize;

pub fn get_app() -> App<'static> {
    App::new("features")
        .about("Fetch the features of a guild from an invite")
        .arg(Arg::with_name("invite")
            .takes_value(true)
            .index(1)
            .about("The invite to fetch features from")
            .required(true)
        )
}

pub async fn handle_input(matches: &ArgMatches) {
    info!("Finding guild features");
    if let Some(matches) = matches.subcommand_matches("features") {
        let code = matches.value_of("invite").unwrap();

        let resp = get_invite(code).await;

        if let Ok(invite) = resp {
            info!("Received okay response from Discord, checking existence of guild");
            if let Some(guild) = invite.guild {
                info!("Guild existed, checking for features");
                println!("Guild features for: {}", guild.name.cyan());

                for feature in guild.features {
                    println!("  \u{2022} {}", feature);
                }
            } else {
                error!("Could not fetch guild");
            }
        } else {
            error!("Invalid response from Discord: {:?}", resp);
        };
    }
}
