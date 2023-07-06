/// Code snippets pulled from open-rs repo
/// https://github.com/Byron/open-rs/blob/main/src/lib.rs
use std::ffi::OsStr;
use std::io;
use std::process::Command;
use std::process::Stdio;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as os;

trait CommandExt {
    fn status_without_output(&mut self) -> io::Result<std::process::ExitStatus>;
}

impl CommandExt for Command {
    fn status_without_output(&mut self) -> io::Result<std::process::ExitStatus> {
        self.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
    }
}

trait IntoResult<T> {
    fn into_result(self, cmd: &Command) -> T;
}

impl IntoResult<io::Result<()>> for io::Result<std::process::ExitStatus> {
    fn into_result(self, cmd: &Command) -> io::Result<()> {
        match self {
            | Ok(status) if status.success() => Ok(()),
            | Ok(status) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Launcher {cmd:?} failed with {:?}", status),
            )),
            | Err(err) => Err(err),
        }
    }
}

pub fn open_url(path: impl AsRef<OsStr>) -> io::Result<()> {
    let mut last_err = None;
    for mut cmd in commands(path) {
        match cmd.status_without_output() {
            | Ok(status) => {
                return Ok(status).into_result(&cmd);
            },
            | Err(err) => last_err = Some(err),
        }
    }
    Err(last_err.expect("no launcher worked, at least one error"))
}

fn commands(path: impl AsRef<OsStr>) -> Vec<Command> {
    os::commands(path)
}
