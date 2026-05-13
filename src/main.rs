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

        Some(Commands::Status {}) => {
            let conf = PolypusConfig::get_default()?;

            if conf.registered.is_empty() {
                ui::info("No services registered. Use 'polypus register <name>'");
                return Ok(());
            }

            let pb = ui::progress_bar(conf.registered.len() as u64);
            let mut status_list = Vec::new();

            for service in &conf.registered {
                pb.set_message(format!("Checking {}", service.name));
                status_list.push(ServiceStatus::new_and_update(service).await?);
                pb.inc(1);
            }
            pb.finish_and_clear();

            for status in &status_list {
                ui::render_status(status);
            }
            println!();
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
