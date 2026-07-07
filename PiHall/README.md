# Pi Hall Docker Stack

## Summary

For the Hall Pi, the following docker stacks are enabled:

- utils
- tailscale
- pihole
- home-power-monitor

## Usage

Build the Home Power Monitor image from a standalone clone on a stronger 64-bit Mac, then copy it to the Pi 3:

```bash
git clone git@github.com:ScottGibb/Home-Power-Monitor.git
cd Home-Power-Monitor

docker buildx build \
  --platform linux/arm64 \
  --target prod \
  -t home-power-monitor:pi3 \
  --build-arg FEATURES="pi-buttons pi-screen mqtt database pir" \
  --load \
  .

docker save home-power-monitor:pi3 -o home-power-monitor-pi3.tar

# Copy the saved image archive to the Pi.
scp home-power-monitor-pi3.tar pi@pihall.local:~

# Import that archive into Docker on the Pi as the home-power-monitor:pi3 image.
ssh pi@pihall.local 'docker load -i ~/home-power-monitor-pi3.tar'
```

Then on the Pi start the stack:

```bash
cd ~/Home-Lab-Containers/PiHall
docker compose up -d
```

The PiHall compose file now runs the prebuilt `home-power-monitor:pi3` image by default. Override it with `HOME_POWER_MONITOR_IMAGE` if you want to use a different tag:

```bash
export HOME_POWER_MONITOR_IMAGE=home-power-monitor:custom
docker compose up -d
```

This is currently preferable to building from the `PiHall/IOT/Home-Power-Monitor` submodule inside this repository. `commitment-issues` uses Git metadata during the Rust build, and submodules expose `.git` as a pointer back into the parent repository rather than as a self-contained Git directory. That breaks isolated Docker builds from the submodule checkout. See: <https://github.com/dysonltd/commitment-issues/issues/40>

## Env Sensitivity

Due to the sensitive nature of this .env and Tailscale add the following for the nginx setup.

```txt
NGINX_CONF_PATH="/home/pi/Home-Lab-Containers/PiHall/nginx.conf"
NGINX_HOST="localhost"
NGINX_PORT="80"
```
