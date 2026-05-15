# Project Memory - pass-gen

## Project Overview
A GUI password generator for Linux using Rust and Slint.

## Key Decisions
- **Framework:** Slint chosen for its modern, declarative UI approach and native performance on Linux.
- **Randomness:** `rand` crate used for secure random string generation.
- **Build System:** `slint-build` used in `build.rs` to compile `.slint` files to Rust code at build time.

## Known Issues
- Basic password generation logic (fixed 16 length for now).
- No "Copy to Clipboard" functionality yet.

## Next Steps
- Implement configurable password length.
- Add "Copy to Clipboard" button.
- Improve UI styling and layout.
