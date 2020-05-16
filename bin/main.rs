use clap::{Arg, App};

#[macro_use]
extern crate log;

mod apps;

use log::Level;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut app = App::new("discord")
        .version(VERSION)
        .author("Joe Banks <joseph@josephbanks.me>")
        .about(DESCRIPTION)
        .subcommand(apps::invite::get_app())
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .takes_value(false)
            .about("Sets the level of verbosity")
        );

    let matches = app.clone().get_matches();

    let level = match matches.occurrences_of("v") {
        0 => Level::Warn,
        1 => Level::Info,
        2 => Level::Debug,
        _ => Level::Trace
    };

    simple_logger::init_with_level(level).expect("Could not init logging");

    info!("Parsing subcommand");

    match matches.subcommand_name() {
        Some("invite") => apps::invite::handle_input(matches.clone()).await,
        None => app.print_help().expect("Could not build help message"),
        _ => error!("Unexpected subcommand"),
    }

    Ok(())
}
