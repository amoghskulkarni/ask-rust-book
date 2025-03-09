# Installation

Follow this [official documentation](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

# Hello, World

Created a `hello-world.rs` file and wrote the first program.

- `main` is the entry point
- `println` is a macro, hence `!`. Macros are like functions, but different (same rules as that of functions don't apply to them)
- `rustc` command to compile. Creates `.pdb` (these files contain debugging information produced by most compilers that target Windows, including information about symbols, types, modules, and so on - [source](https://docs.rs/pdb/latest/pdb/#:~:text=The%20pdb%20create%20parses)) and `.exe` (single executable, with the help of linker) files for the code.
- `rustc .\hello-world.rs`
- `.\hello-world.exe` to run the program

# Hello, Cargo

Cargo is a package manager for Rust, similar to what npm is to JavaScript. Helps manage Rust projects (build, install deps, execute etc.).

- `cargo new <name if the crate>` to create a crate (term for a Rust package)
- `cargo new hello-cargo-default` creates [this crate](./hello-cargo-default/), which doesn't include any git related stuff since it is created inside a Git repository.
- `cargo new --vcs=git hello-cargo-git` overrides this behavior and creates a crate which includes Git related things (such as `.gitignore` file), since it is explicitly stated.
- Toplevel `Cargo.toml` file contains information about the crate and its deps. (similar to `package.json` in Node packages)
- `cargo build` builds (compiles and [does some more](https://doc.rust-lang.org/stable/cargo/)) the crate in debug mode
  - Creates an exe in `/target/debug`
  - `cargo run` looks in this dir
- `cargo run` runs/executes the crate
- `cargo check` checks the code for potential compile errors but doesn't produce the exe.
  - Why? It is often faster than a full-fledged compilation.
  - Can be used continuously while developing the code
- `cargo build --release` for compiling in release mode
  - Creates an exe in `/target/release`
  - Compile time is more, because optimizes exe
  - `cargo run --release` runs the release build
