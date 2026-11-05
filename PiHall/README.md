# Pi Hall Docker Stack

## Summary

For the Hall Pi, the following docker stacks are enabled:

- utils
- tailscale
- pihole
- home-power-monitor
- home-power-monitor-analytics (REST API and MCP)

## Usage

Build the Home Power Monitor image from a standalone clone on a stronger 64-bit Mac, then copy it to the Pi 3:

```bash
git clone git@github.com:ScottGibb/Home-Power-Monitor.git
cd Home-Power-Monitor

docker buildx build \
  --platform linux/arm64 \
  --target prod \
  -t home-power-monitor:pi3 \
  --build-arg FEATURES="pi-buttons pi-screen mqtt database pir http" \
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

## Analytics API and MCP

`IOT/Home-Power-Monitor-Analytics` pins the analytics source as a submodule.
Compose runs the prebuilt `home-power-monitor-analytics:pi3` image alongside the
monitor, sharing its PostgreSQL database and `home_power_network`. Analytics
uses the monitor's internal Docker address for live readings; it does not create
or migrate the database. The existing `postgres_data` volume is unchanged.

Build from a standalone clone, as for the monitor above, so Git build metadata
is available. The pinned analytics commit must be published to its remote first.
From the Home-Lab-Containers repository root on the Mac:

```bash
analytics_revision=$(git -C PiHall/IOT/Home-Power-Monitor-Analytics rev-parse HEAD)
analytics_build_dir=$(mktemp -d)
git clone https://github.com/ScottGibb/Home-Power-Monitor-Analytics.git "$analytics_build_dir"
git -C "$analytics_build_dir" checkout --detach "$analytics_revision"
docker buildx build --platform linux/arm64 --target prod \
  -t home-power-monitor-analytics:pi3 --load "$analytics_build_dir"
docker save home-power-monitor-analytics:pi3 -o home-power-monitor-analytics-pi3.tar
scp home-power-monitor-analytics-pi3.tar pi@pihall.local:~
ssh pi@pihall.local 'docker load -i ~/home-power-monitor-analytics-pi3.tar'
```

After review and publishing the repository changes, update the Pi checkout and
initialise the new submodule. With the existing database and monitor running,
start analytics without recreating those services:

```bash
cd ~/Home-Lab-Containers
git pull --ff-only
git submodule update --init --recursive PiHall/IOT/Home-Power-Monitor-Analytics
cd PiHall
docker compose up -d --no-deps --no-build --pull never home_power_monitor_analytics
```

The default REST address is `http://pihall.local:8081` and the MCP endpoint is
`http://pihall.local:8081/mcp`. Check database connectivity with:

```bash
curl --fail http://pihall.local:8081/ready
```

Optional Compose overrides are `HOME_POWER_MONITOR_ANALYTICS_IMAGE`,
`ANALYTICS_HTTP_PORT` (default `8081`), `ANALYTICS_MAX_SERIES_BUCKETS` (default
`10000`), and `ANALYTICS_MCP_ALLOWED_HOSTS`. The default MCP host allowlist
includes PiHall's short name, `.local` name, and loopback names at the selected
port. To connect by IP address, supply a complete allowlist including that IP
and `IP:port`. The existing `HTTP_PORT` still controls the monitor, not analytics.

The API has no built-in authentication; keep its published port on the trusted
LAN. Do not use `docker compose down --volumes` during this update.

## Env Sensitivity

Due to the sensitive nature of this .env and Tailscale add the following for the nginx setup.

```txt
NGINX_CONF_PATH="/home/pi/Home-Lab-Containers/PiHall/nginx.conf"
NGINX_HOST="localhost"
NGINX_PORT="80"
```
