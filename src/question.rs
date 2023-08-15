use std::io::{Cursor, Error, Read};
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
    pub fn parse_from_bytes(cursor: &mut Cursor<ByteString>) -> Result<DNSQuestion, Error> {
        let mut buf = [0 as u8; 2];
        let name = util::decode_name(cursor);
        cursor.read(&mut buf)?;
        let type_ = u16::from_be_bytes(buf);
        cursor.read(&mut buf)?;
        let class_ = u16::from_be_bytes(buf);
        Ok(
            DNSQuestion {
                name,
                type_,
                class_
            }
        )
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