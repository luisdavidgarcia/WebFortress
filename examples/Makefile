# Define the variable for the binary path
BIN := "../target/debug/web_fortress"

# Check if CONFIG_FILE is set, otherwise print an error and exit
ifndef CONFIG_FILE
$(error CONFIG_FILE is not set. Please provide a configuration file, e.g., 'make CONFIG_FILE=your_config.toml')
endif

all: test

test:
	cargo build
	sudo $(BIN) --config $(CONFIG_FILE)

get_syscalls:
	sudo strace -f -o syscall_web_fortress_output.txt $(BIN) --config $(CONFIG_FILE)

clean_network:
	sudo ip addr del 172.18.0.1/24 dev br0
	sudo ip link delete br0
	sudo ip link delete veth0
