extern crate byte_string;
extern crate hex;

mod header;
mod question;
mod record;
mod util;
mod rfc_type;
mod result;

use std::env;
use std::io::Cursor;
use std::net::UdpSocket;
use std::ops::{DerefMut};
use byte_string::{ByteString};
use header::DNSHeader;
use question::DNSQuestion;
use rand::Rng;

use crate::record::DNSRecord;
use crate::result::DNSPacket;
use crate::rfc_type::ClassType;
use crate::rfc_type::RecordType;

fn build_dns_query(domain_name: &String, record_type: RecordType) -> ByteString {
    let mut rng = rand::thread_rng();
    let header = DNSHeader {
        id: rng.gen_range(0..65535),
        num_questions: 1,
        ..Default::default()
    };
    let question = DNSQuestion {
        name: String::from(domain_name),
        type_: record_type as u16,
        class_: ClassType::IN as u16
    };
    let mut output = header.to_bytes();
    output.append(question.to_bytes().deref_mut());
    output
}

fn send_dns_query(query: ByteString) -> DNSPacket {
    let mut buf = [0; 1024];
    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("could not bind this address");
    socket.send_to(query.as_slice(), "198.41.0.4:53")
        .expect("could not send bytes");
    let response_size = socket.recv(&mut buf).unwrap();
    let response_vector = Vec::from(&buf[0..response_size]);
    read_results(ByteString::from(response_vector))
}

fn read_results(response: ByteString) -> DNSPacket {
    let mut questions = Vec::new();
    let mut answers = Vec::new();
    let mut authorities = Vec::new();
    let mut additionals = Vec::new();
    let mut cursor = Cursor::new(response);
    let header = DNSHeader::parse_from_bytes(&mut cursor);
    for _ in 0..header.num_questions {
        let result = DNSQuestion::parse_from_bytes(&mut cursor);
        questions.push(result);
    }
    for _ in 0..header.num_answers {
        let record = DNSRecord::parse_from_response(&mut cursor);
        answers.push(record);
    }
    for _ in 0..header.num_authorities {
        let record = DNSRecord::parse_from_response(&mut cursor);
        authorities.push(record);
    }
    for _ in 0..header.num_additionals {
        let record = DNSRecord::parse_from_response(&mut cursor);
        additionals.push(record);
    }
    DNSPacket {
        header,
        questions,
        answers,
        authorities,
        additionals
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("example.com");
    let domain = args.get(1).unwrap_or(&default);
    println!("Querying DNS for: {:?}", domain);
    let query = build_dns_query(domain, RecordType::A);
    let results = send_dns_query(query);
    println!("{:#?}", results);
}
