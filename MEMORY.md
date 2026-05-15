# Project Memory - pass-gen

## Project Overview
A GUI password generator for Linux using Rust and Slint.

## Key Decisions
- **Framework:** Slint chosen for its modern, declarative UI approach and native performance on Linux.
- **Randomness:** `rand` crate used for secure random string generation.
- **Build System:** `slint-build` used in `build.rs` to compile `.slint` files to Rust code at build time.
- **Clipboard:** `arboard` crate used for cross-platform clipboard access.
- **Strength Indicator:** Custom heuristic logic in Rust based on length and character variety.
- **Password History:** Implemented using a Slint `VecModel` in Rust, allowing for real-time UI updates and session-persistent (memory only) storage.
- **UI Density:** Used Slint's `if` blocks for conditional rendering to eliminate layout gaps when options are hidden.
- **Theming:** Implemented a custom theme system using a `ThemeColors` struct in Slint, supporting Obsidian (default), Light, Dark, and System modes.
- **Deployment:** Provided an `install.sh` script to automate installation into `~/.local/bin` and create a `.desktop` file for menu integration.

## Known Issues
- Strength indicator is a basic heuristic.
- History is lost when the application is closed (intentional per user privacy requirements).

## Next Steps
- Implement more robust strength analysis using a library like `zxcvbn`.
- Add tooltips or help text for different password formats.
- Consider adding export to CSV/JSON functionality (while respecting privacy settings).
