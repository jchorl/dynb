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
    ```
1. `sudo systemctl enable dynb`
1. `sudo systemctl start dynb`
1. `journalctl -u dynb.service` for logs
