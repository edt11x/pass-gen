# Password Generator (Rust + Slint)

A simple, modern, and highly configurable password generation application for Linux, built with Rust and the Slint UI framework.

## Features
- **Extensive Password Formats**:
  - **Standard**: Fully customizable with uppercase, numbers, and symbols.
  - **Non-ambiguous**: Excludes easily confused characters (e.g., `0`, `O`, `1`, `I`).
  - **Alpha/Alphanumeric**: Pure letter or letter+digit sets.
  - **Base 36 / Base 58 / Base 62**: For various encoding needs.
  - **Hex/Binary/Numeric**: For technical or PIN generation.
  - **ASCII (Printable)**: Uses all non-whitespace printable characters.
  - **TTY Noise**: Focused on non-alphanumeric symbols and punctuation.
  - **Easy to Remember**: Generates "leet-speak" word phrases (e.g., `4lph4-br4v0`).
  - **Work Password**: A specific 14-character repeated format (e.g., `abc<123abc<123`).
  - **Random Integer**: Generates a random number in a user-defined range.
- **Password History**: Keep track of generated passwords during your session. Entries can be copied or deleted individually. (History is memory-only and never saved to disk).
- **Multiple Color Schemes**: Choose between **Obsidian** (default), **Dark**, **Light**, or **System** themes.
- **Strength Indicator**: Real-time feedback on password complexity.
- **Clipboard Integration**: Easy copying of passwords to your clipboard.
- **Dense UI**: Compact layout optimized for speed and visibility.

## Installation

### Prerequisites
- Rust (latest stable)
- Slint dependencies (see [Slint documentation](https://slint.dev/))

### Install to System
You can install the application to your local user directory (`~/.local/bin`) and add it to your applications menu by running:
```bash
./install.sh
```

### Run without Installation
```bash
./run.sh
# OR
cargo run --release
```

## Development
- `ui/appwindow.slint`: UI definition and theme system.
- `src/main.rs`: Application logic, password generation algorithms, and state management.
- `install.sh`: Linux deployment script.

## License
This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
