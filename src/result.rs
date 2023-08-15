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