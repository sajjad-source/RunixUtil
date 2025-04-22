# RunixUtil

A collection of Rust implementations of common Unix utilities. Each utility is prefixed with 'r' to denote its Rust implementation.

## Utilities

### recho

A simple echo command that writes arguments to standard output.

```bash
# Basic usage
cargo run -- "Hello, World!"

# Without trailing newline
cargo run -- -n "Hello, World!"

# Show help and available options
cargo run -- --help
```

### rcat

A file concatenation utility that reads files sequentially and writes them to standard output.

```bash
# Basic usage
cargo run -- file.txt

# Number all lines
cargo run -- -n file.txt

# Number non-blank lines
cargo run -- -b file.txt

# Squeeze multiple empty lines
cargo run -- -s file.txt

# Read from stdin
cat file.txt | cargo run

# Show help and available options
cargo run -- --help
```

### rcp

A file copying utility that copies the contents of source files to target files.

```bash
# Basic usage
cargo run -- source.txt target.txt

# Force overwrite
cargo run -- -f source.txt target.txt

# Interactive prompt before overwrite
cargo run -- -i source.txt target.txt

# Prevent overwrite
cargo run -- -n source.txt target.txt

# Verbose output
cargo run -- -v source.txt target.txt

# Show help and available options
cargo run -- --help
```

### rhead

A utility that displays the first lines of files.

```bash
# Show first 10 lines (default)
cargo run -- file.txt

# Show first N lines
cargo run -- -n 5 file.txt

# Show from multiple files
cargo run -- file1.txt file2.txt

# Read from stdin
cat file.txt | cargo run

# Show help and available options
cargo run -- --help
```

### rmv

A file moving utility that renames files.

```bash
# Basic usage
cargo run -- source.txt target.txt

# Force overwrite
cargo run -- -f source.txt target.txt

# Interactive prompt before overwrite
cargo run -- -i source.txt target.txt

# Prevent overwrite
cargo run -- -n source.txt target.txt

# Show help and available options
cargo run -- --help
```

## Building and Running

Each utility is a separate Cargo project. To use any utility:

1. Navigate to the utility's directory:

```bash
cd <utility-name>
```

2. Build the project:

```bash
cargo build
```

3. Run the utility:

```bash
# Basic usage
cargo run -- [options] [arguments]

# Show help (lists all available options and descriptions)
cargo run -- --help
```

4. Or run the built binary:

```bash
./target/debug/<utility-name> [options] [arguments]
```

## Features

- Written in Rust for safety and performance
- Follows Unix utility conventions
- Implements common options and behaviors
- Proper error handling and exit codes
- Consistent with original Unix utilities
- Built-in help system (use --help flag)

## Error Handling

All utilities follow standard Unix conventions:

- Exit code 0 for success
- Exit code 1 for errors
- Error messages printed to stderr
- Descriptive error messages
