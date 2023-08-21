use std::io::{Cursor, Error, Read};
use byte_string::{ByteString};

#[derive(Debug, Default)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16
}
impl DNSHeader {
    pub fn to_bytes(self: DNSHeader) -> ByteString {
        let mut output= Vec::new();
        output.extend_from_slice(&self.id.to_be_bytes());
        output.extend_from_slice(&self.flags.to_be_bytes());
        output.extend_from_slice(&self.num_questions.to_be_bytes());
        output.extend_from_slice(&self.num_answers.to_be_bytes());
        output.extend_from_slice(&self.num_authorities.to_be_bytes());
        output.extend_from_slice(&self.num_additionals.to_be_bytes());
        ByteString::new(output)
    }
    pub fn parse_from_bytes(cursor: &mut Cursor<ByteString>) -> Result<DNSHeader, Error> {
        let mut buf= [0u8; 2];
        cursor.read_exact(&mut buf)?;
        let id = u16::from_be_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let flags = u16::from_be_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let num_questions = u16::from_be_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let num_answers = u16::from_be_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let num_authorities = u16::from_be_bytes(buf);
        cursor.read_exact(&mut buf)?;
        let num_additionals = u16::from_be_bytes(buf);
        Ok(
            DNSHeader {
                id,
                flags,
                num_questions,
                num_answers,
                num_authorities,
                num_additionals
            }
        )
    }
}