use clap::{crate_authors, crate_name, crate_version, App, AppSettings};
use everhour::status::status;
use std::process;

fn main() {
    let app = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("status"));

    let matches = app.clone().get_matches();

    let result = match matches.subcommand().unwrap() {
        ("status", _) => status(false),
        (command, _) => unreachable!("Invalid subcommand: {}", command),
    };

    match result {
        Ok(true) => process::exit(0),
        _ => process::exit(1),
    }
}
