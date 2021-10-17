# dynb
dynb is a sidecar that takes http requests and sends `nsupdate` commands to BIND.

It is used to run your own dynmamic DNS.

## Installing
### On server
1. `sudo systemctl stop dynb`


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
1. `sudo systemctl daemon-reload`
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

## Adding a zone

```
$ sudo cat /etc/conf/named.conf.local
//
// Do any local configuration here
//

// Consider adding the 1918 zones here, if they are not used in your   
// organization     
//include "/etc/bind/zones.rfc1918";
zone "choo.dev" {
  type master;
  file "/var/lib/bind/db.choo.dev";
  allow-update { 127.0.0.1; };
};

zone "carta.horse" {
  type master;
  file "/var/lib/bind/db.carta.horse";
  allow-update { 127.0.0.1; };
};

logging {
  channel query.log { 
    file "/var/log/named/query.log";
    severity info;
  };
  channel update.log {
    file "/var/log/named/update.log";
    severity info;
  };

  category queries { query.log; };
  category update { update.log; };
};
```

```
$ sudo cat /var/lib/bind/db.choo.dev
$ORIGIN .
$TTL 86400      ; 1 day
choo.dev                IN SOA  choo.dev. admin.choo.dev. (
                                11         ; serial
                                604800     ; refresh (1 week)
                                86400      ; retry (1 day)
                                2419200    ; expire (4 weeks)
                                86400      ; minimum (1 day)
                                )
                        NS      ns1.choo.dev.
$ORIGIN choo.dev.
*                       A       67.164.56.164
ns1                     A       34.82.193.142
```
