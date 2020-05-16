use clap::{App, Arg, ArgMatches};

use crate::http;

use colored::Colorize;

pub fn get_app() -> App<'static> {
    App::new("features")
        .about("Fetch the features of a guild from an invite")
        .version("1.0")
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

        let resp = http::get_invite(code).await;

        if let Ok(data) = resp {
            info!("Received okay response from Discord, checking existence of guild");
            if let Some(guild) = data.get("guild") {
                info!("Guild existed, checking for features");
                println!("Guild features for: {}", guild.get("name").unwrap().as_str().unwrap().cyan());
                if let Some(features) = guild.get("features") {
                    for feature in features.as_array().unwrap() {
                        println!("  \u{2022} {}", feature.as_str().unwrap());
                    }
                } else {
                    error!("Could not fetch features");
                }
            } else {
                error!("Could not fetch guild");
            }
        } else {
            error!("Invalid response from Discord: {:?}", resp);
        };
    }
}
