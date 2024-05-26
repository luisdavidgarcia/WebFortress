use crate::errors::Errcode;
use crate::container_config::ContainerOpts;
use crate::namespaces::userns;
use crate::hostname::set_container_hostname;
use crate::mounts::setmountpoint;
use crate::capabilities::setcapabilities;
use crate::syscalls::setsyscalls;

use nix::unistd::{Pid, close, execve};
use nix::sched::clone;
use nix::sys::signal::Signal;
use nix::sched::CloneFlags;
use std::ffi::CString;

const STACK_SIZE: usize = 1024 * 1024;
fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
    set_container_hostname(&config.hostname)?;
    setmountpoint(&config.mount_dir, &config.addpaths)?;
    userns(config.fd, config.uid)?;
    setcapabilities()?;
    setsyscalls()?;
    Ok(())
}

fn child(config: ContainerOpts) -> isize {
    match setup_container_configurations(&config) {
        Ok(_) => log::info!("Container set up successfully"),
        Err(e) => {
            log::error!("Error while configuring container: {:?}", e);
            return -1;
        }
    }

    if close(config.fd).is_err() {
        log::error!("Error while closing socket ...");
        return -1;
    }

    log::info!("Starting container with command {} and args {:?}", config.path.to_str().unwrap(), config.argv);

    // Check if the command exists within the new root
    let command_path = std::path::Path::new(config.path.to_str().unwrap());
    if !command_path.exists() {
        log::error!("Command not found: {:?}", command_path);
        return -1;
    } else {
        log::debug!("Command found: {:?}", command_path);
    }

    // Log the environment variables if any
    // let env_vars: Vec<CString> = vec![];
    let env_vars: Vec<CString> = vec![
        CString::new("LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib").unwrap(),
    ];

    log::debug!("Environment variables: {:?}", env_vars);

    match execve::<CString, CString>(&config.path, &config.argv, &env_vars) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("Error while trying to perform execve: {:?}", e);
            -1
        }
    }

}


pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);

    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32)
    )
    {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Errcode::ChildProcessError(0))
    }
}
