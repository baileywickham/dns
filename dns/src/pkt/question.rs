use std::fmt;
use std::fmt::Formatter;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use bitvec::view::{BitView};
use nom::IResult;
use crate::pkt::{NBitSlice, parse_name, Serializable, take_u16};
use strum_macros::{EnumString,Display};


pub struct Question {
    pub(crate) qname: String,
    pub(crate) qtype: Qtype,
    pub(crate) qclass: Qclass,
}

#[derive(Debug, EnumString, Display)]
pub enum Qtype {
    #[strum(ascii_case_insensitive)]
    A,
    #[strum(ascii_case_insensitive)]
    CNAME
}

impl Qtype {
    pub(crate) fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Self> {
        let (data, qtype) = take_u16(data).unwrap();
        match qtype {
            0x0001 =>  Ok((data, Qtype::A)),
            0x0005 => Ok((data, Qtype::CNAME)),
            _ => {panic!("error parsing qtype: {}", qtype)}
        }
    }
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        match self {
            Qtype::A => {data.extend_from_bitslice(1u16.view_bits::<Msb0>())}
            Qtype::CNAME => {data.extend_from_bitslice(5u16.view_bits::<Msb0>())}
        }
    }
}

#[derive(Debug, EnumString, Display)]
pub enum Qclass {
    #[strum(ascii_case_insensitive)]
    IN
}

impl Qclass {
    pub(crate) fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Self> {
        let (data, qtype) = take_u16(data).unwrap();
        match qtype {
            0x0001 =>  Ok((data, Qclass::IN)),
            _ => {panic!("error parsing qclass")}
        }
    }
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        match self {
            Qclass::IN => {data.extend_from_bitslice(1u16.view_bits::<Msb0>())}
        }
    }
}

impl Serializable for Question {
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        data.extend(name_to_vec(&self.qname));
        self.qtype.serialize(data);
        self.qclass.serialize(data);
    }
}

impl Question {
    pub fn deserialize<'a>(data: (&'a [u8], usize), raw_data: &[u8]) -> IResult<NBitSlice<'a>, Self> {
        let (data, qname) = parse_name(data, raw_data).unwrap();
        let (data, qtype) = Qtype::deserialize(data).unwrap();
        let (data, qclass) = Qclass::deserialize(data).unwrap();
        Ok((data, Question {
            qname,
            qtype,
            qclass,
        }))
    }
    pub fn new() -> Question {
        Question {
            qname: "".to_string(),
            qtype: Qtype::A,
            qclass: Qclass::IN
        }
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

impl fmt::Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\t{}\t{}", self.qname, self.qtype, self.qclass)
    }
}