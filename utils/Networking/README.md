# Utils Networking Docker Compose

## Summary

This folder contains the common Docker Compose configuration for the nginx service used across all systems in the Home Lab. Instead of duplicating the same Docker Compose configuration in each system's Networking folder, this centralized configuration uses environment variables to customize the setup for each system.

## Usage

This Docker Compose file is designed to be included in each system's main `docker-compose.yml` file using the `include` directive. Each system must provide the following environment variables:

### Required Environment Variables

- **NGINX_HOST**: The IP address of the system (e.g., `192.168.0.67`)
- **NGINX_CONF_PATH**: The absolute path to the system-specific nginx.conf file

### Example

In a system's main `docker-compose.yml`:

```yaml
include:
  - path: ../utils/Networking/docker-compose.yml
    env_file: ./Networking/.env
```

Or with inline environment variables:

```yaml
include:
  - path: ../utils/Networking/docker-compose.yml
    environment:
      NGINX_HOST: "192.168.0.67"
      NGINX_CONF_PATH: "/home/pi/Home-Lab-Containers/HPNas/Networking/nginx.conf"
```

## System-Specific Configuration

Each system still maintains its own:

1. **nginx.conf**: Contains system-specific proxy configurations and server blocks
2. **.env file** (optional): Contains environment variables specific to that system

The nginx.conf files remain in each system's Networking folder because they contain different proxy configurations for the services running on each system.

## Benefits

- **Reduced Redundancy**: Single source of truth for the nginx Docker Compose configuration
- **Easier Maintenance**: Updates to the base configuration only need to be made once
- **Clear Separation**: System-specific configurations (nginx.conf) remain separate from the common infrastructure
- **Consistency**: Ensures all systems use the same nginx image version and configuration structure
