use nom::bits::complete::take;
use bitvec::prelude::{BitVec, Msb0};
use nom::IResult;

pub mod pkt;
pub mod header;
pub mod question;
pub mod message;
pub mod answer;

pub trait Serializable {
    // Takes an Object and serializes the data into `data`
    fn serialize(&self, data: &mut BitVec<u8, Msb0>);

    // takes data and generates an Object, consuming the buffer
    //fn deserialize(data: NBitSlice) -> IResult<NBitSlice, Self>;
}

pub type NBitSlice<'a> = (&'a [u8], usize);

pub fn take_u1(data: NBitSlice) -> IResult<NBitSlice, bool> {
    let (res, b):  (NBitSlice, u8) = take(1u8)(data)?;
    Ok((res, if b > 0 {true} else {false}))
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
    take(16u8)(data)
}
