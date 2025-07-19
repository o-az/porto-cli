# Porto CLI

A command-line interface for Porto - Next-Generation Account Stack for Ethereum.

## Usage

### Installation

```sh
cargo install --locked porto
```

```sh
cargo install
```

### Create a Porto Account

```bash
# Basic account creation
cargo run --package porto onboard

# Create account with admin key for server access
cargo run --package porto onboard --admin-key

# Use custom dialog hostname
cargo run --package porto onboard --dialog custom.porto.sh
```

### Commands

- `onboard` (alias: `o`) - Create a Porto Account
  - `--admin-key`, `-a` - Create and provision an additional admin key for server access
  - `--dialog`, `-d` - Dialog hostname (default: stg.id.porto.sh)

## Development

```bash
# Build the entire workspace
cargo build

# Build only the CLI crate
cargo build --package porto

# Run tests for all workspace members
cargo test --workspace

# Run tests for the CLI crate only
cargo test --package porto

# Run with debug output
RUST_LOG=debug cargo run --package porto -- onboard
```

### Project Structure

```
porto-cli/
├── Cargo.toml           # Workspace configuration
├── README.md
├── .cargo/
│   └── config.toml      # Cargo configuration
└── crates/
    └── cli/             # Main CLI crate
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
        │       ├── relay.rs     # Relay server functionality
        │       └── spinner.rs   # UI components
        └── tests/               # Integration tests
```

### Workspace Configuration

This project uses Cargo workspaces to organize the codebase. The workspace is configured in the root `Cargo.toml` with the following structure:

- **Workspace Members**: Currently includes `crates/cli` as the main CLI crate
- **Shared Dependencies**: Common dependencies and configurations are defined at the workspace level
- **Unified Build**: All crates in the workspace can be built and tested together