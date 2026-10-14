mod battery_info;
mod display;
mod health;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use battery_info::collect_battery_reports;
use display::{print_json, print_table};

/// A cross-platform battery health checker.
#[derive(Parser, Debug)]
#[command(name = "battery-health-checker", version, about)]
struct Cli {
    /// Output machine-readable JSON instead of a formatted table.
    #[arg(long)]
    json: bool,

    /// Continuously refresh the report instead of printing once.
    #[arg(long)]
    watch: bool,

    /// Refresh interval in seconds, used together with --watch.
    #[arg(long, default_value_t = 5)]
    interval: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.watch {
        run_watch_loop(cli.interval, cli.json)?;
    } else {
        run_once(cli.json)?;
    }

    Ok(())
}

fn run_once(json: bool) -> Result<()> {
    let reports = collect_battery_reports()?;
    if json {
        print_json(&reports)?;
    } else {
        print_table(&reports);
    }
    Ok(())
}

fn run_watch_loop(interval_secs: u64, json: bool) -> Result<()> {
    loop {
        // Clear screen and move cursor to top-left before each refresh.
        print!("\x1B[2J\x1B[1;1H");

        let reports = collect_battery_reports()?;
        if json {
            print_json(&reports)?;
        } else {
            print_table(&reports);
        }

        println!("(refreshing every {}s — press Ctrl+C to stop)", interval_secs);
        thread::sleep(Duration::from_secs(interval_secs));
    }
}
