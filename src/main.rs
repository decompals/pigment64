use anyhow::Result;
use clap::{Parser, Subcommand};

mod cli;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Converts a binary image to a PNG
    ToPng {
        #[clap(flatten)]
        args: cli::png::PngArgs,
    },
    /// Converts a PNG to a binary image
    ToBin {
        #[clap(flatten)]
        args: cli::binary::BinaryArgs,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ToPng { args } => {
            cli::png::handle_png(args)?;
        }
        Commands::ToBin { args } => {
            cli::binary::handle_binary(args)?;
        }
    }

    Ok(())
}
