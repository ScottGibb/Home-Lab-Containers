# Home Lab Containers

[![MegaLinter](https://github.com/ScottGibb/Home-Lab-Containers/actions/workflows/mega_linter.yaml/badge.svg)](https://github.com/ScottGibb/Home-Lab-Containers/actions/workflows/mega_linter.yaml)

![Tools and Processes](./docs/languages_and_tools.svg)

## Summary

This repository keeps an active list of all the Docker stacks and key software components for each Pi in my home. This was done to version control each device in my house and maintain the configs across multiple linux devices.

## Downloading the Repository

When downloading the repository, it is advised to recursively download all the submodules as well. To do so, please use the following commands:

```bash
git submodule update --init --recursive
```

## Usage

The repository uses Docker Compose's include feature to organize services across multiple compose files. Each system (HPNas, PiDesk, PiHome, PiLab, PrusaPrint) has a main `docker-compose.yml` file that includes all necessary service definitions.

### Starting Services

To start all services for a specific system, navigate to its directory and run:

```bash
cd <system-directory>  # e.g., cd PiHome
docker compose up -d
```

### Stopping Services

To stop all services for a specific system:

```bash
cd <system-directory>
docker compose down
```

### Viewing Logs

To view logs for all services:

```bash
cd <system-directory>
docker compose logs -f
```

### System Organization

Each system's main `docker-compose.yml` includes:

- **utils/**: Shared services (Portainer, uptime-kuma) used across all systems
- **System-specific services**: Organized in subdirectories (Networking, Storage, IOT, etc.)

### Project Names

Each system has a unified project name defined in its main compose file:

- **HPNas**: `hpnas`
- **PiDesk**: `printing`
- **PiHome**: `pihome`
- **PiLab**: `pilab`
- **PrusaPrint**: `prusaprint`

**Note**: Previously, each subdirectory had its own project name (e.g., `networking`, `iot`, `apps`, `storage`). With the Docker Compose `include` directive, all services are now consolidated under a single project name per system. This is the correct and expected behavior - all included services become part of the parent project.

### Environment Variables

All environment variables are now defined inline in the docker-compose files. You can customize them by editing the respective `docker-compose.yml` files directly.
