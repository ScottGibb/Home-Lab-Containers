#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

/// Async DHT11 sensor reading implementation
/// 
/// This implementation uses Embassy's async/await patterns instead of polling.
/// Benefits over traditional polling approach:
/// - Non-blocking: Other tasks can run while waiting for sensor timing
/// - Lower power consumption: CPU can sleep between sensor readings
/// - Better resource utilization: No busy-waiting loops
/// - Cleaner code: async/await makes timing logic more readable
pub struct AsyncDht11<'a> {
    pin: Output<'a>,
}

impl<'a> AsyncDht11<'a> {
    pub fn new(pin: Output<'a>) -> Self {
        Self { pin }
    }

    /// Read temperature and humidity from DHT11 sensor using async patterns
    /// 
    /// Note: This is a demonstration implementation with mock data.
    /// A complete implementation would:
    /// - Switch pin to input mode for reading
    /// - Use async timing to read 40 bits of data
    /// - Verify checksum
    /// - Return actual sensor readings
    /// 
    /// Traditional polling approach would use:
    /// ```rust
    /// loop {
    ///     if check_condition() {
    ///         break;
    ///     }
    ///     // Busy waiting - wastes CPU cycles
    /// }
    /// ```
    /// 
    /// Async approach uses:
    /// ```rust
    /// Timer::after(duration).await;  // Yields to other tasks
    /// ```
    pub async fn read(&mut self) -> Result<(f32, f32), &'static str> {
        // Send start signal - async allows clean timing control
        self.pin.set_low();
        Timer::after(Duration::from_millis(18)).await;
        self.pin.set_high();
        
        // Wait for sensor response - no busy polling needed
        Timer::after(Duration::from_micros(40)).await;
        
        // In a complete implementation, we would:
        // 1. Switch pin to input mode
        // 2. Wait for sensor to pull line low (async)
        // 3. Read 40 bits of data using async timing
        // 4. Verify checksum
        // 5. Parse temperature and humidity values
        
        // For demonstration purposes, return mock data
        // This demonstrates the async pattern without requiring actual hardware
        let temperature = 23.5;
        let humidity = 45.0;
        
        Ok((temperature, humidity))
    }
}

/// Async task for periodic sensor readings
/// 
/// This task demonstrates async advantages:
/// - Runs independently without blocking other tasks
/// - Uses Timer::after() instead of busy-waiting
/// - Error handling with async retry logic
#[embassy_executor::task]
async fn sensor_task(pin: Output<'static>) {
    let mut sensor = AsyncDht11::new(pin);
    
    loop {
        info!("Reading DHT11 sensor...");
        
        match sensor.read().await {
            Ok((temp, humidity)) => {
                info!("Temperature: {}°C, Humidity: {}%", temp, humidity);
            }
            Err(e) => {
                warn!("Sensor read error: {}", e);
                // Async retry with exponential backoff
                Timer::after(Duration::from_secs(2)).await;
                continue;
            }
        }
        
        // Non-blocking delay between readings
        // Other tasks continue to run during this time
        Timer::after(Duration::from_secs(5)).await;
    }
}

/// LED blink task to demonstrate concurrent async tasks
/// 
/// This runs simultaneously with sensor_task, demonstrating
/// how async allows multiple independent operations
#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    info!("RP2040 Async Sensor Example Started");
    info!("Using Embassy async framework instead of polling");
    
    // Configure LED on GPIO 25 (built-in LED on Pico)
    let led = Output::new(p.PIN_25, Level::Low);
    
    // Configure DHT11 sensor on GPIO 18
    let dht_pin = Output::new(p.PIN_18, Level::High);
    
    // Spawn concurrent async tasks
    // Both tasks run independently without blocking each other
    spawner.spawn(blink_task(led)).unwrap();
    spawner.spawn(sensor_task(dht_pin)).unwrap();
    
    info!("All async tasks spawned successfully");
}
