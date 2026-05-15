# Project Plan - pass-gen

## Phase 1: Initialization (Done)
- [x] Cargo project setup.
- [x] Slint integration.
- [x] Basic UI layout (password display, toggles, generate button).
- [x] Basic generation logic.

## Phase 2: Refinement (Done)
- [x] Add password length slider.
- [x] Implement Clipboard support.
- [x] Add "Clear" button.
- [x] Add password strength indicator.

## Phase 3: Advanced Password Types (Done)
- [x] Implement Non-ambiguous character sets (12 and arbitrary length).
- [x] Implement Numeric, Binary, Hex, Base 36, Base 62, and ASCII Printable formats.
- [x] Implement "Easy to Remember" leet-speak word phrases.
- [x] Implement "Work Password" format (3 non-ambig + < + 3 non-ambig repeated).
- [x] Implement Random Integer range generation.

## Phase 4: UX & Persistent State (Done)
- [x] Implement time-ordered password history list.
- [x] Add ability to delete history entries.
- [x] Ensure history is memory-persistent during the session.
- [x] Optimize UI layout for maximum density (conditional rendering with 'if').

## Phase 5: Customization & Deployment (Done)
- [x] Implement Color Schemes (Light, Dark, System, Obsidian).
- [x] Create a desktop entry file for Linux integration.
- [x] Create an installation script (`install.sh`).
- [x] Build and test release binary.
