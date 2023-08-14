use std::net::{Ipv4Addr};
use byte_string::ByteString;

pub fn bytes_to_u16(a: u8, b: u8) -> u16 {
    let bytes = [a, b];
    u16::from_be_bytes(bytes)
}
pub fn bytes_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    let bytes = [a, b, c, d];
    u32::from_be_bytes(bytes)
}
fn decode_compressed_name(bytes: &ByteString, pointer: usize, length: usize) -> (String, usize) {
    let decoded_pointer = bytes_to_u16((length & 0b0011_1111) as u8, bytes[pointer + 1]);
    let current_pos = pointer + 1;
    let (result, _) = decode_name(&bytes, decoded_pointer as usize);
    (result, current_pos)
}
pub fn decode_name(bytes: &ByteString, start: usize) -> (String, usize) {
    let mut parts: Vec<String> = Vec::new();
    let mut idx: usize = start;
    while bytes[idx] != 0 {
        let length: usize = bytes[idx] as usize;
        if (length & 0b1100_0000) == 192 {
            let (result, next) = decode_compressed_name(bytes, idx, length);
            parts.push(result);
            idx = next;
            break;
        } else {
            let slice = Vec::from(&bytes[(idx+1)..idx+1+length]);
            parts.push(String::from_utf8(slice).unwrap());
            idx = idx + length + 1;
        }
    }
    (parts.join("."), idx + 1)
}
pub fn string_to_ip_addr(bytes: Vec<u8>) -> String {
    if bytes.len() == 4 {
        let a = bytes.get(0).unwrap().to_owned();
        let b = bytes.get(1).unwrap().to_owned();
        let c = bytes.get(2).unwrap().to_owned();
        let d = bytes.get(3).unwrap().to_owned();
        Ipv4Addr::new(a, b, c, d).to_string()
    } else {
        panic!("Could not read ip address from bytes: {:?}", bytes);
    }
}
