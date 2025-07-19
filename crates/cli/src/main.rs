use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod error;
mod utils;

#[derive(Parser)]
#[command(
    name = "porto",
    version,
    author,
    about = "CLI for Porto - Next-Generation Account Stack for Ethereum",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(visible_alias = "o")]
    Onboard {
        #[arg(short, long, default_value = "false")]
        admin_key: bool,

        #[arg(short, long, default_value = "stg.id.porto.sh")]
        dialog: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Clear the screen
    print!("\x1B[2J\x1B[1;1H");

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Onboard { admin_key, dialog }) => {
            commands::onboard::execute(admin_key, dialog).await?;
        }
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            std::process::exit(0);
        }
    }

    Ok(())
}
