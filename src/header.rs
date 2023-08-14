use byte_string::{ByteString};
use crate::util;

#[derive(Debug)]
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
    pub fn parse_from_bytes(bytes: &ByteString) -> DNSHeader {
        let input = bytes.to_vec();
        let id = util::bytes_to_u16(input[0], input[1]);
        let flags = util::bytes_to_u16(input[2], input[3]);
        let num_questions = util::bytes_to_u16(input[4], input[5]);
        let num_answers = util::bytes_to_u16(input[6], input[7]);
        let num_authorities = util::bytes_to_u16(input[8], input[9]);
        let num_additionals = util::bytes_to_u16(input[10], input[11]);
        DNSHeader {
            id,
            flags,
            num_questions,
            num_answers,
            num_authorities,
            num_additionals
        }
    }
}
impl Default for DNSHeader {
    fn default() -> DNSHeader {
        DNSHeader {
            id: 0,
            flags: 0,
            num_questions: 0,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0
        }
    }
}