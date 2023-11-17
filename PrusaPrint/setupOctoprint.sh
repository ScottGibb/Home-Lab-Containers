#!/bin/bash

# Setup Script for downloading the LED Strip Controller Script
current_dir=$(pwd)
echo "Navigating to Home Directory"
cd /home/pi || { echo "Failed to navigate to /home/pi. Exiting."; exit 1; }
echo "Downloading LED Strip Controller Code"
git clone https://github.com/ScottGibb/LED-Strip-Controller-Octoprint.git || { echo "Failed to download LED Strip Controller Code. Exiting."; exit 1; }
echo "Finished Download"

# Install Temp Sensor drivers DHT11
echo "Setting up DHT11 Drivers"
pip3 install adafruit-circuitpython-dht
apt-get update -y
apt-get install libgpiod2 -y
echo "Setup Complete"

# Install CPU limit for creating gifs
echo "Installing telegram dependencies"
apt-get install cpulimit -y
echo "Finished Installation"

# Install Octodash Plugin
cd "$current_dir" || { echo "Failed to navigate back to the original directory. Exiting."; exit 1; }
echo "Downloading Octodash"
wget -qO- https://github.com/UnchartedBull/OctoDash/raw/main/scripts/install.sh > setupOctoDash.sh || { echo "Failed to download Octodash. Exiting."; exit 1; }
echo "Please run setupOctoDash.sh"
