# FindOmni
FindOmni is a powerful and efficient file searching tool built with Rust. It allows users to quickly locate files and directories

![Commit Activity](https://img.shields.io/github/commit-activity/t/YourIdentityDev/FindOmni)

## Features

- **Smart Search:** Prioritize common directories like `C:\Users\<user>\Documents` for faster results.
- **Full Search:** Perform a deep scan of the entire file system for comprehensive results.
- **No GUI Mode:** Run entirely from the command line for minimal resource usage. (You can use this by using "--nogui" as an argument)
- **Advanced Filtering:** Filter search results by file size, date, and patterns.
- **Settings Management:** Customize search settings directly from the CLI.

## Installation

To install FindOmni, you need to have Rust and Cargo installed on your system. Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/YourIdentityDev/findomni.git
cd findomni
cargo build --release
```

## Usage

#### Options and Sub-options

- `-h, --help`: Show help message and exit.
- `--nogui`: Run the search in CLI mode without a graphical user interface.
  - `-so, -smart-only`: Enable only smart search (skips full search after a timeout).
  - `-fo, -full-only`: Enable only full search (disables smart search).
  - `-p, -path <DIRECTORY>`: Restrict the search to a specific directory path.
  - `-t, -timeout <SECONDS>`: Stop the search after the specified time (in seconds).
  - `-e, -exclude <PATTERN>`: Exclude files or directories matching the given pattern from the search results.
  - `-l, -limit <NUMBER>`: Limit the number of search results.
  - `-o, -output <FILE>`: Save search results to a specified file.
  - `-v, -verbose`: Show detailed information about the search process.
  - `-s, -size <FILE-SIZE>`: Filter by file size.
  - `-d, -date <YYYY-MM-DD>`: Filter by last modified date.

## Example

```sh
findomni --nogui -fo "file.txt"
```

This command will perform a full-only search for files and directories containing the term "file.txt".

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes. For major changes (like suggesting a new feature), please open an issue first to discuss what you would like to change.
