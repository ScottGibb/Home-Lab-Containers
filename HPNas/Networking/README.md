# Networking Docker Stack

## Summary

This docker stack is all around IP networking, specifically setting up nginx in connection with Pi-Hole. 

## Configuration

The nginx Docker Compose configuration is now centralized in `utils/Networking/docker-compose.yml` and uses environment variables defined in the `.env` file in this folder.

### Environment Variables

The `.env` file in this folder contains:
- **NGINX_HOST**: The IP address of this system (192.168.0.67)
- **NGINX_CONF_PATH**: The path to the nginx.conf file for this system

### Nginx Configuration

The `nginx.conf` file in this folder contains the system-specific proxy configurations and server blocks. To use this configuration, copy it to the relevant directory:

```bash
cp nginx.conf /home/pi/config/nginx/etc/nginx/nginx.conf
```

### Starting the Service

To start the networking services for this system, run from the HPNas directory:

```bash
docker compose up -d
```

The main `docker-compose.yml` file in the HPNas directory includes both the common networking configuration from `utils/Networking` and the system-specific settings from this folder's `.env` file.
