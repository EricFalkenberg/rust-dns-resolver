# Rust DNS Resolver
A toy DNS resolver built while following the instruction of [this guide](https://implement-dns.wizardzines.com/).
> [!WARNING]
> This is a toy resolver and should not be used as a real one or used as an example for your implementation

```
Usage:
dns-resolver <url>
```
## Output Example
This is an example of a query for twitter.com's A records. The program requests information from one of the root nameservers and receives a response detailing authorities that possess the information we need. The program then makes requests to those authorities to get the necessary A records.
```
Querying DNS for: www.twitter.com
Querying DNS for: www.twitter.com at nameserver address 198.41.0.4
Querying DNS for: www.twitter.com at nameserver address 192.12.94.30
Querying DNS for: a.r06.twtrdns.net at nameserver address 192.12.94.30
Querying DNS for: ns-370.awsdns-46.com at nameserver address 192.12.94.30
Querying DNS for: ns-370.awsdns-46.com at nameserver address 205.251.192.47
Querying DNS for: a.r06.twtrdns.net at nameserver address 205.251.193.114
Querying DNS for: www.twitter.com at nameserver address 205.251.192.179
DNSPacket {
    header: DNSHeader {
        id: 31690,
        flags: 33792,
        num_questions: 1,
        num_answers: 2,
        num_authorities: 8,
        num_additionals: 0,
    },
    questions: [
        DNSQuestion {
            name: "www.twitter.com",
            type_: 1,
            class_: 1,
        },
    ],
    answers: [
        DNSRecord {
            name: "www.twitter.com",
            type_: 5,
            class_: 1,
            ttl: 600,
            data: "�\u{10}",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 1,
            class_: 1,
            ttl: 1800,
            data: "104.244.42.193",
        },
    ],
    authorities: [
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "a.r06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "a.u06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "b.r06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "b.u06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "c.r06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "c.u06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "d.r06.twtrdns.net",
        },
        DNSRecord {
            name: "twitter.com",
            type_: 2,
            class_: 1,
            ttl: 13999,
            data: "d.u06.twtrdns.net",
        },
    ],
    additionals: [],
}
```
