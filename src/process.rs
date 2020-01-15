use std::process::{Child, Command, ExitStatus};

pub fn start<'a, I>(cmd: I) -> Option<Child>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut itr = cmd.into_iter();

    let first = itr.next();

    if let Some(cmd) = first {
        let mut child_cmd = Command::new(cmd);
        for el in itr {
            child_cmd.arg(el);
        }

        let child = child_cmd.spawn();

        if let Result::Ok(child) = child {
            return Some(child);
        }
    }

    return None;
}

pub fn run<'a, I>(cmd: I) -> Option<ExitStatus>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut itr = cmd.into_iter();

    let first = itr.next();

    if let Some(cmd) = first {
        let mut child_cmd = Command::new(cmd);
        for el in itr {
            child_cmd.arg(el);
        }

        let child = child_cmd.status();

        if let Result::Ok(child) = child {
            return Some(child);
        }
    }

    return None;
}

pub fn stop(pid: u32) -> Option<()> {
    println!("stopping process pid: {}", pid);
    return Some(());
}
