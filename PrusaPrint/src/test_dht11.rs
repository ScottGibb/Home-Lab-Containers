// SPDX-FileCopyrightText: 2026 Scott Gibb
// SPDX-License-Identifier: MIT
//
// This is a Rust port of testDHT11.py using async GPIO methods

use anyhow::Result;
use dht_sensor::read_dht_sensor;
use std::time::Duration;
use tokio::time::sleep;

const DHT_PIN: u8 = 18;
const SENSOR_TYPE: &str = "11";

/// Main function that continuously reads and displays DHT11 sensor data
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting DHT11 sensor test on GPIO pin {}...", DHT_PIN);
    println!("Press Ctrl+C to exit\n");

    loop {
        match read_dht_sensor(DHT_PIN, SENSOR_TYPE).await {
            Ok(reading) => {
                let temperature_f = reading.temperature * (9.0 / 5.0) + 32.0;
                println!(
                    "Temp: {:.1} F / {:.1} C    Humidity: {:.0}% ",
                    temperature_f, reading.temperature, reading.humidity
                );
            }
            Err(error) => {
                // Errors happen fairly often, DHT's are hard to read, just keep going
                eprintln!("{}", error);
                sleep(Duration::from_secs(2)).await;
                continue;
            }
        }

        sleep(Duration::from_secs(2)).await;
    }
}
