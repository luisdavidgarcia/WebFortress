import re
import sys
from collections import defaultdict

def parse_strace(strace_file):
    # Patterns to match syscalls and paths
    syscall_pattern = re.compile(r"^\d+\s+([a-zA-Z0-9_]+)\(")
    path_pattern = re.compile(r'\"(/.*?)\"')

    # Dictionaries to hold counts and paths
    syscall_counts = defaultdict(int)
    syscall_paths = defaultdict(set)

    # Parse the strace output file
    with open(strace_file, "r") as file:
        for line in file:
            # Match syscalls
            syscall_match = syscall_pattern.match(line)
            if syscall_match:
                syscall = syscall_match.group(1)
                syscall_counts[syscall] += 1
                
                # Match paths within the syscall
                paths = path_pattern.findall(line)
                for path in paths:
                    syscall_paths[syscall].add(path)

    # Print the counts of each syscall
    print("Syscall Counts:")
    for syscall, count in syscall_counts.items():
        print(f"{syscall}: {count}")

    # Print the paths for each syscall
    print("\nSyscall Paths:")
    for syscall, paths in syscall_paths.items():
        print(f"{syscall}:")
        for path in paths:
            print(f"  {path}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python parse_strace.py <strace_file>")
        sys.exit(1)

    strace_file = sys.argv[1]
    parse_strace(strace_file)
