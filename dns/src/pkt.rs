use nom::bits::complete::take;
use bitvec::prelude::{BitVec, Msb0};
use nom::combinator::peek;
use nom::{IResult};
use nom::multi::count;

pub mod pkt;
pub mod header;
pub mod question;
pub mod message;
pub mod answer;

const PTR_OFFSET: u8 = 0b11000000;

pub trait Serializable {
    // Takes an Object and serializes the data into `data`
    fn serialize(&self, data: &mut BitVec<u8, Msb0>);

    // takes data and generates an Object, consuming the buffer
    //fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Self>;
}

pub type NBitSlice<'a> = (&'a [u8], usize);

pub fn take_u1(data: NBitSlice) -> IResult<NBitSlice, bool> {
    let (res, b): (NBitSlice, u8) = take(1u8)(data)?;
    Ok((res, if b > 0 { true } else { false }))
}

pub fn take_u4(data: NBitSlice) -> IResult<NBitSlice, u8> {
    take(4u8)(data)
}

pub fn take_u8(data: NBitSlice) -> IResult<NBitSlice, u8> {
    take(8u8)(data)
}

pub fn take_u3(data: NBitSlice) -> IResult<NBitSlice, u8> {
    take(3u8)(data)
}

pub fn take_u16(data: NBitSlice) -> IResult<NBitSlice, u16> {
    take(16u16)(data)
}

pub fn take_u32(data: NBitSlice) -> IResult<NBitSlice, u32> {
    take(32u8)(data)
}

pub fn take_bytes(data: NBitSlice, bytes: usize) -> IResult<NBitSlice, Vec<u8>> {
    count(take(8u8), bytes)(data)
}

pub fn parse_name<'a>(mut data: (&'a [u8], usize), raw_data: &[u8]) -> IResult<NBitSlice<'a>, String> {
    let mut name = String::new();
    loop {
        // A label can end with a ptr, recheck the ptr math every loop
        let (_, first_byte): (NBitSlice, u8) = peek(take(8u8))(data)?;
        if first_byte & PTR_OFFSET > 0 {
            let (rem, ptr) = take_u16(data).unwrap();
            let next: NBitSlice = (&raw_data[get_deref_ptr(ptr)..], 0);
            let (_, part) = parse_name(next, raw_data).unwrap();
            data = rem;
            name += &part;
            break;
        } else {
            let (rem, size) = take_u8(data).unwrap();
            data = rem;
            if size == 0 {
                break;
            }
            name.push('.');

            let (rem, buf) = take_bytes(data, size as usize)?;
            data = rem;
            name.push_str(&String::from_utf8(buf).expect("can't parse to utf8"));
        }
    }
    Ok((data, name))
}


fn get_deref_ptr(ptr: u16) -> usize {
    (ptr - ((PTR_OFFSET as u16) << 8)) as usize
}

