all: test

test:
	cargo build
	sudo ./target/debug/crabcan --config test.toml

get_syscalls:
	sudo strace -f -o syscall_crabcan_output.txt ./target/debug/crabcan --config test.toml

clean_network:
	# sudo ip addr del 172.18.0.1/24 dev br0
	# sudo ip link delete br0 type bridge
	sudo ip link delete veth0
	sudo ip link delete veth1
