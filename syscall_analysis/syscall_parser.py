import sys
import re

def parse_strace_output(file_path):
    syscall_counts = {}
    with open(file_path, 'r') as f:
        for line in f:
            # Extract syscall name from each line of strace output
            syscall_match = re.match(r'^\d+\s+(\w+)\(', line)
            if syscall_match:
                syscall_name = syscall_match.group(1)
                # Increment count for this syscall
                syscall_counts[syscall_name] = syscall_counts.get(syscall_name, 0) + 1
    return syscall_counts

def write_syscall_counts_to_file(syscall_counts, output_file):
    with open(output_file, 'w') as f:
        for syscall, count in syscall_counts.items():
            f.write(f"{syscall}: {count}\n")

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python script_name.py input_file output_file")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    syscall_counts = parse_strace_output(input_file)
    write_syscall_counts_to_file(syscall_counts, output_file)
