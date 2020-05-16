use clap::App;

mod features;
mod welcome;

pub fn get_app() -> App<'static> {
    App::new("invite")
        .about("Fetch information about an invite")
        .version("1.0")
        .subcommand(features::get_app())
        .subcommand(welcome::get_app())
}

pub async fn handle_input(matches: clap::ArgMatches) {
    info!("Parsing subcommand");

    if let Some(matches) = matches.subcommand_matches("invite") {
        match matches.subcommand_name() {
            Some("features") => features::handle_input(matches).await, // oh no
            Some("welcome") => welcome::handle_input(matches).await,
            None => get_app()
                .print_help()
                .expect("Could not build help message"),
            _ => error!("Unexpected subcommand"),
        };
    }
}
