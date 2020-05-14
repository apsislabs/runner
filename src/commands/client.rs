use std::io::prelude::*;
use std::os::unix::net::UnixStream;

pub fn send(matches: &clap::ArgMatches<'_>, message: &str) {
    let name = matches.value_of("name").unwrap();
    let socket = format!("/tmp/runner.{}.sock", name);

    println!("sending '{}' command to socket {}...", message, socket);

    let mut stream = match UnixStream::connect(socket) {
        Ok(sock) => sock,
        Err(e) => {
            println!("an error occurred while connecting to socket {:?}", e);
            return;
        }
    };
    stream.write_all(message.as_bytes()).unwrap();
}
