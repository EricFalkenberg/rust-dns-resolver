use byte_string::ByteString;
use crate::util;

#[derive(Debug)]
pub struct DNSQuestion {
    pub name: String,
    pub type_: u16,
    pub class_: u16
}
impl DNSQuestion {
    pub fn to_bytes(self: DNSQuestion) -> ByteString {
        let mut output= Vec::new();
        output.extend(self.encode_dns_name());
        output.extend_from_slice(&self.type_.to_be_bytes());
        output.extend_from_slice(&self.class_.to_be_bytes());
        ByteString::new(output)
    }
    pub fn parse_from_bytes(bytes: &ByteString, start: usize) -> (DNSQuestion, usize) {
        let (name, idx) = util::decode_name(bytes, start);
        let type_ = util::bytes_to_u16(bytes[idx], bytes[idx+1]);
        let class_ = util::bytes_to_u16(bytes[idx+2], bytes[idx+3]);
        (DNSQuestion {
            name,
            type_,
            class_
        }, idx + 4)
    }
    fn encode_dns_name(self: &DNSQuestion) -> Vec<u8> {
        let mut encoded = Vec::new();
        for part in self.name.to_ascii_lowercase().split('.') {
            encoded.push(part.len() as u8);
            encoded.extend_from_slice(part.as_bytes());
        }
        encoded.push(0x00);
        encoded
    }
}