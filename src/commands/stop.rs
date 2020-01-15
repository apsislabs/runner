use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn run(matches: &clap::ArgMatches<'_>) {
    let name = matches.value_of("name").unwrap();
    let socket = format!("/tmp/runner.{}.sock", name);

    println!("sending stop command to socket {}...", socket);

    let mut stream = UnixStream::connect(socket).unwrap();
    stream.write_all(b"stop").unwrap();
}
