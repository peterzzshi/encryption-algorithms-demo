mod rsa;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "encryption-demo")]
#[command(about = "Demonstration of encryption algorithms")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Rsa {
        #[arg(short, long)]
        message: u64,
        #[arg(short = 'p')]
        p: u64,
        #[arg(short = 'q')]
        q: u64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Rsa { message, p, q } => {
            rsa::demo::run_rsa_demo(message, p, q);
        }
    }
}