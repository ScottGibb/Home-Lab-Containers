# IOT Docker Stack

## Summary

The IOT docker stack is used for the smart devices around the house. This involves creating various self built containers alongside Node-Red and Home Assistant, this can all be found in the docker-compose file.

## Projects

### RP2040-Async-Sensor

Modern async-based embedded Rust implementation for Raspberry Pi Pico (RP2040) using the Embassy framework. Demonstrates efficient async/await patterns for sensor reading instead of traditional polling approaches.

**Features:**
- Non-blocking async sensor reading
- Concurrent task execution
- Low power consumption
- Type-safe Rust implementation

**Documentation:**
- [README.md](RP2040-Async-Sensor/README.md) - Setup and usage guide
- [COMPARISON.md](RP2040-Async-Sensor/COMPARISON.md) - Detailed comparison with polling approach

### Other Projects

This directory also contains submodules for various other IoT projects:
- HC-SR501-Node-Red-Presence-Detector
- LED-Strip-Controller-Software
- Smart-RF-Plug-Transmitter
- Web-Based-Fan-Controller
