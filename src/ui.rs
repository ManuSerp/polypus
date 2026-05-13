use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::{ServiceStatus, StatusEnum, config::DCService};

pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

pub fn progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓░"),
    );
    pb
}

pub fn success(msg: &str) {
    println!("{} {}", style("✓").green().bold(), msg);
}

pub fn error(msg: &str) {
    println!("{} {}", style("✗").red(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", style("ℹ").blue(), msg);
}

pub fn render_status(status: &ServiceStatus) {
    let emoji = match status.status {
        StatusEnum::Up => style("✅").green(),
        StatusEnum::Unhealthy => style("🛑").red(),
        StatusEnum::Unknown => style("⚠️").yellow(),
        StatusEnum::Exited => style("❌").red(),
        _ => style("❓").dim(),
    };

    let status_text = match status.status {
        StatusEnum::Up | StatusEnum::Healthy => style(&status.status).green().bold(),
        StatusEnum::Exited | StatusEnum::Unhealthy => style(&status.status).red().bold(),
        StatusEnum::Unknown => style(&status.status).yellow().bold(),
        _ => style(&status.status).dim(),
    };

    println!("\n{}", style("─".repeat(60)).dim());
    println!(
        "{} {} {}",
        emoji,
        style(&status.service.name).cyan().bold(),
        status_text
    );

    if !status.containers_status.is_empty() {
        for container in &status.containers_status {
            let icon = match container.status {
                StatusEnum::Up | StatusEnum::Healthy => style("▶").green(),
                StatusEnum::Exited => style("■").red(),
                _ => style("●").dim(),
            };

            let c_status = match container.status {
                StatusEnum::Up | StatusEnum::Healthy => style(&container.status).green(),
                StatusEnum::Exited => style(&container.status).red(),
                StatusEnum::Unhealthy => style(&container.status).red(),
                StatusEnum::Unknown => style(&container.status).yellow(),
                _ => style(&container.status).dim(),
            };

            println!("  {} {} ({})", icon, style(&container.name).dim(), c_status);
        }
    }
}

pub fn render_service_list(services: &[DCService]) {
    println!("\n{}", style("─".repeat(60)).dim());
    for serv in services {
        println!(
            "  {} {} ({})",
            style("📦").dim(),
            style(&serv.name).cyan().bold(),
            style(&serv.kind).yellow()
        );
    }
    println!("{}", style("─".repeat(60)).dim());
}
