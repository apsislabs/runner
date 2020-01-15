extern crate ctrlc;

use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixListener;
use std::process::Child;

pub fn run(matches: &clap::ArgMatches<'_>) {
    let cmd: Vec<&str> = matches.values_of("arguments").unwrap().collect();
    let name = matches.value_of("name").unwrap();
    let socket = format!("/tmp/runner.{}.sock", name);

    println!("listening on socket {}...", socket);
    let listener = UnixListener::bind(&socket).unwrap();

    // set the interrupt handler
    ctrlc::set_handler(move || {
        println!("\ngot an interrupt, cleaning up the socket {}", socket);
        let _ = std::fs::remove_file(&socket).unwrap();
        std::process::exit(0);
    })
    .expect("Error setting interrupt handler");

    // println!("auto starting process...");
    // let mut opt_child = Some(start_process(cmd.clone()));
    let mut opt_child = messages::handle_start(cmd.clone(), None);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                for line in stream.lines() {
                    // refresh the current process:
                    opt_child = check_process(opt_child);

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

fn check_process(child: Option<Child>) -> Option<Child> {
    return match child {
        Some(mut chi) => {
            let is_exited = chi.try_wait().unwrap().is_some();
            if is_exited {
                None
            } else {
                Some(chi)
            }
        }
        None => None,
    };
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
            "stop" => handle_stop(child),
            "start" => handle_start(cmd, child),
            "restart" => handle_restart(cmd, child),
            _ => child,
        };
    }

    fn handle_stop(child: Option<Child>) -> Option<Child> {
        println!("stopping process...");
        match child {
            Some(mut chi) => {
                let id = chi.id();
                super::stop_process(&mut chi);
                println!("\tprocess stopped (pid: {}).", id);
            }
            None => println!("\talready stopped."),
        }
        return None;
    }

    pub fn handle_start(cmd: Vec<&str>, child: Option<Child>) -> Option<Child> {
        println!("starting process...");
        return match child {
            Some(chi) => {
                println!("\talready running (pid: {}).", chi.id());
                Some(chi)
            }
            None => {
                let process = super::start_process(cmd.clone());
                println!("\tprocess started (pid: {}).", process.id());
                Some(process)
            }
        };
    }

    fn handle_restart(cmd: Vec<&str>, child: Option<Child>) -> Option<Child> {
        println!("restarting process...");
        let mut result = handle_stop(child);
        result = handle_start(cmd, result);

        return result;
    }
}
