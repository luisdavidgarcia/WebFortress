# Define the variable for the binary path
BIN := "../target/debug/web_fortress"

CONFIG_FILE := "test_python.toml"

all: 

bash_test:
	cargo build
	sudo $(BIN) --config configuration_files/bash_test.toml

python_test:
	cargo build
	sudo $(BIN) --config configuration_files/python_test.toml

prepare_dirs:
	sudo mkdir -p /var/run /var/lock
	sudo chown -R www-data:www-data /var/run /var/lock

dvwa_test: prepare_dirs clean_veth
	cargo build
	sudo $(BIN) --config configuration_files/dvwa_test.toml

get_syscalls:
	cargo build
	sudo strace -f -o syscall_web_fortress_output.txt $(BIN) --config $(CONFIG_FILE)

create_mountdir:
	set -e
	mkdir -p mountdir
	cp /bin/bash ./mountdir/

clean_network:
	sudo ip addr del 172.18.0.1/24 dev veth0
	sudo ip link delete veth0

clean_veth:
	sudo ip link delete veth0 || true
	sudo ip link delete veth1 || true
