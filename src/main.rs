extern crate clap;
use clap::{App, Arg, SubCommand};

mod commands;
mod process;

fn main() {
    let matches = App::new("runner")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("serve")
                .about("serves an application")
                .arg(Arg::with_name("name").index(1))
                .arg(Arg::with_name("arguments").multiple(true)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("serve") {
        commands::serve::run(matches);
    }
}
