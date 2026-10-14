use anyhow::{Context, Result};
use battery::units::energy::watt_hour;
use battery::units::ratio::percent;
use battery::units::thermodynamic_temperature::degree_celsius;
use battery::{Manager, State};
use serde::Serialize;

use crate::health::{compute_health_percent, health_rating, HealthRating};

/// A fully-resolved snapshot of one battery's metrics at a point in time.
#[derive(Debug, Clone, Serialize)]
pub struct BatteryReport {
    pub index: usize,
    pub vendor: String,
    pub model: String,
    pub state: String,
    pub charge_percent: f32,
    pub energy_full_wh: f32,
    pub energy_full_design_wh: f32,
    pub health_percent: Option<f32>,
    pub health_rating: String,
    pub cycle_count: Option<u32>,
    pub temperature_celsius: Option<f32>,
    pub time_to_full_mins: Option<f32>,
    pub time_to_empty_mins: Option<f32>,
}

/// Queries the OS for all batteries and builds a report for each.
/// Returns an empty Vec (not an error) if no batteries are present, e.g. on a desktop.
pub fn collect_battery_reports() -> Result<Vec<BatteryReport>> {
    let manager = Manager::new().context("failed to initialize battery manager")?;
    let mut reports = Vec::new();

    for (index, battery_result) in manager
        .batteries()
        .context("failed to enumerate batteries")?
        .enumerate()
    {
        let bat = battery_result.context("failed to read battery info")?;

        // The battery crate stores values in base SI units internally (joules,
        // kelvin, seconds) via the `uom` crate. Always convert through `.get::<unit>()`
        // rather than reading `.value` directly, which returns the raw SI value.
        let energy_full_wh = bat.energy_full().get::<watt_hour>();
        let energy_full_design_wh = bat.energy_full_design().get::<watt_hour>();

        let health_percent = compute_health_percent(energy_full_wh, energy_full_design_wh);
        let rating: HealthRating = health_rating(health_percent);

        let state_str = match bat.state() {
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Full => "Full",
            State::Empty => "Empty",
            State::Unknown => "Unknown",
            _ => "Unknown",
        }
        .to_string();

        let report = BatteryReport {
            index,
            vendor: bat.vendor().unwrap_or("Unknown").to_string(),
            model: bat.model().unwrap_or("Unknown").to_string(),
            state: state_str,
            charge_percent: bat.state_of_charge().get::<percent>(),
            energy_full_wh,
            energy_full_design_wh,
            health_percent,
            health_rating: rating.as_str().to_string(),
            cycle_count: bat.cycle_count(),
            temperature_celsius: bat.temperature().map(|t| t.get::<degree_celsius>()),
            // Time's base SI unit is the second, so dividing the raw value by 60
            // to get minutes is correct without needing a unit conversion here.
            time_to_full_mins: bat.time_to_full().map(|t| t.value / 60.0),
            time_to_empty_mins: bat.time_to_empty().map(|t| t.value / 60.0),
        };

        reports.push(report);
    }

    Ok(reports)
}
