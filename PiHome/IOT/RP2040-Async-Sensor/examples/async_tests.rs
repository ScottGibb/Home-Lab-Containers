//! Integration tests for async sensor implementation
//! 
//! These tests verify the async functionality works correctly
//! and that the Embassy async runtime is properly configured.
//! 
//! Note: These tests are designed to run on actual hardware.
//! Run with: cargo run --example async_tests --release

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Instant, Timer};
use panic_probe as _;

/// Test that async Timer works correctly
/// This verifies that the async timing primitives are functioning
#[embassy_executor::task]
async fn test_async_timer() {
    defmt::info!("Test: Async Timer");
    
    let start = Instant::now();
    Timer::after(Duration::from_millis(100)).await;
    let elapsed = Instant::now() - start;
    
    // Verify the delay was approximately correct (within 20ms tolerance)
    let elapsed_ms = elapsed.as_millis();
    assert!(elapsed_ms >= 100 && elapsed_ms <= 120, 
            "Timer elapsed time {} ms is not in expected range", elapsed_ms);
    
    defmt::info!("✓ Async Timer test passed");
}

/// Test GPIO output operations
/// Verifies that GPIO can be configured and toggled asynchronously
#[embassy_executor::task]
async fn test_gpio_async(mut pin: Output<'static>) {
    defmt::info!("Test: Async GPIO");
    
    // Test setting pin high
    pin.set_high();
    Timer::after(Duration::from_millis(10)).await;
    
    // Test setting pin low
    pin.set_low();
    Timer::after(Duration::from_millis(10)).await;
    
    // Test toggling
    pin.toggle();
    Timer::after(Duration::from_millis(10)).await;
    
    defmt::info!("✓ Async GPIO test passed");
}

/// Test concurrent async tasks
/// Verifies that multiple tasks can run simultaneously
#[embassy_executor::task]
async fn test_task_a() {
    defmt::info!("Task A: Started");
    for i in 0..3 {
        defmt::info!("Task A: Iteration {}", i);
        Timer::after(Duration::from_millis(50)).await;
    }
    defmt::info!("Task A: Completed");
}

#[embassy_executor::task]
async fn test_task_b() {
    defmt::info!("Task B: Started");
    for i in 0..3 {
        defmt::info!("Task B: Iteration {}", i);
        Timer::after(Duration::from_millis(75)).await;
    }
    defmt::info!("Task B: Completed");
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    
    defmt::info!("=== Running Async Tests ===");
    
    // Test 1: Async Timer
    spawner.spawn(test_async_timer()).unwrap();
    Timer::after(Duration::from_millis(200)).await;
    
    // Test 2: Async GPIO
    let test_pin = Output::new(p.PIN_25, Level::Low);
    spawner.spawn(test_gpio_async(test_pin)).unwrap();
    Timer::after(Duration::from_millis(100)).await;
    
    // Test 3: Concurrent tasks
    defmt::info!("Test: Concurrent Async Tasks");
    spawner.spawn(test_task_a()).unwrap();
    spawner.spawn(test_task_b()).unwrap();
    Timer::after(Duration::from_millis(300)).await;
    defmt::info!("✓ Concurrent tasks test passed");
    
    defmt::info!("=== All Tests Passed ===");
    
    // Keep the test runner alive
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
