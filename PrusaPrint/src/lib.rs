use anyhow::{Context, Result};
use rppal::gpio::{Gpio, Level};
use std::time::{Duration, Instant};

const DHT_TIMEOUT_MICROS: u64 = 100;

/// Represents DHT sensor data
#[derive(Debug)]
pub struct DhtReading {
    pub temperature: f32,
    pub humidity: f32,
}

/// Reads data from DHT11 or DHT22 sensor using async GPIO
/// 
/// Note: This implementation uses tokio::time::sleep for timing, which may have
/// reduced precision compared to blocking implementations. For timing-critical
/// DHT sensor communication, the timeout values have been adjusted to work
/// reliably with async scheduler overhead. If more precise timing is required,
/// consider using blocking GPIO operations instead.
pub async fn read_dht_sensor(pin_number: u8, sensor_type: &str) -> Result<DhtReading> {
    let gpio = Gpio::new().context("Failed to initialize GPIO")?;
    
    // Send start signal - set pin as output
    {
        let mut output_pin = gpio
            .get(pin_number)
            .context("Failed to get GPIO pin")?
            .into_output();
        
        output_pin.set_low();
        tokio::time::sleep(Duration::from_millis(18)).await;
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
        // DHT11: data[0] = integer part, data[1] = fractional part (in tenths)
        // Example: 55.3% = data[0]=55, data[1]=3
        let humidity = data[0] as f32 + (data[1] as f32 / 10.0);
        let temperature = data[2] as f32 + (data[3] as f32 / 10.0);
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
