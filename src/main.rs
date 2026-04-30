use clap::{Parser, Subcommand};
use polypus::Result;
use polypus::config::{DCService, PolypusConfig};
#[derive(Parser)]
#[command(name = "nprobe")]
#[command(about = "A AI probe tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Register {},
    Config {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Register {}) => {
            println!("Registering...");
        }
        Some(Commands::Config {}) => {
            let conf = PolypusConfig::new_from_path("config.json".to_string())?;
            println!("Config: {:?}", conf);
        }
        None => {
            println!("No command provided");
        }
    }
    Ok(())
}
