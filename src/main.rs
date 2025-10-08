use encryption_demo::rsa;
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
        // Future algorithms handled here:
        // Algorithm::Sha256 { message } => sha256::demo::run_sha256_demo(&message),
        // Algorithm::Ecc { message, curve } => ecc::demo::run_ecc_demo(&message, &curve),
    }
}