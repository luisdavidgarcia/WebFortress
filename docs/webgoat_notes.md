# General Notes on WebGoat

## Insights
<details>

One really weird insight is how for when you start a child process for execve 
you need to set the envrioment variable otherwise it will neer be able to find 
the Java shared libraries even though they exist, so in child.rs this had 
to be set:

```bash
let env_vars: Vec<CString> = vec![
    CString::new("LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib").unwrap(),
];
```

The `execve` function in Unix-like operating systems is used to execute a program. It replaces the current process image with a new process image specified by the `path` parameter. The function takes three arguments:

1. `path`: A string that specifies the path to the executable file.
2. `argv`: A vector of argument strings (including the program name as the first argument) to pass to the new process.
3. `envp`: A vector of environment variables to pass to the new process.

Here’s why each of these arguments is required:

### 1. `path`
This is the path to the executable file that you want to run. It must be a valid path to an executable binary.

### 2. `argv`
`argv` stands for "argument vector." It is a list of strings passed as command-line arguments to the new program. By convention, the first element of `argv` is the name of the program itself, and the subsequent elements are the arguments to the program.

For example, if you run `ls -l /home`, `argv` would be:
- `argv[0]`: `"ls"`
- `argv[1]`: `"-l"`
- `argv[2]`: `"/home"`

### 3. `envp`
`envp` stands for "environment pointer." It is a list of environment variables that the new program should use. Environment variables are key-value pairs that can affect the way running processes behave on a computer.

For example:
- `LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib`

When you use `execve`, you have to explicitly pass these environment variables because the new process will not inherit the environment of the calling process unless you do so. 

### Why `execve` Needs All Three Arguments

1. **Program Path**: `execve` needs to know what program to execute, hence the `path`.
2. **Argument Vector (`argv`)**: The program you are executing might need command-line arguments to function correctly. These arguments can control the behavior of the program.
3. **Environment (`envp`)**: The environment can influence how the program behaves. For example, `LD_LIBRARY_PATH` is used by the dynamic linker to find shared libraries. Without the correct environment, the program might not function as expected.

In your case, adding the `LD_LIBRARY_PATH` environment variable ensures that the dynamic linker can find the necessary shared libraries (`libjli.so` in this case), which is why the modified code works.

Here is a brief explanation of the modified part of your code:

```rust
// Log the environment variables if any
let env_vars: Vec<CString> = vec![
    CString::new("LD_LIBRARY_PATH=/usr/lib/jvm/java-17-openjdk-amd64/lib").unwrap(),
];
log::debug!("Environment variables: {:?}", env_vars);
```

This creates a `CString` vector with the necessary environment variable `LD_LIBRARY_PATH` and passes it to `execve`. This ensures that the new process created by `execve` has the correct environment settings, allowing it to locate and use the required shared libraries.

</details>