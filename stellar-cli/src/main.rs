use clap::{Parser, Subcommand};

mod scan;

#[derive(Parser)]
#[command(name = "Stellar", about = "Programming language for creating music.")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Scan {
        #[arg(value_name = "FILE")]
        filepath: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { filepath } => {
            scan::run(&filepath);
        }
    }
}
