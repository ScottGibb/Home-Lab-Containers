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

# Installing Libcamera.conf
echo "Copying over libcamera config"
cp libcamera.conf /boot/camera-streamer/libcamera.conf
echo "Finishing Libcamera config"

# Overwriting getDHTTemp.py
echo "Overwriting getDHTTemp.py"
cp -rf getDHTTemp.py ~/oprint/lib/python3.9/site-packages/octoprint_enclosure/getDHTTemp.py
echo "Finished Overwriting getDHTTemp.py"

echo "Finished, octoprint setup"