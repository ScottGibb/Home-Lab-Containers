#!/bin/sh

# Setup Script for downloading the LED Strip Controller Script

echo "Starting OCtoprint Install"

current_dir=$(pwd)
echo "Navigating to Home Directory"
cd /home/pi || { echo "Failed to navigate to /home/pi. Exiting."; exit 1; }
echo "Downloading LED Strip Controller Code"
git clone https://github.com/ScottGibb/LED-Strip-Controller-Octoprint.git || { echo "Failed to download LED Strip Controller Code. Exiting."; exit 1; }
cd "$current_dir" || { echo "Failed to navigate back to the original directory. Exiting."; exit 1; }
echo "Finished Download"


# Install Rust (required for DHT11 async GPIO implementation)
echo "Installing Rust toolchain"
if ! command -v cargo > /dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck source=/dev/null
    . "$HOME/.cargo/env"
fi
echo "Rust installation complete"

# Build DHT11 sensor Rust binaries
echo "Building DHT11 sensor binaries with async GPIO"
cd "$current_dir" || { echo "Failed to navigate to current directory. Exiting."; exit 1; }
cargo build --release || { echo "Failed to build Rust binaries. Exiting."; exit 1; }
echo "Build complete"

# Install binaries to /usr/local/bin
echo "Installing DHT11 sensor binaries"
cp target/release/dht_sensor /usr/local/bin/dht_sensor || { echo "Failed to install dht_sensor. Exiting."; exit 1; }
cp target/release/test_dht11 /usr/local/bin/test_dht11 || { echo "Failed to install test_dht11. Exiting."; exit 1; }
chmod +x /usr/local/bin/dht_sensor
chmod +x /usr/local/bin/test_dht11
echo "DHT11 sensor binaries installed"

# Create wrapper script for compatibility with octoprint_enclosure plugin
echo "Creating compatibility wrapper"
cat > /usr/local/bin/getDHTTemp.sh << 'EOF'
#!/bin/sh
# Wrapper script to maintain compatibility with octoprint_enclosure plugin
# Calls the Rust implementation with async GPIO
exec /usr/local/bin/dht_sensor "$@"
EOF
chmod +x /usr/local/bin/getDHTTemp.sh
echo "Compatibility wrapper created"

# Install CPU limit for creating gifs
echo "Installing telegram dependencies"
apt-get install cpulimit -y
echo "Finished Installation"

# Installing Libcamera.conf
echo "Copying over libcamera config"
cp libcamera.conf /boot/camera-streamer/libcamera.conf
echo "Finishing Libcamera config"

echo "Finished, octoprint setup"