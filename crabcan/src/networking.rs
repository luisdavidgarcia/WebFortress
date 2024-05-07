use crate::errors::Errcode;
use nix::fcntl::OFlag;
use nix::sched::{setns, CloneFlags};
use nix::unistd::Pid;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::process::Command;

pub fn setup_container_networking(container_pid: Pid, container_port: u16, host_port: u16) -> Result<String, Errcode> {
    _execute_command(Command::new("ip")
        .args(&["link", "add", "veth0", "type", "veth", "peer", "name", "veth1"]), 2)?;

    _execute_command(Command::new("ip")
        .args(&["link", "set", "veth1", "netns", &container_pid.to_string()]), 3)?;

    _execute_command(Command::new("ip")
        .args(&["addr", "add", "172.18.0.1/24", "dev", "veth0"]), 3)?;

    _execute_command(Command::new("ip")
        .args(&["link", "set", "veth0", "up"]), 4)?;

    // Change the namespace using setns - Works for Now
    let ns_path = format!("/proc/{}/ns/net", container_pid);
    let ns_file = File::open(&ns_path).map_err(|_| Errcode::NetworkError(10))?;
    let fd = ns_file.as_raw_fd();

    setns(fd, CloneFlags::CLONE_NEWNET).map_err(|_| Errcode::NetworkError(11))?;

    _execute_command(Command::new("ip")
        .args(&["addr", "add", "172.18.0.2/24", "dev", "veth1"]), 5)?;

    _execute_command(Command::new("ip")
        .args(&["link", "set", "veth1", "up"]), 6)?;

    _execute_command(Command::new("ip")
        .args(&["route", "add", "default", "via", "172.18.0.1"]), 7)?;

    // Forward traffic from host_port to container_port - Works, but very specific to the IP
    _execute_command(Command::new("iptables")
        .args(&["-t", "nat", "-A", "PREROUTING", "-p", "tcp", "--dport", &host_port.to_string(), "-j", "DNAT", "--to-destination", &format!("172.18.0.2:{}", container_port)]), 8)?;

    _execute_command(Command::new("iptables")
        .args(&["-t", "nat", "-A", "POSTROUTING", "-p", "tcp", "-d", "172.18.0.2", "--dport", &container_port.to_string(), "-j", "MASQUERADE"]), 9)?;

    Ok("172.18.0.2/24".to_string())
}

fn _execute_command(command: &mut Command, error_value: u8) -> Result<(), Errcode> {
    let status = command.status().map_err(|_| Errcode::NetworkError(error_value))?;
    if status.success() {
        Ok(())
    } else {
        println!("Failed to execute command: {:?}", command);  // Log the command that failed
        println!("Error value: {}", error_value);
        Err(Errcode::NetworkError(error_value))
    }
}
