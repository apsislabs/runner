extern crate ctrlc;

use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::process::Child;
use std::path::Path;
use std::fs;

pub fn run(matches: &clap::ArgMatches<'_>) {
    let cmd: Vec<&str> = matches.values_of("arguments").unwrap().collect();
    let name = matches.value_of("name").unwrap();
    let force = matches.is_present("force");
    let socket = format!("/tmp/runner.{}.sock", name);

    let socket_exists = Path::new(&socket).exists();

    if socket_exists {
        if force {
            println!("The socket {} already exists. Attempting to replace it.", socket);
            fs::remove_file(&socket).unwrap();
        } else {
            println!("The socket {} already exists. To overwrite it run with --force.", socket);
            std::process::exit(1);
        }
    }

    let listener = match UnixListener::bind(&socket) {
        Ok(sock) => sock,
        Err(e) => {
            println!("an error occurred while binding to socket {:?}", e);
            std::process::exit(1);
        }
    };
    println!("listening on socket {}...", socket);

    // set the interrupt handler
    ctrlc::set_handler(move || {
        println!("\ngot an interrupt, cleaning up the socket {}", socket);
        let _ = std::fs::remove_file(&socket);
        std::process::exit(0);
    })
    .expect("Error setting interrupt handler");

    let mut opt_child = messages::handle_start(cmd.clone(), None, 0);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                for line in stream.lines() {
                    // refresh the current process:
                    opt_child = crate::process::is_running(opt_child);

                    // now handle the given message
                    if let Ok(l) = line {
                        opt_child = messages::handle_message(l.as_str(), cmd.clone(), opt_child);
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}

fn start_process(cmd: Vec<&str>) -> Child {
    let child = crate::process::start(cmd);
    let child = child.unwrap();

    return child;
}

fn stop_process(child: &mut Child) {
    child.kill().unwrap();
}

mod messages {
    use std::process::Child;
    pub fn handle_message(message: &str, cmd: Vec<&str>, child: Option<Child>) -> Option<Child> {
        return match message {
            "stop" => handle_stop(child, 0),
            "start" => handle_start(cmd, child, 0),
            "restart" => handle_restart(cmd, child, 0),
            _ => child,
        };
    }

    fn handle_stop(child: Option<Child>, indent: u32) -> Option<Child> {
        println!("{}stopping process...", t(indent));
        match child {
            Some(mut chi) => {
                let id = chi.id();
                super::stop_process(&mut chi);
                println!("{}process stopped (pid: {}).", t(indent + 1), id);
            }
            None => println!("{}already stopped.", t(indent + 1)),
        }
        return None;
    }

    pub fn handle_start(cmd: Vec<&str>, child: Option<Child>, indent: u32) -> Option<Child> {
        println!("{}starting process...", t(indent));
        return match child {
            Some(chi) => {
                println!("{}already running (pid: {}).", t(indent + 1), chi.id());
                Some(chi)
            }
            None => {
                let process = super::start_process(cmd.clone());
                println!("{}process started (pid: {}).", t(indent + 1), process.id());
                Some(process)
            }
        };
    }

    fn handle_restart(cmd: Vec<&str>, child: Option<Child>, indent: u32) -> Option<Child> {
        println!("{}restarting process...", t(indent));
        let mut result = handle_stop(child, indent + 1);
        result = handle_start(cmd, result, indent + 1);

        return result;
    }

    fn t(indent: u32) -> String {
        return (0..indent).map(|_| "\t").collect::<String>();
    }
}
