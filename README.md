# lumon-mdr

A Rust-powered terminal app inspired by Severance's Macrodata Refinement (MDR) interface. It runs as an SSH server, letting users "refine data" in a nostalgic, retro-futuristic TUI—just like Lumon's innies.

## Description

This project recreates the mysterious data refinement experience shown in Apple TV+'s "Severance" series. The interface mimics the retro-futuristic terminal that Lumon employees use to sort numbers based on how they "feel." 

The application features:
- A loading screen with authentic Lumon-style messages
- Data containers for number refinement
- Progress tracking with visual feedback
- Mouse and keyboard support for data manipulation
- Terminal color detection for different display environments

## Technologies

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Ratatui](https://github.com/ratatui-org/ratatui) - TUI (Text User Interface) library 
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [Rand](https://github.com/rust-random/rand) - Random number generation

## Installation

Make sure you have Rust installed. If not, install it through [rustup](https://rustup.rs/).

```bash
# Clone the repository
git clone https://github.com/theabecaster/lumon-mdr.git
cd lumon-mdr

# Build the project
cargo build --release
```

## Usage

Run the application with:

```bash
cargo run
```

Or use the compiled binary:

```bash
./target/release/lumon-mdr
```

### Controls

- `q` - Quit the application
- `1-5` - Add data to specific containers (5 units)
- `Space` - Add random values to a random container
- `r` - Reset all containers
- Mouse - Click on specific areas to interact with data

## Running as SSH Server

(Future functionality) The application will allow remote access through SSH, creating a faithful recreation of the Lumon experience.

## License

All Rights Reserved.
