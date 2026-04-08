# Pi Hall Docker Stack

## Summary

For the Hall Pi, the following docker stacks are enabled:

- utils
- tailscale
- pihole

## Usage

To start all services:

```bash
docker compose up -d
```

## Env Sensitivity

Due to the sensitive nature of this .env and Tailscale add the following for the nginx setup.

```txt
NGINX_CONF_PATH="/home/pi/Home-Lab-Containers/PiHall/nginx.conf"
```
