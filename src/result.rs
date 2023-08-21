use crate::header::DNSHeader;
use crate::question::DNSQuestion;
use crate::record::DNSRecord;
use crate::rfc_type::{ClassType, RecordType};

#[derive(Debug)]
pub struct DNSResult {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSRecord>,
    pub authorities: Vec<DNSRecord>,
    pub additionals: Vec<DNSRecord>
}
impl DNSResult {
    pub fn further_info_required(self: &DNSResult) -> bool {
        self.answers.is_empty() && !self.authorities.is_empty()
    }
    pub fn print_records(self: &DNSResult) {
        println!("\nNAME\t\t\tTTL\tCLASS\tTYPE\tDATA");
        for answer in &self.answers {
            let class_: Option<ClassType> = num::FromPrimitive::from_u16(answer.class_);
            let type_: Option<RecordType> = num::FromPrimitive::from_u16(answer.type_);
            println!(
                "{0}\t\t{1}\t{2:?}\t{3:?}\t{4}",
                answer.name,
                answer.ttl,
                class_.unwrap_or(ClassType::UNKNOWN),
                type_.unwrap_or(RecordType::UNKNOWN),
                answer.data
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::header::DNSHeader;
    use crate::record::DNSRecord;
    use crate::result::DNSResult;

    #[test]
    fn further_info_required() {
        let result = DNSResult {
            header: DNSHeader {
                ..Default::default()
            },
            questions: vec![],
            answers: vec![],
            authorities: vec![
                DNSRecord {
                    name: String::from("www.google.com"),
                    type_: 1,
                    class_: 1,
                    ttl: 1,
                    data: String::from("asdfasdf")
                }
            ],
            additionals: vec![],
        };
        assert_eq!(result.further_info_required(), true);
    }
    #[test]
    fn further_info_not_required() {
        let result = DNSResult {
            header: DNSHeader {
                ..Default::default()
            },
            questions: vec![],
            answers: vec![
                DNSRecord {
                    name: String::from("www.google.com"),
                    type_: 1,
                    class_: 1,
                    ttl: 1,
                    data: String::from("asdfasdf")
                }
            ],
            authorities: vec![],
            additionals: vec![],
        };
        assert_eq!(result.further_info_required(), false);
    }
}