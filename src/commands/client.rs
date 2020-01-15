use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn send(matches: &clap::ArgMatches<'_>, message: &str) {
    let name = matches.value_of("name").unwrap();
    let socket = format!("/tmp/runner.{}.sock", name);

    println!("sending '{}' command to socket {}...", message, socket);

    let mut stream = UnixStream::connect(socket).unwrap();
    stream.write_all(message.as_bytes()).unwrap();
}
