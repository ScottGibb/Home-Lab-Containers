# Pi Home Docker Stack

## Summary

For the Home Pi, the following docker stacks are enabled:

- utils
- iot
- networking
- apps
- security
- finances (optional, commented out by default)

## Usage

To start all services:

```bash
docker compose up -d
```

To stop all services:

```bash
docker compose down
```

To view logs:

```bash
docker compose logs -f
```
