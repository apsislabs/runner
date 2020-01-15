use std::process::{Child, Command};

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
