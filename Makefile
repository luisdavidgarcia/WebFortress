# Define the variable for the binary path
BIN := "./target/debug/web_fortress"

all: test

test:
	cargo build
	sudo $(BIN) --config test.toml

get_syscalls:
	sudo strace -f -o syscall_web_fortress_output.txt $(BIN) --config test.toml

clean_network:
	sudo ip addr del 172.18.0.1/24 dev br0
	sudo ip link delete br0
	sudo ip link delete veth0
