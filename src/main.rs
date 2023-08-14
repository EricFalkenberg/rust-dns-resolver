extern crate byte_string;
extern crate hex;

mod header;
mod question;
mod record;
mod util;
mod rfc_type;
mod result;

use std::env;
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

fn send_dns_query(query: ByteString) -> ByteString {
    let mut buf = [0; 1024];
    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("could not bind this address");
    socket.send_to(query.as_slice(), "198.41.0.4:53")
        .expect("could not send bytes");
    let response_size = socket.recv(&mut buf).unwrap();
    let response_vector = Vec::from(&buf[0..response_size]);
    ByteString::from(response_vector)
}

fn read_results(response: ByteString) -> DNSPacket {
    let mut questions = Vec::new();
    let mut answers = Vec::new();
    let mut authorities = Vec::new();
    let mut additionals = Vec::new();
    let header = DNSHeader::parse_from_bytes(&response);
    let mut idx = 12;
    for _ in 0..header.num_questions {
        let (result, new_pos) = DNSQuestion::parse_from_bytes(&response, idx);
        questions.push(result);
        idx = new_pos;
    }
    for _ in 0..header.num_answers {
        let (record, new_pos) = DNSRecord::parse_from_response(&response, idx);
        answers.push(record);
        idx = new_pos;
    }
    for _ in 0..header.num_authorities {
        let (record, new_pos) = DNSRecord::parse_from_response(&response, idx);
        authorities.push(record);
        idx = new_pos;
    }
    // for _ in 0..header.num_additionals {
    //     let (record, new_pos) = DNSRecord::parse_from_response(&response, idx);
    //     additionals.push(record);
    //     idx = new_pos;
    // }
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
    let response = send_dns_query(query);
    let results = read_results(response);
    println!("{:#?}", results);
}
