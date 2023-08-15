use num_derive::FromPrimitive;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(FromPrimitive)]
pub enum RecordType {
    UNKNOWN = -1, // UNKNOWN
    A = 1, // host address
    NS = 2, // authoritative name server
    MD = 3, // mail destination (obsolete - use MX)
    MF = 4, // mail forwarder (obsolete - use MX)
    CNAME = 5, // canonical name for an alias
    SOA = 6, // marks start of zone of authority
    MB = 7, // mailbox domain name (experimental)
    MG = 8, // mail group member (experimental)
    MR = 9, // mail rename domain name (experimental)
    NULL = 10, // null RR (experimental)
    WKS = 11, // well known service description
    PTR = 12, // domain name pointer
    HINFO = 13, // host information
    MINFO = 14, // mailbox or mail list information
    MX = 15, // mail exchange
    TXT = 16 // text strings
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(FromPrimitive)]
pub enum ClassType {
    UNKNOWN = -1, // UNKNOWN
    IN = 1, // internet
    CS = 2, // CSNET class (obsolete)
    CH = 3, // CHAOS class
    HS = 4  // Hesiod [Dyer 87]
}

