use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

const _QR: u16 = 1 << 15;
const _OP: u16 = 1 << 14;
const _OP1: u16 = 1 << 13;
const _OP2: u16 = 1 << 12;
const _OP3: u16 = 1 << 11;
const _AA: u16 = 1 << 10;
const _TC: u16 = 1 << 9;
const _RD: u16 = 1 << 8;
const _RA: u16 = 1 << 7;
const _Z: u16 = 1 << 6;
const _AD: u16 = 1 << 5;
const _CD: u16 = 1 << 4;

pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>
}

#[derive(Debug)]
pub struct Header {
    id: u16,
    qr: bool,
    opcode: u8,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    z: u8,
    rcode: u8,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16
}

pub struct Question {
    qname: String,
    qtype: String,
    qclass: String,
}

#[derive(Debug)]
pub struct Answer {
    header: Header,
    name: String,
    ty: String,
    class: String,
    ttl: u32,
    rdlength: u16,
    rddata: Vec<u8>
}

impl Message {
    pub fn read(data: Vec<u8>) -> Message {
        let (header, c) = Header::read(&data);
        for _ in 1..header.qdcount {
            let mut qs = Question::read(c);

        }

        Message {
            header,
            questions: vec![],
            answers: vec![]
        }

    }
}

impl Question {
    pub fn read(data: &Vec<u8>) -> (Question, &Vec<u8>) {
        Question {
            qname: "".to_string(),
            qtype: "".to_string(),
            qclass: "".to_string()
        }
    }
    pub fn new() -> Question {
        Question {
            qtype: "".to_string(),
            qclass: "".to_string(),
            qname: "".parse().unwrap()
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend(self.header.to_vec());
        vec.extend(name_to_vec(&self.qname));
        vec.extend(ty_to_u16(&self.qtype).to_be_bytes());
        vec.extend(class_to_u16(&self.qclass).to_be_bytes());
        vec
    }

    pub fn build(id: u16, url: &str, ty: &str) -> Question {
        let mut header = Header::new();
        header.id = id;
        header.rd = true;
        header.qdcount = 1;

        let mut question = Question::new();
        question.qname = url.parse().unwrap();
        question.qtype = ty.parse().unwrap();
        question.qclass = "IN".to_string();
        question
    }

}

impl Header {
    pub fn read(data: &Vec<u8>) -> (Header, &Vec<u8>) {
        let mut c = Cursor::new(data);
        let mut header = Header::new();
        header.id = c.read_u16::<BigEndian>().unwrap();
        header.parse_opts(c.read_u16::<BigEndian>().unwrap());
        header.qdcount = c.read_u16::<BigEndian>().unwrap();
        header.ancount = c.read_u16::<BigEndian>().unwrap();
        header.nscount = c.read_u16::<BigEndian>().unwrap();
        header.arcount = c.read_u16::<BigEndian>().unwrap();
        header.rcode = 0;
        (header, &data[c.position() as usize..].to_vec())
    }
    fn parse_opts(&mut self, data: u16) {
        self.qr = get_u1(data, _QR);
        self.opcode = get_u4(data, _OP);
        self.aa = get_u1(data, _AA);
        self.tc = get_u1(data, _TC);
        self.rd = get_u1(data, _RD);
        self.ra = get_u1(data, _RA);
        self.z = get_u4(data, _Z);
        self.rcode = get_u4(data, _CD);
    }

