use owo_colors::OwoColorize;

use crate::battery_info::BatteryReport;

/// Prints a colored, human-readable table of battery reports to stdout.
pub fn print_table(reports: &[BatteryReport]) {
    if reports.is_empty() {
        println!("No batteries detected on this system.");
        return;
    }

    for report in reports {
        println!("{}", format!("Battery #{}", report.index).bold());
        println!("  Vendor / Model : {} / {}", report.vendor, report.model);
        println!("  State          : {}", report.state);
        println!("  Charge         : {:.1}%", report.charge_percent);

        match report.health_percent {
            Some(p) => {
                let colored_health = colorize_by_rating(&report.health_rating, format!("{:.1}%", p));
                println!("  Health         : {} ({})", colored_health, report.health_rating);
            }
            None => println!("  Health         : N/A (no design capacity reported)"),
        }

        println!(
            "  Capacity       : {:.2} Wh full / {:.2} Wh design",
            report.energy_full_wh, report.energy_full_design_wh
        );

        match report.cycle_count {
            Some(c) => println!("  Cycle count    : {}", c),
            None => println!("  Cycle count    : N/A"),
        }

        match report.temperature_celsius {
            Some(t) => println!("  Temperature    : {:.1}°C", t),
            None => println!("  Temperature    : N/A"),
        }

        if let Some(mins) = report.time_to_full_mins {
            println!("  Time to full   : {:.0} min", mins);
        }
        if let Some(mins) = report.time_to_empty_mins {
            println!("  Time to empty  : {:.0} min", mins);
        }

        println!();
    }
}

fn colorize_by_rating(rating: &str, text: String) -> String {
    match rating {
        "Excellent" => text.green().bold().to_string(),
        "Good" => text.green().to_string(),
        "Fair" => text.yellow().to_string(),
        "Poor" => text.red().bold().to_string(),
        _ => text.dimmed().to_string(),
    }
}

/// Prints all reports as pretty-printed JSON.
pub fn print_json(reports: &[BatteryReport]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(reports)?;
    println!("{}", json);
    Ok(())
}
