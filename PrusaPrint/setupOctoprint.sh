#!/bin/bash

# Setup Script for downloading the LED Strip Controller Script

echo "Navigating to Home Directory"
cd /home/pi || exit
echo "Downloading LED Strip Controller Code"
git clone https://github.com/ScottGibb/LED-Strip-Controller-Octoprint.git
echo "Finished Download"

# Install Temp Sensor drivers DHT11
echo "Setting up DHT11 Drivers
pip3 install adafruit-circuitpython-dht
sudo apt-get update -y
sudo apt-get install libgpiod2 -y
echo "Setup Complete"
