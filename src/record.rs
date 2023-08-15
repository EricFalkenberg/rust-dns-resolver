use std::io::{Cursor, Error, Read};
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
    pub fn parse_from_response(cursor: &mut Cursor<ByteString>) -> Result<DNSRecord, Error> {
        let mut u8_buf = [0u8; 2];
        let mut u32_buf = [0u8; 4];
        let name = util::decode_name(cursor)?;
        cursor.read(&mut u8_buf)?;
        let type_ = u16::from_be_bytes(u8_buf);
        cursor.read(&mut u8_buf)?;
        let class_ = u16::from_be_bytes(u8_buf);
        cursor.read(&mut u32_buf)?;
        let ttl = u32::from_be_bytes(u32_buf);
        cursor.read(&mut u8_buf)?;
        let data_len = u16::from_be_bytes(u8_buf) as usize;
        if type_ == RecordType::A as u16 {
            let mut data = vec![0u8; data_len];
            cursor.read(&mut data)?;
            Ok(
                DNSRecord {
                    name,
                    type_,
                    class_,
                    ttl,
                    data: util::string_to_ip_addr(data)
                }
            )
        } else if type_ == RecordType::NS as u16 || type_ == RecordType::CNAME as u16 {
            let uri = util::decode_name(cursor)?;
            Ok(
                DNSRecord {
                    name,
                    type_,
                    class_,
                    ttl,
                    data: uri
                }
            )
        } else {
            let mut data = vec![0u8; data_len];
            cursor.read(&mut data)?;
            Ok(
                DNSRecord {
                    name,
                    type_,
                    class_,
                    ttl,
                    data: String::from_utf8_lossy(&data).to_string()
                }
            )
        }
    }
    pub fn is_host_address_record_for(self: &DNSRecord, domain_name: &String) -> bool {
        self.type_ == RecordType::A as u16 && &self.name == domain_name
    }
}