mod commands;
use clap::{Parser, Subcommand, ValueEnum};
use geocalc::{BHOrientationLine, Plane};

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

    OrientOne {
        #[arg(long)]
        bearing: f64,

        #[arg(long)]
        inclination: f64,

        #[arg(long)]
        alpha: f64,

        #[arg(long)]
        beta: f64,

        #[arg(long)]
        bottom: bool,
    },
}

#[derive(ValueEnum, Clone)]
enum Structure {
    Plane,
    Lineation,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Borehole(borehole)) => {
            commands::borehole(borehole);
        }
        Some(Commands::OrientOne {
            bearing,
            inclination,
            alpha,
            beta,
            bottom,
        }) => {
            let plane = Plane::alpha_beta(
                bearing,
                inclination * -1.0,
                alpha,
                beta,
                if bottom {
                    BHOrientationLine::Bottom
                } else {
                    BHOrientationLine::Top
                },
            );

            println!("{plane:#?}");
        }
        None => {
            println!("No command specified");
        }
    }
}
