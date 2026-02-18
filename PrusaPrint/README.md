# Prusa Print Docker Structure

## Summary

For Prusaprint, the setupOctoprint.sh should be called and the following docker stacks are enabled:

- utils
- networking

## Usage

To start all services:

```bash
docker compose up -d
```

To stop all services:

```bash
docker compose down
```

To view logs:

```bash
docker compose logs -f
```

## Setup Octoprint

As for setting up Octoprint, the script setupOctoprint.sh will need to be run along the extra installs section of this ReadMe.md

### Restoring Backup and Plugins

You will also need to reinstall the plugins. This is done through the Octoprint plugin manager UI and the following files:

- plugin_list.json

Simply upload it and it will do the rest of installing them.

### Extra Installs

#### Docker

Required for the rest of the home lab network, will need to install docker engine and then run docker compose.
You will need to install it using the following [instructions](https://docs.docker.com/engine/install/raspberry-pi-os/)

Once this is done you can then start the stack using:

```bash
docker compose up -d
```

#### LED Strip Controller

Required for turning off and on the lights, you will have to enter the following into scripts outputs of the enclosure plugin

Turn On the Lights

```bash
~/LED-Strip-Controller-Octoprint/scripts/TurnOnLights.sh
```

Turn off the Lights

```bash
~/LED-Strip-Controller-Octoprint/scripts/TurnOffLights.sh
```

#### Setup DHT11 Sensor

The DHT11 sensor implementation has been refactored to use Rust with async GPIO methods via the `rppal` library. This provides better performance and more reliable sensor readings on Raspberry Pi.

The setup script (`setupOctoprint.sh`) will automatically:
- Install the Rust toolchain if not present
- Build the DHT11 sensor binaries with async GPIO support
- Install the binaries to `/usr/local/bin/`
- Create compatibility wrappers for existing tools

**Manual Testing:**

To test the DHT11 sensor, run:

```bash
test_dht11
```

This will continuously read and display temperature and humidity values from GPIO pin 18 (configurable in the source).

**Usage with Octoprint Enclosure Plugin:**

The `dht_sensor` binary can be used directly:

```bash
dht_sensor 11 18  # For DHT11 on GPIO 18
```

Or via the compatibility wrapper:

```bash
getDHTTemp.sh 11 18
```

**Legacy Python Implementation:**

The original Python scripts ([getDHTTemp.py](./getDHTTemp.py), [testDHT11.py](./testDHT11.py)) are retained in the repository for reference and backward compatibility, but are no longer used by default in the setup process. The Rust implementation provides the same functionality with improved reliability through async GPIO operations and better performance.

#### Telegram

When setting up the Telegram plugin, you will have to set up the botfather token and thent talk to the bot, to enable all the features.

You will also need to add the following to before and after the camera takes a photo:

Turn On the Lights

```bash
~/LED-Strip-Controller-Octoprint/scripts/TurnOnLights.sh
```

Turn off the Lights

```bash
~/LED-Strip-Controller-Octoprint/scripts/TurnOffLights.sh
```

This is so that the images are actually seeable in the dark.

##### Potential Issues

When setting up the telegram plugin, I had issues where i could not have the multicam plugin enabled at the same time. This meant that the status plugin command didnt work.

#### Octolapse

The scripts within octolapse have to be from the root level and cant use `~`, thus the scripts are as follows:

```bash
/home/pi/LED-Strip-Controller-Octoprint/scripts/TurnOffLights.sh

/home/pi/LED-Strip-Controller-Octoprint/scripts/TurnOnLights.sh
```

![alt text](./docs/octolapse_settings.png)

#### Spoolman

![alt text](./docs/spoolman_settings.png)

#### Octodash

As for extra installs, Octodash cant be installed using the plugin manager and the following command must be used:

```bash
bash <(wget -qO- https://github.com/UnchartedBull/OctoDash/raw/main/scripts/install.sh)
```

## Useful Links

- [OctoDash](https://github.com/UnchartedBull/OctoDash/wiki/Installation)
- [Camera Module V3 Autofocus Settings](https://community.octoprint.org/t/manual-raspi-camera-v3-webcam-auto-focus/53025)
- [DHT11 Not showing up on GUI](https://github.com/vitormhenrique/OctoPrint-Enclosure/issues/435)
- [DHT11 Setup](https://pimylifeup.com/raspberry-pi-dht11-sensor/)
