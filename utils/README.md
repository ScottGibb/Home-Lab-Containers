# Utils Docker Package

## Summary

The utils subfolder of this repository, is full of docker containers that are used across each device in the Gibb Network. This stack is called utils and can be used simply by performing the following command:

```bash
docker compose up -d
```

## Services

### Core Services

- **Portainer**: Web-based Docker management interface (ports 8000, 9000, 9443)
- **Portainer Agent**: Remote agent for Portainer (port 9001)
- **Uptime Kuma**: Self-hosted monitoring tool (port 3001)

### Tools (Profile-based)

- **MegaLinter**: Code quality and security linting tool

## MegaLinter Usage

MegaLinter is available as a Docker Compose service for local linting. It's configured with the `tools` profile, so it won't start automatically with other services.

### Running MegaLinter Locally

From the repository root or any system directory (HPNas, PiHome, etc.):

```bash
# Run MegaLinter on the current directory
docker compose --profile tools run --rm megalinter

# Run MegaLinter on a specific directory
MEGALINTER_SOURCE_PATH=/path/to/code docker compose --profile tools run --rm megalinter
```

### Configuration

MegaLinter uses the configuration file `.mega-linter.yaml` in the repository root. The Docker Compose service is configured to:

- Apply fixes automatically (`APPLY_FIXES=all`)
- Validate the entire codebase (`VALIDATE_ALL_CODEBASE=true`)
- Use the documentation flavor for optimal performance

For more information about MegaLinter configuration, see the [official documentation](https://megalinter.io/).
