# Project Memory - pass-gen

## Project Overview
A GUI password generator for Linux using Rust and Slint.

## Key Decisions
- **Framework:** Slint chosen for its modern, declarative UI approach and native performance on Linux.
- **Randomness:** `rand` crate used for secure random string generation.
- **Build System:** `slint-build` used in `build.rs` to compile `.slint` files to Rust code at build time.
- **Clipboard:** `arboard` crate used for cross-platform clipboard access.
- **Strength Indicator:** Custom heuristic logic in Rust based on length and character variety (upper, lower, digits, symbols).

## Known Issues
- Strength indicator is a basic heuristic.

## Next Steps
- Improve UI styling and layout (e.g., custom colors/fonts).
- Implement more robust strength analysis using a library like `zxcvbn`.
- Add tooltips or help text.
