# battery-health-checker

Cross-platform Rust CLI that reports battery health (capacity retention vs.
design capacity), charge state, cycle count, temperature, and time
remaining.

## Build

```bash
cargo build --release
```

## Run

```bash
# One-shot, formatted table
cargo run

# JSON output (for scripting / piping into other tools)
cargo run -- --json

# Continuously refresh every 5s (default)
cargo run -- --watch

# Continuously refresh every 10s
cargo run -- --watch --interval 10
```

## How health is computed

```
health_percent = (energy_full / energy_full_design) * 100
```

- `energy_full_design`: the battery's rated capacity when new.
- `energy_full`: its current maximum charge capacity (degrades over time
  as the battery ages).

Buckets:
| Health %   | Rating    |
|------------|-----------|
| > 90%      | Excellent |
| 80% – 90%  | Good      |
| 60% – 80%  | Fair      |
| < 60%      | Poor      |

If the OS doesn't report a design capacity, health shows as `N/A` rather
than a misleading number.

## Platform notes

- **Linux**: reads `/sys/class/power_supply/BAT*/` — works out of the box,
  no special permissions needed.
- **Windows**: uses the Windows Power API.
- **macOS**: uses IOKit.
- Field availability (cycle count, temperature) can vary by hardware/driver
  even on the same OS — the code treats these as `Option<T>` and prints
  `N/A` when absent rather than guessing.
- **No battery found** (e.g. on a desktop): the tool prints a message and
  exits cleanly rather than erroring.

## Tests

```bash
cargo test
```

Covers the pure health-calculation logic in `src/health.rs`
(`compute_health_percent`, bucket boundaries, zero-design-capacity edge
case).

## Project layout

```
src/
  main.rs          CLI entry point (clap), --json / --watch / --interval
  battery_info.rs  Wraps the `battery` crate, builds BatteryReport structs
  health.rs        Pure functions: health % and rating bucket (unit tested)
  display.rs       Table + JSON rendering
```
