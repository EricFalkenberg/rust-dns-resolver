use std::io::{Cursor, Error, ErrorKind};
use std::net::UdpSocket;
use std::ops::DerefMut;
use byte_string::ByteString;
use rand::Rng;
use crate::header::DNSHeader;
use crate::question::DNSQuestion;
use crate::record::DNSRecord;
use crate::result::DNSResult;
use crate::rfc_type::{ClassType, RecordType};

pub fn resolve(domain_name: &String) -> Result<DNSResult, Error> {
    let nameserver_ip = String::from("198.41.0.4");
    _resolve(domain_name, &nameserver_ip)
}
fn _resolve(domain_name: &String, nameserver_ip: &String) -> Result<DNSResult, Error> {
    println!("Querying DNS for: {0} at nameserver address {1}", domain_name, nameserver_ip);
    let query = build_query(domain_name, RecordType::A);
    let response = send_query(query, nameserver_ip)?;
    if response.further_info_required() {
        let nameserver_name = &response.authorities[0].data;
        // look for the ip for the next nameserver in the additionals
        for entry in &response.additionals {
            if entry.is_host_address_record_for(nameserver_name) {
                return _resolve(domain_name, &entry.data);
            }
        }
        // if we can't find it, we're in luck cause we're a dns resolver.
        let ns_result = _resolve(nameserver_name, nameserver_ip)?;
        if ns_result.answers.len() > 0 {
            let answer = ns_result.answers.get(0).unwrap();
            return _resolve(domain_name, &answer.data);
        }
        Err(Error::from(ErrorKind::NotFound))
    } else {
        Ok(response)
    }
}
fn build_query(domain_name: &String, record_type: RecordType) -> ByteString {
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
fn send_query(query: ByteString, nameserver_ip: &String) -> Result<DNSResult, Error> {
    let mut buf = [0; 1024];
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let mut ip = String::from(nameserver_ip);
    ip.push_str(":53");
    socket.send_to(query.as_slice(), ip)?;
    let response_size = socket.recv(&mut buf)?;
    let response_vector = Vec::from(&buf[0..response_size]);
    read_results(ByteString::from(response_vector))
}
fn read_results(response: ByteString) -> Result<DNSResult, Error> {
    let mut questions = Vec::new();
    let mut answers = Vec::new();
    let mut authorities = Vec::new();
    let mut additionals = Vec::new();
    let mut cursor = Cursor::new(response);
    let header = DNSHeader::parse_from_bytes(&mut cursor)?;
    for _ in 0..header.num_questions {
        let result = DNSQuestion::parse_from_bytes(&mut cursor)?;
        questions.push(result);
    }
    for _ in 0..header.num_answers {
        let record = DNSRecord::parse_from_response(&mut cursor)?;
        answers.push(record);
    }
    for _ in 0..header.num_authorities {
        let record = DNSRecord::parse_from_response(&mut cursor)?;
        authorities.push(record);
    }
    for _ in 0..header.num_additionals {
        let record = DNSRecord::parse_from_response(&mut cursor)?;
        additionals.push(record);
    }
    Ok(
        DNSResult {
            header,
            questions,
            answers,
            authorities,
            additionals
        }
    )
}

