# Define the variable for the binary path
BIN := "../target/debug/web_fortress"

# Check if CONFIG_FILE is set, otherwise print an error and exit
CONFIG_FILE := "test_python.toml"

all: 

test:
	cargo build
	sudo $(BIN) --config $(CONFIG_FILE)

get_syscalls:
	cargo build
	sudo strace -f -o syscall_web_fortress_output.txt $(BIN) --config $(CONFIG_FILE)

create_mountdir:
	set -e
	mkdir -p mountdir
	cp /bin/bash ./mountdir/

clean_network:
	sudo ip addr del 172.18.0.1/24 dev br0
	sudo ip link delete br0
	sudo ip link delete veth0
