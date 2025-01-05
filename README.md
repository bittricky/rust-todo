# Rust Todo CLI

A simple command-line todo application written in Rust.

## Prerequisites

1. Install Rust and Cargo:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Follow the on-screen instructions to complete the installation.

2. Verify the installation:
   ```bash
   rustc --version
   cargo --version
   ```

## Building the Project

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd rust-todo
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Usage

The todo application supports the following commands:

1. Add a new todo:
   ```bash
   cargo run -- add "Your todo item"
   ```

2. List all todos:
   ```bash
   cargo run -- list
   ```

3. Mark a todo as done:
   ```bash
   cargo run -- done <id>
   ```

4. Remove a todo:
   ```bash
   cargo run -- remove <id>
   ```

## Project Structure

- `src/main.rs`: Core application logic
- `Cargo.toml`: Project dependencies and metadata
