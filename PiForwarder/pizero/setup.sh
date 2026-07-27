#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
repo_dir=$(dirname "$script_dir")
environment_file="$repo_dir/.env"
service_environment=/etc/pi-forwarder/pi-forwarder.env
nginx_site=/etc/nginx/sites-available/pi-forwarder

if [ "$(id -u)" -ne 0 ]; then
    echo "Run as root: sudo $0" >&2
    exit 1
fi

if [ ! -f "$environment_file" ]; then
    echo "Missing $environment_file; copy .env.example to .env and configure it first." >&2
    exit 1
fi

apt-get update
apt-get install -y ca-certificates curl gettext-base nginx

# Tailscale's repository supplies the current Raspberry Pi OS package, including
# the architecture required by a Pi Zero.
curl -fsSL https://tailscale.com/install.sh | sh

install -d -m 0700 /etc/pi-forwarder
install -m 0600 "$environment_file" "$service_environment"
"$repo_dir/scripts/render-forwarder-config.sh" "$service_environment" "$nginx_site"
ln -sfn ../sites-available/pi-forwarder /etc/nginx/sites-enabled/pi-forwarder
rm -f /etc/nginx/sites-enabled/default
install -m 0644 "$repo_dir/pizero/systemd/pi-forwarder-tailscale.service" /etc/systemd/system/pi-forwarder-tailscale.service

systemctl daemon-reload
systemctl enable --now tailscaled.service
set -a
# shellcheck disable=SC1090
. "$service_environment"
set +a
set -f
tailscale up --auth-key="$TAILSCALE_AUTH_KEY" --hostname="$TAILSCALE_HOSTNAME" $TS_EXTRA_ARGS
set +f
systemctl enable --now pi-forwarder-tailscale.service
nginx -t
systemctl enable --now nginx.service

echo "Pi Zero forwarder is configured. Check: systemctl status pi-forwarder-tailscale nginx"
