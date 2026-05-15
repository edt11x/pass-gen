# Project Memory - pass-gen

## Project Overview
A GUI password generator for Linux using Rust and Slint.

## Key Decisions
- **Framework:** Slint chosen for its modern, declarative UI approach and native performance on Linux.
- **Randomness:** `rand` crate used for secure random string generation.
- **Build System:** `slint-build` used in `build.rs` to compile `.slint` files to Rust code at build time.
- **Clipboard:** `arboard` crate used for cross-platform clipboard access.

## Known Issues
- Basic password generation logic (now configurable length).
- No "Clear" button yet.

## Next Steps
- Add "Clear" button.
- Improve UI styling and layout (e.g., custom colors/fonts).
- Implement password strength indicator.
