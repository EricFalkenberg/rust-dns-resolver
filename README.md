# Rust DNS Resolver
A toy DNS resolver built while following the instruction of [this guide](https://implement-dns.wizardzines.com/).
> [!WARNING]
> This is a toy resolver and should not be used as a real one or used as an example for your implementation

```
Usage:
dns-resolver <url>
```
## Output Example
This example showcases what a query to a root name-server returns without the RECURSION_DESIRED flag turned on. The nameserver responds with a list of nameservers that posess the knowledge of where we can find www.google.com.
```
Output Example:
Querying DNS for: "www.google.com"
DNSPacket {
    header: DNSHeader {
        id: 27745,
        flags: 33280,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 13,
        num_additionals: 11,
    },
    questions: [
        DNSQuestion {
            name: "www.google.com",
            type_: 1,
            class_: 1,
        },
    ],
    answers: [],
    authorities: [
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "e.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "b.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "j.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "m.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "i.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "f.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "a.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "g.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "h.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "l.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "k.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "c.gtld-servers.net",
        },
        DNSRecord {
            name: "com",
            type_: 2,
            class_: 1,
            ttl: 172800,
            data: "d.gtld-servers.net",
        },
    ],
    additionals: [],
}
```
