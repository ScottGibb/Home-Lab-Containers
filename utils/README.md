# Utils Docker Package

## Summary

The utils subfolder of this repository contains Docker containers and configurations that are used across each device in the Gibb Network. 

## Components

### Shared Services (utils/docker-compose.yml)

This stack contains services that are deployed on every system:

- **Portainer**: Container management UI
- **uptime-kuma**: Uptime monitoring service

To start these services, run from any system directory:

```bash
docker compose up -d
```

### Networking (utils/Networking/)

The `Networking` subfolder contains the common nginx reverse proxy configuration used across all systems. This configuration uses environment variables to customize settings for each system (IP addresses, configuration file paths, etc.).

See [utils/Networking/README.md](./Networking/README.md) for more details on the networking setup.
