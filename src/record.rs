use byte_string::{ByteString};
use crate::rfc_type::RecordType;
use crate::util;

#[derive(Debug)]
pub struct DNSRecord {
    pub name: String,
    pub type_: u16,
    pub class_: u16,
    pub ttl: u32,
    pub data: String
}
impl DNSRecord {
    pub fn parse_from_response(bytes: &ByteString, start: usize) -> (DNSRecord, usize) {
        let (name, idx) = util::decode_name(bytes, start);
        let type_ = util::bytes_to_u16(bytes[idx], bytes[idx+1]);
        let class_ = util::bytes_to_u16(bytes[idx+2], bytes[idx+3]);
        let ttl = util::bytes_to_u32(bytes[idx+4], bytes[idx+5], bytes[idx+6], bytes[idx+7]);
        let data_len = util::bytes_to_u16(bytes[idx+8], bytes[idx+9]) as usize;
        if type_ == RecordType::A as u16 {
            let data: Vec<u8> = Vec::from(&bytes[idx+10..idx+10+data_len]);
            (DNSRecord {
                name,
                type_,
                class_,
                ttl,
                data: util::string_to_ip_addr(data)
            }, idx + 9 + data_len)
        } else if type_ == RecordType::NS as u16 {
            let (uri, idx) = util::decode_name(bytes, idx+10);
            (DNSRecord {
                name,
                type_,
                class_,
                ttl,
                data: uri
            }, idx)
        } else {
            let data: Vec<u8> = Vec::from(&bytes[idx+10..idx+10+data_len]);
            (DNSRecord {
                name,
                type_,
                class_,
                ttl,
                data: String::from_utf8(data).unwrap()
            }, idx + 9 + data_len)
        }
    }
}