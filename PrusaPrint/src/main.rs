use anyhow::{Context, Result};
use dht_sensor::read_dht_sensor;
use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

const TMP_RESULTS_PATH: &str = "/tmp/oprint/enclosure_plugin/";
const MAX_RETRIES: u32 = 10;

/// Main function that mimics getDHTTemp.py behavior
/// 
/// Exit codes:
/// - 0: Success - sensor read successfully
/// - 2: Invalid arguments or sensor type
/// - 3: All read attempts failed, but last cached result was used
/// - 4: Complete failure - all reads failed and no cache available
#[tokio::main]
async fn main() -> Result<()> {
    // Create results directory
    fs::create_dir_all(TMP_RESULTS_PATH).context("Failed to create results directory")?;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <sensor_type> <pin_number>", args[0]);
        eprintln!("  sensor_type: 11 (DHT11), 22 (DHT22), or 2302 (DHT22)");
        eprintln!("  pin_number: GPIO pin number (e.g., 18 for GPIO 18)");
        // Exit code 2: Invalid arguments
        std::process::exit(2);
    }

    let sensor_type = &args[1];
    let pin_number: u8 = args[2]
        .parse()
        .context("Invalid pin number")?;

    // Validate sensor type
    if !["11", "22", "2302"].contains(&sensor_type.as_str()) {
        eprintln!("Invalid sensor type. Must be 11, 22, or 2302");
        // Exit code 2: Invalid arguments
        std::process::exit(2);
    }

    // Try to read sensor with retries
    let mut retry_count = 0;

    while retry_count <= MAX_RETRIES {
        match read_dht_sensor(pin_number, sensor_type).await {
            Ok(reading) => {
                let results_str = format!("{:.1} | {:.1}", reading.temperature, reading.humidity);
                println!("{}", results_str);

                // Save results to file
                let results_path = Path::new(TMP_RESULTS_PATH).join("dht_results.txt");
                fs::write(&results_path, &results_str)
                    .context("Failed to write results file")?;

                // Exit code 0: Success
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Read attempt {} failed: {}", retry_count + 1, e);
                retry_count += 1;
                if retry_count <= MAX_RETRIES {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    // If all retries failed, try to read last saved result
    let results_path = Path::new(TMP_RESULTS_PATH).join("dht_results.txt");
    if results_path.exists() {
        let results_str = fs::read_to_string(&results_path)
            .context("Failed to read last saved results")?;
        println!("{}", results_str);
        // Exit code 3: All retries failed but using cached result
        std::process::exit(3);
    } else {
        eprintln!("All read attempts failed and no previous results available");
        // Exit code 4: Complete failure with no fallback
        std::process::exit(4);
    }
}
