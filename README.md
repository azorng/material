# Material

A material design color palette for the terminal

<img src="https://i.ibb.co/2MDKmh7/Screenshot-2022-08-02-at-16-43-12.png" alt="drawing" width="400"/>

## Installation

### Homebrew

```bash
brew tap azorng/material
brew install material
```

### Cargo

First, install [Rust](https://www.rust-lang.org/tools/install) (using the recommended `rustup` installation method) and then

```bash
cargo install material --locked --features=cli
```

## Usage

Run the command ``material`` in the terminal.
Type the color code to copy its hex color to the clipboard. Type Esc to exit.

## As a library

This crate can also be used as a library in your own apps.

```bash
cargo add material
```

```rust
use material_colors::colors;

assert_eq!(colors::RED_50.to_string(), "#ffebee");
assert_eq!(colors::RED_100.to_string(), "#ffcdd2");
```

### From Ratatui

Colors provided by the library can be converted to [Ratatui](https://ratatui.rs) colors. Just
enable the `ratatui` feature.

```bash
cargo add material --features=ratatui
```

```rust
use material_colors::colors;
use ratatui::prelude::*;

let line = Line::styled("hello world", Style::new().fg(colors::RED_50.into()));
```
