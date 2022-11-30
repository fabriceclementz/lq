# lq

> WIP

lq is command-line tool for processing and querying logs.

## Features

- Convert to JSON and CSV (WIP)
- Transform logs with a map function (WIP)
- Query logs with SQL (WIP)

## Usage

```sh
# convert NGINX logs to JSON
cat nginx.log | lq nginx -c json > logs.json

# convert NGINX logs to CSV
cat nginx.log | lq nginx -c csv > logs.csv

# convert NGINX logs to JSON with a map function
cat nginx.log | lq nginx -transform 'map { ts:ts:"%Y-%m-%d", ip:str, method:str, bytes:int, status_code:int } | json'
```

## Configuration file

```yaml
# patterns in /etc/lq.conf
nginx: ^(?P<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3})\s-\s-\s\[(?P<ts>.+)]\s"(?P<method>GET|POST|PUT|HEAD|PATCH|TRACE|OPTIONS|PURGE)\s(?P<uri>[^\s]*)\s(?P<protocol>[^"]*)"\s(?P<status_code>\d{3})\s
apache:
varnish: ^(?P<ip>\d{1,3}\.\d{1,3}.\d{1,3}.\d{1,3})\s-\s-\s\[(?P<ts>.+)\]\s"(?P<method>GET|POST|PUT|PATCH|TRACE|OPTIONS|PURGE)\s(?P<uri>http(s?)://[^\s]*)\s(?P<proto>[^\\/]*)/(?P<proto_version>[^"]*)"\s(?P<status_code>\d{3})\s(?P<bytes>\d+)\s
```
