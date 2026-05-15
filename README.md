# Password Generator (Rust + Slint)

A simple, modern password generation application for Linux, built with Rust and the Slint UI framework.

## Features
- **Multiple Formats**: Choose between Standard (customizable), Base36, Hex, and Numeric passwords.
- Generate secure passwords with configurable length (8-64 characters).
- Toggle uppercase letters, numbers, and symbols.
- **Password Strength Indicator** (Weak, Medium, Strong).
- **Copy to Clipboard** and **Clear** functionality.
- Clean and responsive Slint-based GUI.

## Prerequisites
- Rust (latest stable)
- Slint dependencies (see [Slint documentation](https://slint.dev/))

## Usage
You can run the application directly using Cargo:
```bash
cargo run
```
Or use the provided run script:
```bash
./run.sh
```

## Development
- `ui/appwindow.slint`: UI definition.
- `src/main.rs`: Application logic and event handling.
- `build.rs`: Compiles Slint UI files.
