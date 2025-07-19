# Porto CLI

A command-line interface for Porto - Next-Generation Account Stack for Ethereum.

## Installation

```bash
cargo install --path .
```

## Usage

### Create a Porto Account

```bash
# Basic account creation
porto onboard

# Create account with admin key for server access
porto onboard --admin-key

# Use custom dialog hostname
porto onboard --dialog custom.porto.sh
```

### Commands

- `onboard` (alias: `o`) - Create a Porto Account
  - `--admin-key`, `-a` - Create and provision an additional admin key for server access
  - `--dialog`, `-d` - Dialog hostname (default: stg.id.porto.sh)

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- onboard
```

## Architecture

The CLI is structured using modern Rust best practices:

- **clap v4** - Command-line argument parsing with derive macros
- **tokio** - Async runtime for handling concurrent operations
- **anyhow/thiserror** - Error handling with context
- **serde** - Serialization/deserialization of data structures
- **indicatif** - Progress indicators and spinners
- **dialoguer** - Interactive prompts

### Project Structure

```
porto-cli/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point and CLI definition
│   ├── error.rs         # Error types and handling
│   ├── commands/        # Command implementations
│   │   ├── mod.rs
│   │   └── onboard.rs   # Onboard command
│   └── utils/           # Utility modules
│       ├── mod.rs
│       ├── crypto.rs    # Cryptographic operations
│       ├── dialog.rs    # Dialog communication
│       └── spinner.rs   # UI components
└── tests/               # Integration tests
```