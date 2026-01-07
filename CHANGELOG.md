# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2] - 2026-01-07

### Security
- **Navigation Lock:** Implemented a navigation handler that restricts browsing strictly to YouTube and Google Account domains.
    - Prevents accidental clicks on external links (ads, phishing, etc.).
    - Blocks all navigation to domains other than `youtube.com`, `youtu.be`, and `accounts.google.com`.

## [0.1.1] - 2026-01-07

### Fixed
- Fixed a bug where returning from BGM Mode to Normal Mode caused the player to be obscured or appear black.
- Fixed layout issues when toggling modes by triggering a window resize event.

## [0.1.0] - 2026-01-07

### Added
- Initial release of CAPRA.
- BGM Mode (Distraction-free mode) with 'T' key.
- macOS native menu bar and shortcuts.
- English and Japanese documentation.
