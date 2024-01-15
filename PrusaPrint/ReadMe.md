# Prusa Print Docker Structure

## Summary

For Prusaprint, the setupOctoprint.sh should be called and the following docker stacks are enabled:

- utils

## Setup Octoprint

As for setting up Octoprint, the script setupOctoprint.sh will need to be run along the extra installs section of this ReadMe.md

### Restoring Backup and Plugins

You will also need to reinstall the plugins and the Octoprint configuration. This is done through the Octoprint UI and the following files:

- plugin_list.json
- octopi-backup-xxx.zip

It is recommended you do it, by first installing the plugins then applying the backup file, sadly the backup file cant be VC on GitHub so it can be found on the NAS.

### Installing the Spool Manager Plugin

Sadly as of the 15/0/2024, the Spool Manager provided by the Octoprint plugin repository is out of date and no longer works in terms of QR code functionality. In order to fix this the following repo can be used [mdziekon Fork](https://github.com/mdziekon/OctoPrint-SpoolManager). However this plugin also doesnt install properly using the link. The best result is to download the plugin using gihub clone specifically the V2.0.0 release.

### Extra Installs

As for extra installs, Octodash cant be installed using the plugin manager and the following command must be used:

```bash
bash <(wget -qO- https://github.com/UnchartedBull/OctoDash/raw/main/scripts/install.sh)
```

## Useful Links

- [OctoDash](https://github.com/UnchartedBull/OctoDash/wiki/Installation)
- [Camera Module V3 Autofocus Settings](https://community.octoprint.org/t/manual-raspi-camera-v3-webcam-auto-focus/53025)
- [DH11 Not showing up on GUI](https://github.com/vitormhenrique/OctoPrint-Enclosure/issues/435)
