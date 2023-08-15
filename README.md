# Rust DNS Resolver
A toy DNS resolver built while following the instruction of [this guide](https://implement-dns.wizardzines.com/).
> [!WARNING]
> This is a toy resolver and should not be used as a real one or used as an example for your implementation

```
Usage:
dns-resolver <url>
```
## Output Example
This is an example of a query for google.com's A records. The program requests information from one of the root nameservers and receives a response detailing authorities that possess the information we need. The program then makes requests to those authorities to get the necessary A records.
```
Querying DNS for: "www.google.com"
Querying DNS for: www.google.com at nameserver address 198.41.0.4
Querying DNS for: www.google.com at nameserver address 192.12.94.30
Querying DNS for: www.google.com at nameserver address 216.239.34.10
DNSPacket {
    header: DNSHeader {
        id: 37875,
        flags: 33792,
        num_questions: 1,
        num_answers: 6,
        num_authorities: 0,
        num_additionals: 0,
    },
    questions: [
        DNSQuestion {
            name: "www.google.com",
            type_: 1,
            class_: 1,
        },
    ],
    answers: [
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.99",
        },
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.147",
        },
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.106",
        },
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.103",
        },
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.105",
        },
        DNSRecord {
            name: "www.google.com",
            type_: 1,
            class_: 1,
            ttl: 300,
            data: "142.251.167.104",
        },
    ],
    authorities: [],
    additionals: [],
}
```
