/// Code snippet pulled from open-rs repo
/// https://github.com/Byron/open-rs/blob/main/src/macos.rs
use std::ffi::OsStr;
use std::process::Command;

pub fn commands<T: AsRef<OsStr>>(path: T) -> Vec<Command> {
    let mut cmd = Command::new("/usr/bin/open");
    cmd.arg(path.as_ref());
    vec![cmd]
}
