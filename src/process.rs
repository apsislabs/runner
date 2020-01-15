use std::process::Command;
pub fn start<'a, I>(cmd: I) -> Option<u32>
where
    I: IntoIterator<Item = &'a str>,
{
    println!("running a command");
    let mut itr = cmd.into_iter();

    let first = itr.next();

    if let Some(cmd) = first {
        let mut child_cmd = Command::new(cmd);
        // println!("cmd is {}", cmd);

        // println!("args:");
        for el in itr {
            child_cmd.arg(el);
            // println!("\t{}", el);
        }

        println!("cmd: {:?}", child_cmd);

        let child = child_cmd.spawn(); //.expect("failed to spawn the command");

        if let Result::Ok(child) = child {
            println!("child process pid: {}", child.id());
            return Some(child.id());
        }

        return None;
    }

    return None;
}
