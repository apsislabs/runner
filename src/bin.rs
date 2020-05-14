extern crate clap;
use clap::{App, Arg, SubCommand};

use runnerlib::commands;

fn main() {
    let mut cli = App::new("runner").version("0.1.0").subcommand(
        SubCommand::with_name("serve")
            .about("serves an application")
            .arg(Arg::with_name("name").index(1))
            .arg(Arg::with_name("arguments").multiple(true)),
    );

    // add all the client subcommands
    let client_commands = vec!["stop", "start", "restart"];
    for sub_cmd in client_commands.iter() {
        cli = cli.subcommand(SubCommand::with_name(sub_cmd).arg(Arg::with_name("name").index(1)));
    }

    let matches = cli.get_matches();

    if let Some(matches) = matches.subcommand_matches("serve") {
        commands::serve::run(matches);
    }

    for sub_cmd in client_commands.iter() {
        if let Some(matches) = matches.subcommand_matches(sub_cmd) {
            commands::client::send(matches, sub_cmd);
        }
    }
}
