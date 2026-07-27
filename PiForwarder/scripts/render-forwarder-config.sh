#!/bin/sh
set -eu

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <environment-file> <output-file>" >&2
    exit 64
fi

environment_file=$1
output_file=$2
template_file="$(CDPATH= cd -- "$(dirname "$0")/../config" && pwd)/forwarder.conf.template"

set -a
# shellcheck disable=SC1090
. "$environment_file"
set +a

: "${NAS_TAILSCALE_IP:?NAS_TAILSCALE_IP must be set}"
: "${NAS_TAILSCALE_PORT:?NAS_TAILSCALE_PORT must be set}"

case "$NAS_TAILSCALE_PORT" in
    *[!0-9]* | '') echo "NAS_TAILSCALE_PORT must be a number" >&2; exit 64 ;;
esac

if [ "$NAS_TAILSCALE_PORT" -lt 1 ] || [ "$NAS_TAILSCALE_PORT" -gt 65535 ]; then
    echo "NAS_TAILSCALE_PORT must be between 1 and 65535" >&2
    exit 64
fi

envsubst '${NAS_TAILSCALE_IP} ${NAS_TAILSCALE_PORT}' < "$template_file" > "$output_file"
