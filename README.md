# rust-fundementals
Exercises to learn rust

## Structure

This repo is a [Cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) where each numbered folder covers one fundamental topic. Read the source file, run the binary, and run the tests.

| Module | Topic |
|--------|-------|
| `01_hello_world` | `println!`, `eprintln!`, `format!`, debug formatting |
| `02_variables` | `let`, `let mut`, `const`, shadowing, type annotations |
| `03_data_types` | integers, floats, booleans, chars, tuples, arrays, strings |
| `04_control_flow` | `if`/`else`, `loop`, `while`, `for`, `match`, loop labels |
| `05_functions` | functions, return values, closures, higher-order functions |
| `06_ownership` | ownership, move semantics, borrowing, references, slices |
| `07_structs` | structs, `impl` blocks, methods, associated functions, tuple structs |
| `08_enums` | enums with data, `Option`, `Result`, `?` operator, recursive types |

## Getting started

```bash
# Build everything
cargo build --workspace

# Run all tests
cargo test --workspace

# Run a specific module
cargo run -p hello_world
cargo run -p variables
# ...and so on

# Run tests for a single module
cargo test -p ownership
```

## Prerequisites

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
