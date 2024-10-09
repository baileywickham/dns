use std::fmt;
use std::fmt::Formatter;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use nom::IResult;
use crate::pkt::question::{Qclass, Qtype};
use crate::pkt::{NBitSlice, parse_name, Serializable, take_bytes, take_u16, take_u32};

pub struct Answer {
    name: String,
    ty: Qtype,
    class: Qclass,
    ttl: u32,
    rdlength: u16,
    rddata: Vec<u8>,
    parsed_data: String
}

impl Serializable for Answer {
    fn serialize(&self, _data: &mut BitVec<u8, Msb0>) {
        todo!()
    }
}


impl Answer {
    pub fn deserialize<'a>(data: (&'a [u8], usize), raw_data: &[u8]) -> IResult<NBitSlice<'a>, Answer> {
        let (data, name) = parse_name(data, raw_data).unwrap();
        let (data, ty) = Qtype::deserialize(data).unwrap();
        let (data, class) = Qclass::deserialize(data).unwrap();
        let (data, ttl) = take_u32(data).unwrap();
        let (data, rdlength) = take_u16(data).unwrap();
        let (data, rddata) = take_bytes(data, rdlength as usize).unwrap();
        let mut ans = Answer {
            name,
            ty,
            class,
            ttl,
            rdlength,
            rddata,
            parsed_data: "".to_string()
        };
        ans.parse_record(raw_data);
        Ok((data, ans))
    }

    fn parse_record(&mut self, raw_data: &[u8]) {
        self.parsed_data =
            match self.ty {
            Qtype::A => { format! {"{}.{}.{}.{}", self.rddata[0], self.rddata[1], self.rddata[2], self.rddata[3]} }
            Qtype::CNAME => {
                let (_, name) = parse_name((&self.rddata, 0), raw_data).unwrap();
                name
            }
        };
    }
}


impl fmt::Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\t{}\t{}\t{}\t{:?}", self.name, self.ty, self.class, self.ttl, self.parsed_data)
    }
}
