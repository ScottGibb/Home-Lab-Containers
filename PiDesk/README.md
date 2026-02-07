# PiDesk Containers

## Summary

For the Desk Pi, the following docker stacks are enabled:

- utils
- printing

This system runs on a Raspberry Pi Zero underneath the Standing Desk. This Pi is primarily used for running the label Maker that is sat above the Desk. The majority of the functionality of this system is outlined in the [CUPS Dymo Label Writer 450 repository](https://github.com/ScottGibb/Cups_Dymo-450)

Simply run the following to get the PiDesk Docker stack up and then cd into the utils directory to use that docker stack as well.

```bash

bash docker-compose.sh --up # Bring up Stack
bash docker-compose.sh --up # Bring down Stack

```
