use std::fmt;
use std::fmt::{Formatter};
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use crate::pkt::answer::Answer;
use crate::pkt::header::Header;
use crate::pkt::question::Question;
use crate::pkt::Serializable;

pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>
}

impl Serializable for Message {
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        self.header.serialize(data);
        for q in self.questions.iter() {
            q.serialize(data);
        }
        for a in self.answers.iter() {
            a.serialize(data);
        }
    }
}

impl Message {
    pub fn deserialize(data: &[u8]) -> Message {
        let mut message = Message::new();
        let (mut buf , header) = Header::deserialize((data, 0)).unwrap();
        message.header = header;

        for _ in 0..message.header.qdcount {
            let (rem, q) = Question::deserialize(buf, data).unwrap();
            buf = rem;
            message.questions.push(q);
        }
        for _ in 0..message.header.ancount {
            let (rem, a) = Answer::deserialize(buf, data).unwrap();
            buf = rem;
            message.answers.push(a);
        }
        message
    }

    pub fn new() -> Message {
        Message {
            header: Header::new(),
            questions: vec![],
            answers: vec![]
        }
    }

    pub fn build(id: u16, url: &str, ty: &str) -> Message {
        let mut header = Header::new();
        header.id = id;
        header.rd = true;
        header.qdcount = 1;

        let mut question = Question::new();
        question.qname = url.parse().unwrap();
        question.qtype = ty.parse().unwrap();
        question.qclass = "IN".parse().unwrap();

        let mut message = Message::new();
        message.header = header;
        message.questions.push(question);
        message
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.header)?;
        writeln!(f, "Question(s)")?;
        for q in self.questions.iter() {
            write!(f, "{}", q)?;
        }

        writeln!(f, "Answer(s)")?;
        for a in self.answers.iter() {
            write!(f, "{}", a)?;
        }
        write!(f, "")
    }
}
