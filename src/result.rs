use crate::header::DNSHeader;
use crate::question::DNSQuestion;
use crate::record::DNSRecord;

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
        self.answers.len() == 0 && self.authorities.len() > 0
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