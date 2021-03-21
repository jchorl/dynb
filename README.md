# dynb
dynb is a sidecar that takes http requests and sends `nsupdate` commands to BIND.

It is used to run your own dynmamic DNS.

## Installing
### Local
1. `cargo build --release`
1. `scp target/release/dynb bind:`
1. `scp scripts/dynb.service bind:`

### On server
1. `sudo cp dynb.service /etc/systemd/system/`
1. `sudo EDITOR=vim systemctl edit dynb`
    Add environment vars, e.g.
    ```bash
    [Service]
    Environment="PORT=8080"
    Environment="AUTHENTICATION_TOKEN=<token>"
    ```
1. `sudo systemctl enable dynb`
1. `sudo systemctl start dynb`
1. `journalctl -u dynb.service` for logs

## Croning up a client
```shell
j@troy:~$ sudo chmod +x /etc/cron.hourly/ddns
j@troy:~$ sudo cat /etc/cron.hourly/ddns
#!/bin/sh

set -eux

curl \
  -X PUT \
  -H "Authorization: Bearer <auth token>" \
  https://ns1.choo.dev/update

sudo docker run -it --rm \
    jchorl/wdping \
    --name ddns \
    --frequency daily \
    --domain https://watchdog.joshchorlton.com
```
