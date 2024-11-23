use clap::{Parser, Subcommand};

mod scan;
mod parse;

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
    Parse {
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
        Command::Parse { filepath } => parse::run(&filepath),
    }
}
