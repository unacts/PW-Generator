# Rust Password Generator

A fast, lightweight, cross-platform desktop password generator built with Rust and `egui`.

## Features

- **Customizable Length:** Generate passwords from 6 to 64 characters via an interactive slider.
- **Character Set Controls:** Toggle uppercase, lowercase, numbers, and special symbols.
- **Ambiguous Character Filter:** Option to exclude easily confused characters (`I`, `l`, `1`, `O`, `0`).
- **Live Strength Indicator:** Real-time visual strength meter (`WEAK`, `MODERATE`, `STRONG`, `VERY STRONG`).
- **One-Click Copy:** Copy generated passwords straight to the system clipboard.
- **100% Offline:** Runs locally with zero network requests.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021 or newer)

### Run from Source

```bash
git clone [https://github.com/pulldox/PW-Generator.git]
cd PW-Generator
cargo run --release