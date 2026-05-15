# Password Generator (Rust + Slint)

A simple, modern password generation application for Linux, built with Rust and the Slint UI framework.

## Features
- Generate 16-character secure passwords.
- Toggle uppercase letters, numbers, and symbols.
- Clean and responsive Slint-based GUI.

## Prerequisites
- Rust (latest stable)
- Slint dependencies (see [Slint documentation](https://slint.dev/))

## Usage
```bash
cargo run
```

## Development
- `ui/appwindow.slint`: UI definition.
- `src/main.rs`: Application logic and event handling.
- `build.rs`: Compiles Slint UI files.
