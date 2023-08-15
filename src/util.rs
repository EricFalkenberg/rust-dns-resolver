use std::io::{Cursor, Error, Read, Seek, SeekFrom};
use std::net::{Ipv4Addr};
use byte_string::ByteString;

fn decode_compressed_name(cursor: &mut Cursor<ByteString>) -> Result<String, Error> {
    let mut buf = [0 as u8; 2];
    cursor.read(&mut buf)?;
    buf[0] = buf[0] & 0b0011_1111;
    let decoded_pointer = u16::from_be_bytes(buf);
    let current_pos = cursor.position();
    cursor.seek(SeekFrom::Start(decoded_pointer as u64))?;
    let result = decode_name(cursor);
    cursor.seek(SeekFrom::Start(current_pos))?;
    result
}
pub fn decode_name(cursor: &mut Cursor<ByteString>) -> Result<String, Error> {
    let mut parts: Vec<String> = Vec::new();
    loop {
        let length = cursor.bytes()
            .next()
            .transpose()?
            .unwrap_or(0);
        if length == 0 {
            break
        }
        else if (length & 0b1100_0000) == 192 {
            cursor.seek(SeekFrom::Current(-1))?;
            let result = decode_compressed_name(cursor)?;
            parts.push(result);
            break;
        } else {
            let mut buf = vec![0u8; length as usize];
            cursor.read_exact(&mut buf)?;
            parts.push(String::from_utf8(buf).unwrap());
        }
    }
    Ok(parts.join("."))
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
