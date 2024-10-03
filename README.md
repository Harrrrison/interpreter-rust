# Rust Interpreter

**Rust Interpreter** is a performant and extensible interpreter written in Rust, designed to evaluate expressions efficiently. This project demonstrates key concepts in interpreter design and provides a solid foundation for building more advanced language features.

## Features

- **Fast execution**: Built with Rust, leveraging its powerful memory management and performance optimizations.
- **Extensible architecture**: Easily add new features or extend the interpreter to handle more complex language constructs.
- **Clear code structure**: Modular, well-documented code, making it easy to follow and contribute to.

### Currently working on

- **Implementing Statements and State**:  This will allow a user to create and store variables along with console output

## Getting Started

### Prerequisites

To get started, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70 or higher)
- Cargo (Rust’s package manager)

### Installation

1. Clone this repository:

    ```bash
    git clone https://github.com/Harrrrison/interpreter-rust.git
    cd interpreter-rust
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the interpreter:

    ```bash
    cargo run
    ```

## Usage

The interpreter supports evaluating simple arithmetic expressions. Here's an example of how to use it:

```bash
$ cargo run
> 5 + 3 * (2 - 1)
8
