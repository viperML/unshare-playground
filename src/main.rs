use nix::{
    mount::{mount, MsFlags},
    sched::{unshare, CloneFlags},
    sys::wait::waitpid,
    unistd::{fork, pipe, ForkResult, Pid},
    NixPath,
};

use std::{io::BufRead, os::unix::prelude::*};
use std::{io::BufReader, os::fd::RawFd};

fn main() -> anyhow::Result<()> {
    let (reader, writer) = os_pipe::pipe()?;

    unsafe {
        let fork_result = fork();

        match fork_result {
            Ok(ForkResult::Parent { child, .. }) => {
                drop(writer);
                println!("Hello from parent!");

                let reader = BufReader::new(reader);

                for line in reader.lines() {
                    println!("child wrote: {:?}", line);
                }

                let status = waitpid(child, None)?;

                println!("Child died: {:?}", status);
            }
            Ok(ForkResult::Child) => {
                drop(reader);
                nix::unistd::dup2(writer.as_fd().as_raw_fd(), nix::libc::STDOUT_FILENO)?;

                child();
            }
            Err(err) => println!("{:?}", err),
        }
    }

    Ok(())
}
fn child() {
    println!("Hello from child!");

    println!("Unsharing..");

    let flags = CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWNS;

    let result = unshare(flags);
    println!("unshare_result: {:?}", result);

    let none_typed: Option<&'static str> = None;

    let mount_flags = MsFlags::MS_PRIVATE | MsFlags::MS_BIND;
    let result = mount(
        Some("/tmp"),
        "/home/ayats/shelter",
        none_typed,
        mount_flags,
        none_typed,
    );
    println!("mount_result: {:?}", result);

    match std::fs::read_dir("/home/ayats/shelter") {
        Err(err) => println!("Failed to readdir: {:?}", err),
        Ok(r) => {
            for direntry in r {
                println!("Direnty: {:?}", direntry);
            }
        }
    }
}
