# Pi Forwarder

## Summary

PiForwarder is a Raspberry Pi deployment for a remote household. It provides a
single, narrow route from the recipient LAN to one service on the NAS. Devices
on that LAN do not gain access to the rest of the home network.

Two implementations share the same connection settings and Nginx forwarding
template in this directory:

- `docker/`: Docker Compose, suitable for a Pi with Docker support.
- `pizero/`: native Tailscale and Nginx services, suitable for a Pi Zero where
  Docker is undesirable or unsupported.

The gateway listens on the configured port and forwards traffic over Tailscale
to the NAS.

## Usage

Reserve `192.168.0.67` for the Pi in the recipient router before starting the stack. The recipient LAN must use the `192.168.0.0/24` subnet for clients to use that address.

Create a private `.env` containing `TAILSCALE_AUTH_KEY`, `TAILSCALE_HOSTNAME`,
`TS_EXTRA_ARGS`, `NAS_TAILSCALE_IP`, and `NAS_TAILSCALE_PORT`. The Pi
listens on `NAS_TAILSCALE_PORT` and forwards to the same port on the NAS. The
`.env` file is ignored by Git.

Generate the Tailscale auth key as a pre-authorized key for this gateway in the Tailscale admin console. Keep the corresponding access policy private to your Tailnet administration.

For Docker, the shared utils stack additionally needs `NGINX_CONF_PATH`,
`NGINX_HOST`, and `NGINX_PORT`; the example points at
`/home/pi/Home-Lab-Containers/PiForwarder/config/nginx.conf`.

## Shared Nginx configuration

Both variants use `config/forwarder.conf.template`. Its
`NAS_TAILSCALE_IP` and `NAS_TAILSCALE_PORT` placeholders become the Nginx
`proxy_pass` destination.

The Docker Nginx image substitutes those values automatically. Native Nginx on
the Pi Zero does not, so `scripts/render-forwarder-config.sh` reads the private
environment file, validates the forwarding port, substitutes the two values,
and writes a usable Nginx site configuration. The Pi Zero setup calls it as:

```sh
./scripts/render-forwarder-config.sh \
  /etc/pi-forwarder/pi-forwarder.env \
  /etc/nginx/sites-available/pi-forwarder
```

## Docker variant

Start the stack on the remote Pi:

```bash
cd ~/Home-Lab-Containers/PiForwarder/docker
docker compose --env-file ../.env up -d
```

The explicit `--env-file ../.env` keeps the private environment shared between
the Docker and Pi Zero variants. Append normal Compose options as needed, for
example `docker compose --env-file ../.env up -d --build`.

## Pi Zero variant

Install Raspberry Pi OS Lite on the Pi Zero, clone this repository, create the
shared `.env`, then run:

```bash
cd ~/Home-Lab-Containers/PiForwarder
sudo ./pizero/setup.sh
```

The script installs Tailscale using its Raspberry Pi OS package source, plus
Nginx and `envsubst`. It copies the private settings to
`/etc/pi-forwarder/pi-forwarder.env`, renders the shared Nginx template, joins
the Tailnet once with the auth key, and enables `tailscaled`, the PiForwarder
settings service, and Nginx. It is safe to re-run after changing `.env`; it
replaces only PiForwarder-owned configuration. Restart
`pi-forwarder-tailscale` and `nginx` afterwards.

The forwarded service is then available on the recipient network at:

```text
http://192.168.0.67:<NAS_TAILSCALE_PORT>
```

The Tailnet policy should permit this gateway to access only the configured NAS
target port. Use a pre-authorized Tailscale auth key, preferably tagged for this
gateway; the key is needed only when the Pi first joins or is re-authenticated.
