use clap::{Parser, Subcommand};
use polypus::config::{DCService, PolypusConfig, is_file};
use polypus::{Result, ServiceStatus, ui};

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
            let pwd = std::env::current_dir()?;
            let exts = ["yaml", "yml"];

            let mut found_path = None;
            for ext in exts {
                let path = pwd.join(format!("docker-compose.{}", ext));
                if let Some(path_str) = path.to_str() {
                    if is_file(path_str) {
                        found_path = Some(path_str.to_string());
                        break;
                    }
                }
            }

            let pwd_str = match found_path {
                Some(p) => p,
                None => {
                    ui::error(
                        "No docker-compose.yaml or docker-compose.yml found in current directory",
                    );
                    return Ok(());
                }
            };

            let sp = ui::spinner("Registering service...");
            let service = DCService::new_from_dc(name.clone(), pwd_str.to_string())?;
            let mut conf = PolypusConfig::get_default()?;
            conf.register(service)?;
            sp.finish_and_clear();

            ui::success(&format!("Service '{}' registered", name));
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
            let conf = PolypusConfig::get_default()?;

            if conf.registered.is_empty() {
                ui::info("No services registered. Use 'polypus register <name>'");
                return Ok(());
            }

            ui::render_service_list(&conf.registered);
        }
        None => {
            println!("No command provided");
        }
    }
    Ok(())
}
