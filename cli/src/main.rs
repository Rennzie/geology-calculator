mod commands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Borehole(commands::Borehole),
    OrientOne(commands::OrientOne),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Borehole(borehole)) => {
            commands::borehole(borehole);
        }
        Some(Commands::OrientOne(orient_one)) => {
            commands::orient_one(orient_one);
        }
        None => {
            println!("No command specified");
        }
    }
}