    pub fn new() -> Header {
        Header {
            id: 0,
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&self.id.to_be_bytes());

        let mut opts: u16 = 0;
        set_u1(&mut opts, self.qr, _QR);
        set_u4(&mut opts, self.opcode, _OP);
        set_u1(&mut opts, self.aa, _AA);
        set_u1(&mut opts, self.tc, _TC);
        set_u1(&mut opts, self.rd, _RD);
        set_u1(&mut opts, self.ra, _RA);
        // _CD might be wrong?
        set_u4(&mut opts, self.rcode, _CD);
        vec.extend_from_slice(&opts.to_be_bytes());

        vec.extend_from_slice(&self.qdcount.to_be_bytes());
        vec.extend_from_slice(&self.ancount.to_be_bytes());
        vec.extend_from_slice(&self.nscount.to_be_bytes());
        vec.extend_from_slice(&self.arcount.to_be_bytes());
        vec
    }
}

impl Answer {
    pub fn read(mut data: &Vec<u8>) -> (Answer, &Vec<u8>) {
        let (header, data) = Header::read(&data);
        let mut c = Cursor::new(data);
        let mut name = "".to_string();
        loop  {
            let size: usize = c.read_u8().unwrap() as usize;
            if size == 0 {
                break
            }
            if name.len() != 0 {
                name.push('.');
            }
            let mut buf = vec![0u8;size];
            c.read_exact(&mut buf).unwrap();
            name.push_str(&String::from_utf8(buf).expect("can't parse to utf8"));
        }

        let mut ans = Answer {
            header,
            name,
            ty: u16_to_ty(c.read_u16::<BigEndian>().unwrap()),
            class: u16_to_class(c.read_u16::<BigEndian>().unwrap()),
            ttl: c.read_u32::<BigEndian>().unwrap(),
            rdlength:  c.read_u16::<BigEndian>().unwrap(),
            rddata: vec![]
        };
        let mut buf = vec![0u8; ans.rdlength as usize];
        c.read_exact(&mut buf).unwrap();
        ans.rddata = buf;
        (ans, &data[c.position() as usize..].to_vec())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut vec = vec!();
        vec.extend(&self.header.to_vec());
        vec.extend(name_to_vec(&self.name));
        vec.extend(ty_to_u16(&self.ty).to_be_bytes());
        vec.extend(class_to_u16(&self.class).to_be_bytes());
        vec.extend(&self.ttl.to_be_bytes());
        vec.extend(&self.rdlength.to_be_bytes());
        vec.extend(&self.rddata);
        vec
    }
}

fn set_u1(data: &mut u16, value: bool, offset: u16) {
    if value {
        *data |= offset;
    } else {
        *data &= !offset
    };
}

pub fn set_u4(data: &mut u16, value: u8, start_offset: u16) {
    set_u1(data, (1 & value) > 0, start_offset );
    set_u1(data, (1 << 1 & value) > 0, start_offset >> 1 );
    set_u1(data, (1 << 2 & value) > 0, start_offset >> 2);
    set_u1(data, (1 << 3 & value) > 0, start_offset >> 3);
}

fn get_u1(data: u16, offset: u16) -> bool {
    data & offset > 1
}

// No idea if this works
fn get_u4(data: u16, offset: u16) -> u8 {
    let mut rsp: u8 = 0;
    rsp += if offset & data > 0 {1} else {0};
    rsp <<= 1;
    rsp += if offset >> 1 & data > 0 {1} else {0};
    rsp <<= 1;
    rsp += if offset >> 2 & data > 0 {1} else {0};
    rsp <<= 1;
    rsp += if offset >> 3 & data > 0 {1} else {0};
    rsp << 1
}
fn u16_to_ty(value: u16) -> String {
    match value {
        0x0001 =>  "A".to_string(),
        _ => panic!("invalid type")
    }

}
fn u16_to_class(value: u16) -> String {
    match value {
         0x0001 => "IN".to_string(),
        _ => panic!("invalid class")
    }
}
fn ty_to_u16(value: &str) -> u16 {
    match value {
        "A" =>  0x0001,
        _ => panic!("invalid type")
    }
}
fn class_to_u16(value: &str) -> u16 {
    match value {
        "IN" =>  0x0001,
        _ => panic!("invalid class")
    }
}
fn name_to_vec(value: &str) -> Vec<u8> {
    let split = value.split(".");
    let mut data = vec![];
    for s in split {
        data.push(s.len().to_be_bytes()[7]);
        data.extend_from_slice(s.as_bytes());
    }
    data.push(0);
    data
}