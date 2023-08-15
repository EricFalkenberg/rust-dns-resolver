# Rust DNS Resolver [![Rust](https://github.com/EricFalkenberg/rust-dns-resolver/actions/workflows/rust.yml/badge.svg)](https://github.com/EricFalkenberg/rust-dns-resolver/actions/workflows/rust.yml)
A toy DNS resolver built while following the instruction of [this guide](https://implement-dns.wizardzines.com/).
> [!WARNING]
> This is a toy resolver and should not be used in any official capacity. 

```
Usage:
dns-resolver <url>
```
## Output Example
The following shows the output when we attempt to resolve the IP address of `www.twitter.com`.
```
Querying 198.41.0.4 for www.twitter.com
Querying 192.12.94.30 for www.twitter.com
Querying 192.12.94.30 for a.r06.twtrdns.net
Querying 192.12.94.30 for ns-370.awsdns-46.com
Querying 205.251.192.47 for ns-370.awsdns-46.com
Querying 205.251.193.114 for a.r06.twtrdns.net
Querying 205.251.192.179 for www.twitter.com

NAME			TTL	CLASS	TYPE	DATA
www.twitter.com		600	IN	CNAME	twitter.com
twitter.com		1800	IN	A	104.244.42.193
```
