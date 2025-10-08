use encryption_demo::{rsa, sha256};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "encryption-demo")]
#[command(about = "Demonstration of various encryption algorithms")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    algorithm: Algorithm,
}

#[derive(Subcommand)]
enum Algorithm {
    Rsa {
        #[arg(short, long)]
        message: String,

        #[arg(short = 'p')]
        p: u64,
        #[arg(short = 'q')]
        q: u64,
    },

    Sha256 {
        #[arg(short, long)]
        message: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.algorithm {
        Algorithm::Rsa { message, p, q } => {
            // Auto-detect: try parsing as number, fall back to text
            if let Ok(num) = message.parse() {
                rsa::demo::run_rsa_demo(num, p, q);
            } else {
                rsa::demo::run_rsa_demo_text(&message, p, q);
            }
        }
        Algorithm::Sha256 { message } => {
            if message.chars().all(|c| c.is_ascii_hexdigit()) && message.len() % 2 == 0 && message.len() > 0 {
                match hex::decode(&message) {
                    Ok(bytes) => sha256::demo::run_sha256_demo(bytes),
                    Err(_) => sha256::demo::run_sha256_demo_text(&message),
                }
            } else {
                sha256::demo::run_sha256_demo_text(&message);
            }
        }
        // Future algorithms handled here:
        // Algorithm::Ecc { message, curve } => ecc::demo::run_ecc_demo(&message, &curve),
    }
}