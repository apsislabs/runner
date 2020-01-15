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

    println!("setting interrupt handler...");
    ctrlc::set_handler(move || {
        println!("\ngot an interrupt, cleaning up the socket {}", socket);
        let _ = std::fs::remove_file(&socket).unwrap();
        std::process::exit(0);
    })
    .expect("Error setting interrupt handler");

    println!("auto starting process...");
    let mut opt_child = Some(start_process(cmd.clone()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                for line in stream.lines() {
                    // refresh the current process:
                    opt_child = check_process(opt_child);

                    // now handle the given message
                    if let Ok(l) = line {
                        opt_child = handle_message(l.as_str(), cmd.clone(), opt_child);
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
    println!("starting process: {:?}", cmd.clone());
    let child = crate::process::start(cmd);
    let child = child.unwrap();
    println!("\tpid: {}", child.id());

    return child;
}

fn stop_process(child: &mut Child) {
    println!("killing child process: {}", child.id());
    child.kill().unwrap();
    println!("child has been killed");
}

fn restart_process(cmd: Vec<&str>, child: &mut Child) -> Child {
    stop_process(child);
    return start_process(cmd);
}

fn handle_message(message: &str, cmd: Vec<&str>, child: Option<Child>) -> Option<Child> {
    match &child {
        Some(chi) => println!("processing command {}, child pid: {}", message, chi.id()),
        None => println!("processing command {}, no child pid", message),
    };
    return match message {
        "stop" => {
            println!("stopping process...");

            match child {
                Some(mut chi) => {
                    println!("a process exists.");
                    stop_process(&mut chi);
                }
                None => {}
            }
            None
        }
        "start" => {
            println!("starting process...");
            match child {
                Some(chi) => {
                    println!("the process is already running. Not doing anything");
                    Some(chi)
                }
                None => {
                    println!("fake starting now...");
                    Some(start_process(cmd.clone()))
                }
            }
        }
        "restart" => {
            println!("restarting process");
            match child {
                Some(mut chi) => {
                    println!("the process is running. killing and restarting");
                    Some(restart_process(cmd, &mut chi))
                }
                None => {
                    println!("the process isn't running. Just starting it.");
                    Some(start_process(cmd.clone()))
                }
            }
        }
        _ => None,
    };
}
