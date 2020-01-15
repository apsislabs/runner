use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
// use std::sync::mpsc::channel;
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(matches: &clap::ArgMatches<'_>) {
    println!("serving");

    let cmd: Vec<&str> = matches.values_of("arguments").unwrap().collect();
    let name = matches.value_of("name").unwrap();

    // let mut child = start_process(cmd.clone());
    // let mut has_child = true;

    let mut opt_child = Some(start_process(cmd.clone()));

    let socket = format!("/tmp/runner.{}.sock", name);

    // child.into_inner();

    // let mut child_option: Option<&mut Child> = Some(&mut child);

    // let child = Arc::new(Mutex::new(start));

    println!("listening on socket {}...", socket);
    let listener = UnixListener::bind(socket).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = BufReader::new(stream);
                for line in stream.lines() {
                    if let Ok(l) = line {
                        // let opt = match has_child {
                        //     true => Some(child),
                        //     false => None,
                        // };

                        // match opt {
                        //     Some(_) => println!("opt with some"),
                        //     None => println!("opt with none"),
                        // };

                        opt_child = handle_message(l.as_str(), cmd.clone(), opt_child);
                        // match new_child_opt {
                        //     Some(new_child) => {
                        //         child = new_child;
                        //         has_child = true;
                        //     }
                        //     None => {
                        //         has_child = false;
                        //     }
                        // };
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
    println!("starting process: {:?}", cmd.clone().into_iter());
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
    // match child {
    //     Some(chi) => println!(
    //         "processing command {}, child pid: {}",
    //         message,
    //         (&mut *chi).id()
    //     ),
    //     None => println!("processing command {}, no child pid", message),
    // };
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
