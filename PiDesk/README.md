# PiDesk Containers

## Summary

For the Desk Pi, the following docker stacks are enabled:

- utils
- networking
- printing (CUPS Dymo Label Writer 450)

This system runs on a Raspberry Pi Zero underneath the Standing Desk. This Pi is primarily used for running the label Maker that is sat above the Desk. The majority of the functionality of this system is outlined in the [CUPS Dymo Label Writer 450 repository](https://github.com/ScottGibb/Cups_Dymo-450)

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
