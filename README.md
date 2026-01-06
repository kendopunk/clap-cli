# Clap CLI

A simple command-line interface (CLI) application built with Rust and `clap`. Manifested in the form of a simple task list management system.

## Features

- **Subcommands:** Organize your CLI with distinct actions.
- **Arguments & Options:** Define and parse command-line inputs.
- **Help Messages:** Automatically generated and customizable help.

## Usage

### Informational

```bash
# Get general help
cargo run -- --help

# Get help for a specific subcommand (e.g., 'add')
cargo run -- help add

# get version
cargo run -- --version

# run test suite
cargo test
```

---

### Task List Commands

```bash
# add a task to the task list (tasks.json)
cargo run -- add "Test Task"

# list out all tasks
cargo run -- list

# list out only completed tasks
cargo run -- list-completed

# mark a task as completed (by its ID)
cargo run -- complete 1

# remove a task (by its ID)
cargo run -- remove 1
```

## Installation

To install `clap-cli`, you'll need Rust and Cargo installed. If you don't have them, you can get them from [rustup.rs](https://rustup.rs/).

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/kendopunk/clap-cli.git
    cd clap-cli
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Run the application:**
    You can run the application directly from the target directory:

    ```bash
    ./target/release/clap-cli --help
    ```

    Or, to make it globally accessible (ensure `~/.cargo/bin` is in your `PATH`):

    ```bash
    cargo install --path .
    clap-cli --help
    ```

4.  **Uninstall**
    ```bash
    cargo uninstall clap-cli
    ```
