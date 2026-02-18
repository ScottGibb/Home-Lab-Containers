use anyhow::{Context, Result};
use rppal::gpio::{Gpio, Level};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const TMP_RESULTS_PATH: &str = "/tmp/oprint/enclosure_plugin/";
const MAX_RETRIES: u32 = 10;
const DHT_TIMEOUT_MICROS: u64 = 100;

/// Represents DHT sensor data
#[derive(Debug)]
struct DhtReading {
    temperature: f32,
    humidity: f32,
}

/// Reads data from DHT11 or DHT22 sensor using async GPIO
async fn read_dht_sensor(pin_number: u8, sensor_type: &str) -> Result<DhtReading> {
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

    // Parse temperature and humidity based on sensor type
    let (temperature, humidity) = if sensor_type == "11" {
        // DHT11: Integer values only
        let humidity = data[0] as f32 + (data[1] as f32 * 0.1);
        let temperature = data[2] as f32 + (data[3] as f32 * 0.1);
        (temperature, humidity)
    } else {
        // DHT22/DHT2302: 16-bit values
        let humidity = ((data[0] as u16) << 8 | data[1] as u16) as f32 / 10.0;
        let mut temperature = (((data[2] & 0x7F) as u16) << 8 | data[3] as u16) as f32 / 10.0;
        
        // Check for negative temperature (MSB of data[2] indicates sign)
        if data[2] & 0x80 != 0 {
            temperature = -temperature;
        }
        
        (temperature, humidity)
    };

    Ok(DhtReading {
        temperature,
        humidity,
    })
}

/// Main function that mimics getDHTTemp.py behavior
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
        std::process::exit(2);
    }

    let sensor_type = &args[1];
    let pin_number: u8 = args[2]
        .parse()
        .context("Invalid pin number")?;

    // Validate sensor type
    if !["11", "22", "2302"].contains(&sensor_type.as_str()) {
        eprintln!("Invalid sensor type. Must be 11, 22, or 2302");
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
        std::process::exit(3);
    } else {
        eprintln!("All read attempts failed and no previous results available");
        std::process::exit(4);
    }
}
