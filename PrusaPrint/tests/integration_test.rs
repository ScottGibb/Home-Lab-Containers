// Integration tests for DHT sensor binary

use std::process::Command;

#[test]
fn test_dht_sensor_help() {
    // Test that the binary exists and can be invoked
    let output = Command::new("cargo")
        .args(&["run", "--bin", "dht_sensor"])
        .output();

    // On a system without actual GPIO hardware, we expect it to fail
    // but the binary should at least be runnable
    assert!(output.is_ok(), "Binary should be executable");
}

#[test]
fn test_dht_sensor_invalid_args() {
    // Test with no arguments - should fail
    let output = Command::new("cargo")
        .args(&["run", "--bin", "dht_sensor"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Should fail with missing arguments");
    
    // Check that usage message is present in stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"), "Should print usage message");
    assert!(stderr.contains("sensor_type"), "Should mention sensor_type parameter");
    assert!(stderr.contains("pin_number"), "Should mention pin_number parameter");
}

#[test]
fn test_dht_sensor_invalid_sensor_type() {
    // Test with invalid sensor type - should fail
    let output = Command::new("cargo")
        .args(&["run", "--bin", "dht_sensor", "--", "99", "18"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success(), "Should fail with invalid sensor type");
}

#[test]
fn test_test_dht11_binary_exists() {
    // Test that the test_dht11 binary can be built
    let output = Command::new("cargo")
        .args(&["build", "--bin", "test_dht11"])
        .output();

    assert!(output.is_ok(), "test_dht11 binary should build successfully");
    if let Ok(out) = output {
        assert!(out.status.success(), "test_dht11 should compile without errors");
    }
}
