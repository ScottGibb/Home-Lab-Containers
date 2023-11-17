#!/bin/bash

# Setup Script for downloading the LED Strip Controller Script

echo "Navigating to Home Directory"
cd /home/pi || exit
echo "Downloading LED Strip Controller Code"
git clone https://github.com/ScottGibb/LED-Strip-Controller-Octoprint.git
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

echo "Installing Octodash"
sh <(wget -qO- https://github.com/UnchartedBull/OctoDash/raw/main/scripts/install.sh)
echo "Finished Install"
