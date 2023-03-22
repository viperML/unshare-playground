use nix::{
    mount::{mount, MsFlags},
    sched::{unshare, CloneFlags},
    sys::wait::waitpid,
    unistd::{fork, ForkResult, Pid},
    NixPath,
};

fn main() -> anyhow::Result<()> {
    unsafe {
        let fork_result = fork();

        match fork_result {
            Ok(ForkResult::Parent { child, .. }) => parent(child)?,
            Ok(ForkResult::Child) => child(),
            Err(err) => println!("{:?}", err),
        }
    }

    Ok(())
}

fn parent(child: Pid) -> anyhow::Result<()> {
    println!("Hello from parent!");

    let status = waitpid(child, None)?;

    println!("Child died: {:?}", status);

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
