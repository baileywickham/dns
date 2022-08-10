use std::fmt;
use std::fmt::Formatter;
use crate::pkt::{NBitSlice, Serializable, take_u1, take_u16, take_u3, take_u4};
use bitvec::prelude::*;
use nom::IResult;
use crate::pkt::header::Opcode::{IQuery, Query, Status};
use crate::pkt::header::Rcode::{FormatError, NameError, NoError, NotImplemented, Refused};
use strum_macros::{EnumString,Display};

pub(crate) struct Header {
    pub(crate) id: u16,
    qr: bool,
    opcode: Opcode, // u4
    aa: bool,
    tc: bool,
    pub(crate) rd: bool,
    ra: bool,
    z: u8,
    rcode: Rcode,
    pub(crate) qdcount: u16,
    pub(crate) ancount: u16,
    nscount: u16,
    arcount: u16
}

#[derive(Debug, EnumString, Display)]
enum Opcode {
    Query,
    IQuery,
    Status
}

#[derive(Debug, EnumString, Display)]
enum Rcode {
    NoError,
    FormatError,
    NameError,
    NotImplemented,
    Refused
}

impl Serializable for Opcode {
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        match self {
            Opcode::Query => {
                data.extend(&0u8.view_bits::<Msb0>()[..4]);
            }
            Opcode::IQuery => {
                data.extend(&1u8.view_bits::<Msb0>()[..4]);
            }
            Opcode::Status => {
                data.extend(&2u8.view_bits::<Msb0>()[..4]);
            }
        }
    }
}
impl Opcode {
    fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Opcode>{
        let (data, code) = take_u4(data).unwrap();
        match code {
            0 => Ok((data, Query)),
            1 => Ok((data, IQuery)),
            2 => Ok((data, Status)),
            _ => {panic!{"Unsupported rcode"}},
        }
    }
}

impl Serializable for Rcode {
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        match self {
            Rcode::NoError => {
                data.extend(&0u8.view_bits::<Msb0>()[..4]);
            }
            Rcode::FormatError => {
                data.extend(&1u8.view_bits::<Msb0>()[..4]);
            }
            Rcode::NameError => {
                data.extend(&2u8.view_bits::<Msb0>()[..4]);
            }
            Rcode::NotImplemented => {
                data.extend(&3u8.view_bits::<Msb0>()[..4]);
            }
            Rcode::Refused => {
                data.extend(&4u8.view_bits::<Msb0>()[..4]);
            }
        }
    }
}
impl Rcode {
    fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Self> {
        let (res, code) = take_u4(data).unwrap();
        match code {
            0 => Ok((res, NoError)),
            1 => Ok((res, FormatError)),
            2 => Ok((res, NameError)),
            3 => Ok((res, NotImplemented)),
            4 => Ok((res, Refused)),
            _ => {panic!{"Unsupported rcode"}},
        }
    }
}

impl Serializable for Header {
    fn serialize(&self, data: &mut BitVec<u8, Msb0>) {
        data.extend_from_bitslice(self.id.view_bits::<Msb0>());
        data.push(self.qr);
        self.opcode.serialize(data);
        data.push(self.aa);
        data.push(self.tc);
        data.push(self.rd);
        data.push(self.ra);
        data.extend(&self.z.view_bits::<Msb0>()[..3]);
        self.rcode.serialize(data);
        data.extend(self.qdcount.view_bits::<Msb0>());
        data.extend(self.ancount.view_bits::<Msb0>());
        data.extend(self.nscount.view_bits::<Msb0>());
        data.extend(self.arcount.view_bits::<Msb0>());
    }
}

impl Header {
    pub fn new() -> Header {
        Header {
            id: 0,
            qr: false,
            opcode: Opcode::Query,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: Rcode::NoError,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0
        }
    }
    pub fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Header> {
        let (rem, id) = take_u16(data).unwrap();
        let (rem, qr) = take_u1(rem).unwrap();
        let (rem, opcode) = Opcode::deserialize(rem).unwrap();
        let (rem, aa) = take_u1(rem).unwrap();
        let (rem, tc) = take_u1(rem).unwrap();
        let (rem, rd) = take_u1(rem).unwrap();
        let (rem, ra) = take_u1(rem).unwrap();
        let (rem, z) = take_u3(rem).unwrap();
        let (rem, rcode) = Rcode::deserialize(rem).unwrap();
        let (rem, qdcount) = take_u16(rem).unwrap();
        let (rem, ancount) = take_u16(rem).unwrap();
        let (rem, nscount) = take_u16(rem).unwrap();
        let (rem, arcount) = take_u16(rem).unwrap();
        Ok((rem, Header {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount
        }))
    }
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Header")?;
        writeln!(f, "ID\tType\tResponse")?;
        writeln!(f, "{}\t{}\t{}", self.id, self.opcode, self.rcode)
    }
}