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
        /// The message as a number (e.g., 4, 42, 123)
        #[arg(short, long)]
        message: u64,
        /// Prime number p (e.g., 3, 5, 7, 11, 13, 17, 19, 23)
        #[arg(short = 'p')]
        p: u64,
        /// Prime number q (e.g., 3, 5, 7, 11, 13, 17, 19, 23)
        #[arg(short = 'q')]
        q: u64,
    },
    RsaText {
        /// The text message to encrypt (max 8 characters)
        #[arg(short, long)]
        text: String,
        /// Prime number p (must be large enough: try 61, 97, 127, 251, 503, 997)
        #[arg(short = 'p')]
        p: u64,
        /// Prime number q (must be large enough: try 53, 89, 113, 241, 499, 991)
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
        Commands::RsaText { text, p, q } => {
            rsa::demo::run_rsa_demo_text(&text, p, q);
        }
    }
}