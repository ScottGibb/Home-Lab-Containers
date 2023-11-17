#!/bin/bash

# Setup Script for downloading the LED Strip Controller Script

echo "Navigating to Home Directory"
cd /home/pi || exit
echo "Downloading LED Strip Controller Code"
git clone https://github.com/ScottGibb/LED-Strip-Controller-Octoprint.git
echo "Finished Download"
