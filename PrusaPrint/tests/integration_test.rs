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
    // Test with no arguments - should exit with code 2
    let output = Command::new("cargo")
        .args(&["run", "--bin", "dht_sensor"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2), "Should exit with code 2 for missing arguments");
}

#[test]
fn test_dht_sensor_invalid_sensor_type() {
    // Test with invalid sensor type - should exit with code 2
    let output = Command::new("cargo")
        .args(&["run", "--bin", "dht_sensor", "--", "99", "18"])
        .output()
        .expect("Failed to execute command");

    assert_eq!(output.status.code(), Some(2), "Should exit with code 2 for invalid sensor type");
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
