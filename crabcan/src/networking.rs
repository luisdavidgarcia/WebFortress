use crate::errors::Errcode; 

use std::process::Command;

use nix::unistd::Pid;

pub fn setup_container_networking(container_pid: Pid) -> Result<String, Errcode> {
    _execute_command(Command::new("ip")
        .args(["link", "add", "name", "br0", "type", "bridge"]), 2)?;

    _execute_command(Command::new("ip")
        .args(["addr", "add", "172.18.0.1/24", "dev", "br0"]), 3)?;

    _execute_command(Command::new("ip")
        .args(["link", "set", "dev", "br0", "up"]), 4)?;

    _execute_command(Command::new("ip")
        .args(["link", "add", "vethA", "type", "veth", "peer", "name", "vethB"]), 5)?;

    _execute_command(Command::new("ip")
        .args(["link", "set", "vethA", "master", "br0"]), 6)?;

    _execute_command(Command::new("ip")
        .args(["link", "set", "vethA", "up"]), 7)?;

    _execute_command(Command::new("ip")
        .args(["link", "set", "vethB", "netns", &container_pid.to_string()]), 8)?;

    // Note: You might need to adjust this for the actual path or handling
    _execute_command(Command::new("nsenter")
        .args(["--net=/proc/&container_pid.to_string()/ns/net", "ip", "addr", "add", "172.17.0.2/24", "dev", "vethB"]), 9)?;

    _execute_command(Command::new("nsenter")
        .args(["--net=/proc/&container_pid.to_string()/ns/net", "ip", "link", "set", "dev", "vethB", "up"]), 10)?;

    _execute_command(Command::new("sysctl")
        .args(["-w", "net.ipv4.ip_forward=1"]), 11)?;

    _execute_command(Command::new("iptables")
        .args(["-t", "nat", "-A", "POSTROUTING", "-s", "172.17.0.0/24", "-o", "eth0", "-j", "MASQUERADE"]), 12)?;

    let ip_address = "172.17.0.2/24".to_string();

    Ok(ip_address)
}

// Helper function to handle command execution errors
fn _execute_command(command: &mut Command, error_value: u8) -> Result<(), Errcode> {
    command.status()
        .map_err(|e| Errcode::NetworkError(1)) // Convert any std::io::Error to your Errcode
        .and_then(|status| if status.success() {
            Ok(())
        } else {
            Err(Errcode::NetworkError(error_value))
        })
}