# Pi Hall Docker Stack

## Summary

For the Hall Pi, the following docker stacks are enabled:

- utils
- tailscale
- pihole
- home-power-monitor

## Usage

To start all services:

```bash
git submodule update --init --recursive
docker compose up -d
```

The `home_power_monitor` service is built from the submodule at `PiHall/IOT/Home-Power-Monitor`. If that directory has not been initialized on the target machine, Docker Compose will fail with `failed to read dockerfile: open Dockerfile: no such file or directory`.

## Env Sensitivity

Due to the sensitive nature of this .env and Tailscale add the following for the nginx setup.

```txt
NGINX_CONF_PATH="/home/pi/Home-Lab-Containers/PiHall/nginx.conf"
NGINX_HOST="localhost"
NGINX_PORT="80"
```
