// SPDX-FileCopyrightText: 2026 Scott Gibb
// SPDX-License-Identifier: MIT
//
// This is a Rust port of testDHT11.py using async GPIO methods

use anyhow::{Context, Result};
use rppal::gpio::{Gpio, Level};
use std::time::{Duration, Instant};
use tokio::time::sleep;

const DHT_PIN: u8 = 18;
const DHT_TIMEOUT_MICROS: u64 = 100;

/// Represents DHT sensor data
#[derive(Debug)]
struct DhtReading {
    temperature: f32,
    humidity: f32,
}

/// Reads data from DHT11 sensor using async GPIO
async fn read_dht11(pin_number: u8) -> Result<DhtReading> {
    let gpio = Gpio::new().context("Failed to initialize GPIO")?;
    
    // Send start signal - set pin as output
    {
        let mut output_pin = gpio
            .get(pin_number)
            .context("Failed to get GPIO pin")?
            .into_output();
        
        output_pin.set_low();
        sleep(Duration::from_millis(18)).await;
        output_pin.set_high();
        tokio::time::sleep(Duration::from_micros(30)).await;
    } // output_pin is dropped here, releasing the pin

    // Switch to input mode to read data
    let input_pin = gpio
        .get(pin_number)
        .context("Failed to get GPIO pin")?
        .into_input();

    // Wait for sensor response
    let mut timeout_counter = 0;
    while input_pin.read() == Level::High && timeout_counter < DHT_TIMEOUT_MICROS {
        timeout_counter += 1;
        tokio::time::sleep(Duration::from_micros(1)).await;
    }

    if timeout_counter >= DHT_TIMEOUT_MICROS {
        anyhow::bail!("Timeout waiting for sensor response");
    }

    timeout_counter = 0;
    while input_pin.read() == Level::Low && timeout_counter < DHT_TIMEOUT_MICROS {
        timeout_counter += 1;
        tokio::time::sleep(Duration::from_micros(1)).await;
    }

    timeout_counter = 0;
    while input_pin.read() == Level::High && timeout_counter < DHT_TIMEOUT_MICROS {
        timeout_counter += 1;
        tokio::time::sleep(Duration::from_micros(1)).await;
    }

    // Read 40 bits of data
    let mut data: [u8; 5] = [0; 5];
    for byte in data.iter_mut() {
        for bit_idx in 0..8 {
            // Wait for the start of the bit
            timeout_counter = 0;
            while input_pin.read() == Level::Low && timeout_counter < DHT_TIMEOUT_MICROS {
                timeout_counter += 1;
                tokio::time::sleep(Duration::from_micros(1)).await;
            }

            // Measure the high pulse duration
            let start = Instant::now();
            timeout_counter = 0;
            while input_pin.read() == Level::High && timeout_counter < DHT_TIMEOUT_MICROS {
                timeout_counter += 1;
                tokio::time::sleep(Duration::from_micros(1)).await;
            }
            let duration = start.elapsed();

            // If high pulse is longer than 30 microseconds, it's a '1'
            if duration.as_micros() > 30 {
                *byte |= 1 << (7 - bit_idx);
            }
        }
    }

    // Verify checksum
    let checksum = data[0]
        .wrapping_add(data[1])
        .wrapping_add(data[2])
        .wrapping_add(data[3]);
    if checksum != data[4] {
        anyhow::bail!(
            "Checksum mismatch: expected {}, got {}",
            data[4],
            checksum
        );
    }

    // Parse temperature and humidity (DHT11 format)
    let humidity = data[0] as f32 + (data[1] as f32 * 0.1);
    let temperature = data[2] as f32 + (data[3] as f32 * 0.1);

    Ok(DhtReading {
        temperature,
        humidity,
    })
}

/// Main function that continuously reads and displays DHT11 sensor data
#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting DHT11 sensor test on GPIO pin {}...", DHT_PIN);
    println!("Press Ctrl+C to exit\n");

    loop {
        match read_dht11(DHT_PIN).await {
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
