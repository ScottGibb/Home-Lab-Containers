# Networking Docker Stack

## Summary

This docker stack is all around IP networking, specifically setting up nginx.

## Configuration

The nginx Docker Compose configuration is now centralized in `utils/Networking/docker-compose.yml` and uses environment variables defined in the `.env` file in this folder.

### Environment Variables

The `.env` file in this folder contains:
- **NGINX_HOST**: The IP address of this system (192.168.0.81)
- **NGINX_CONF_PATH**: The path to the nginx.conf file for this system

### Nginx Configuration

The `nginx.conf` file in this folder contains the system-specific proxy configurations and server blocks.

### Starting the Service

To start the networking services for this system, run from the PrusaPrint directory:

```bash
docker compose up -d
```

The main `docker-compose.yml` file in the PrusaPrint directory includes both the common networking configuration from `utils/Networking` and the system-specific settings from this folder's `.env` file.
