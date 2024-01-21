# Storage Docker Stack

## Summary

The Storage docker stack is for selfhosted storage solutions such as Samba and GitLab. Within the docker compose file it is recommended to change the default passwords. For this reason the actual passwords are not version controlled. The lines of interest are the following:

```bash
    command: '-u "USERNAMEGOESHERE;PASSWORDGOESHERE"'
```
