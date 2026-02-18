# Networking Docker Stack

## Summary

The networking stack is primarily concerned with nginx and PiHole for in-house DNS and adblocking functionality.

## Configuration

### Nginx Service

The nginx Docker Compose configuration is now centralized in `utils/Networking/docker-compose.yml` and uses environment variables defined in the `.env` file in this folder.

The `.env` file in this folder contains:
- **NGINX_HOST**: The IP address of this system (192.168.0.68)
- **NGINX_CONF_PATH**: The path to the nginx.conf file for this system

The `nginx.conf` file in this folder contains the system-specific proxy configurations and server blocks.

### PiHole Service

The PiHole service configuration remains in this folder's `docker-compose.yml` file as it is unique to the PiHome system.

### Starting the Services

To start the networking services for this system, run from the PiHome directory:

```bash
docker compose up -d
```

The main `docker-compose.yml` file in the PiHome directory includes:
1. The common networking configuration (nginx) from `utils/Networking` with system-specific environment variables
2. The PiHome-specific networking configuration (pihole) from this folder
