use clap::{Parser, Subcommand};
use polypus::config::{DCService, PolypusConfig, is_file};
use polypus::{Result, ServiceStatus};

#[derive(Parser)]
#[command(name = "nprobe")]
#[command(about = "A AI probe tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Register {
        /// Name of the service to register
        name: String,
    },
    Status {},
    Ls {},
    Docker_debug {},
    Config {},
}
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Register { name }) => {
            let mut pwd = std::env::current_dir()?;
            pwd = pwd.join("docker-compose.yaml");

            println!("Registering {}...", pwd.display());
            let pwd_str = pwd.to_str().ok_or("Invalid path")?;

            if !is_file(pwd_str) {
                println!("No docker-compose.yaml found in current directory");
                return Ok(());
            } else {
                let service = DCService::new_from_dc(name, pwd_str.to_string())?;
                let mut conf = PolypusConfig::get_default()?;
                conf.register(service)?;
                return Ok(());
            }
        }
        Some(Commands::Docker_debug {}) => {
            println!("Debugging docker...");
            println!("{:?}", polypus::docker::ps().await?);
        }
        Some(Commands::Status {}) => {
            let conf = PolypusConfig::get_default()?;
            let mut status_list = Vec::new();
            for service in &conf.registered {
                status_list.push(ServiceStatus::new_and_update(service).await?);
            }
            for status in status_list {
                let s = status.pretty_print();
                let cs = status
                    .pretty_print_containers()
                    .iter()
                    .map(|s| format!("  - {}", s))
                    .collect::<Vec<String>>()
                    .join("\n");

                println!(" SERVICE:\n {} \nCONTAINERS:\n {}", s, cs);
            }
        }

        Some(Commands::Config {}) => {
            let conf = PolypusConfig::get_default()?;

            println!("Config: {:?}", conf);
        }
        Some(Commands::Ls {}) => {
            println!("Listing registered services...");
            let conf = PolypusConfig::get_default()?;

            for serv in conf.registered {
                println!("Service: {}, Kind: {}", serv.name, serv.kind);
            }
        }
        None => {
            println!("No command provided");
        }
    }
    Ok(())
}
