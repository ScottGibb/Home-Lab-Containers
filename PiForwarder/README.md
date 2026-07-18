# Pi Forwarder Docker Stack

## Summary

PiForwarder is a Raspberry Pi deployment for a remote household. It provides:

- utils
- tailscale
- a local gateway

Devices on the recipient's network connect to the Pi on the configured forwarding port. The gateway forwards that traffic over Tailscale to the NAS without giving that network access to the rest of the home LAN.

## Usage

Reserve `192.168.0.67` for the Pi in the recipient router before starting the stack. The recipient LAN must use the `192.168.0.0/24` subnet for clients to use that address.

Create a private `PiForwarder/.env` containing `TAILSCALE_AUTH_KEY`, `TAILSCALE_HOSTNAME`, `TS_EXTRA_ARGS`, `NAS_TAILSCALE_IP`, and `NAS_TAILSCALE_PORT`. The Pi listens on `NAS_TAILSCALE_PORT` and forwards to the same port on the NAS.

Generate the Tailscale auth key as a pre-authorized key for this gateway in the Tailscale admin console. Keep the corresponding access policy private to your Tailnet administration.

The shared utils stack also requires `NGINX_CONF_PATH`, `NGINX_HOST`, and `NGINX_PORT`; point `NGINX_CONF_PATH` at `/home/pi/Home-Lab-Containers/PiForwarder/config/nginx.conf`.

Start the stack on the remote Pi:

```bash
cd ~/Home-Lab-Containers/PiForwarder
docker compose up -d
```

The forwarded service is then available on the recipient network at:

```text
http://192.168.0.67:<NAS_TAILSCALE_PORT>
```

The Tailnet policy should permit this gateway to access only the configured NAS target port.
