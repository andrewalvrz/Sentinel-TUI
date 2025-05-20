// This file was automatically generated, do not edit
use crate::MavlinkVersion;
#[allow(unused_imports)]
use crate::{};
use crate::{error::*, Message};
#[cfg(not(feature = "std"))]
use alloc::string::ToString;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[allow(unused_imports)]
use bitflags::bitflags;
#[allow(unused_imports)]
use bytes::{Buf, BufMut, Bytes, BytesMut};
#[allow(unused_imports)]
use num_derive::FromPrimitive;
#[allow(unused_imports)]
use num_derive::ToPrimitive;
#[allow(unused_imports)]
use num_traits::FromPrimitive;
#[allow(unused_imports)]
use num_traits::ToPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TEST_TYPES_DATA {
    pub u64: u64,
    pub s64: i64,
    pub d: f64,
    pub u64_array: [u64; 3],
    pub s64_array: [i64; 3],
    pub d_array: [f64; 3],
    pub u32: u32,
    pub s32: i32,
    pub f: f32,
    pub u32_array: [u32; 3],
    pub s32_array: [i32; 3],
    pub f_array: [f32; 3],
    pub u16: u16,
    pub s16: i16,
    pub u16_array: [u16; 3],
    pub s16_array: [i16; 3],
    pub c: char,
    pub s: [char; 10],
    pub u8: u8,
    pub s8: i8,
    pub u8_array: [u8; 3],
    pub s8_array: [i8; 3],
}
impl TEST_TYPES_DATA {
    pub const ENCODED_LEN: usize = 179usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TEST_TYPES_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TEST_TYPES_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.u64 = buf.get_u64_le();
        _struct.s64 = buf.get_i64_le();
        _struct.d = buf.get_f64_le();
        for idx in 0..3usize {
            let val = buf.get_u64_le();
            _struct.u64_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_i64_le();
            _struct.s64_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f64_le();
            _struct.d_array[idx] = val;
        }
        _struct.u32 = buf.get_u32_le();
        _struct.s32 = buf.get_i32_le();
        _struct.f = buf.get_f32_le();
        for idx in 0..3usize {
            let val = buf.get_u32_le();
            _struct.u32_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_i32_le();
            _struct.s32_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.f_array[idx] = val;
        }
        _struct.u16 = buf.get_u16_le();
        _struct.s16 = buf.get_i16_le();
        for idx in 0..3usize {
            let val = buf.get_u16_le();
            _struct.u16_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_i16_le();
            _struct.s16_array[idx] = val;
        }
        _struct.c = buf.get_u8() as char;
        for idx in 0..10usize {
            let val = buf.get_u8() as char;
            _struct.s[idx] = val;
        }
        _struct.u8 = buf.get_u8();
        _struct.s8 = buf.get_i8();
        for idx in 0..3usize {
            let val = buf.get_u8();
            _struct.u8_array[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_i8();
            _struct.s8_array[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.u64);
        _tmp.put_i64_le(self.s64);
        _tmp.put_f64_le(self.d);
        for val in &self.u64_array {
            _tmp.put_u64_le(*val);
        }
        for val in &self.s64_array {
            _tmp.put_i64_le(*val);
        }
        for val in &self.d_array {
            _tmp.put_f64_le(*val);
        }
        _tmp.put_u32_le(self.u32);
        _tmp.put_i32_le(self.s32);
        _tmp.put_f32_le(self.f);
        for val in &self.u32_array {
            _tmp.put_u32_le(*val);
        }
        for val in &self.s32_array {
            _tmp.put_i32_le(*val);
        }
        for val in &self.f_array {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u16_le(self.u16);
        _tmp.put_i16_le(self.s16);
        for val in &self.u16_array {
            _tmp.put_u16_le(*val);
        }
        for val in &self.s16_array {
            _tmp.put_i16_le(*val);
        }
        _tmp.put_u8(self.c as u8);
        for val in &self.s {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.u8);
        _tmp.put_i8(self.s8);
        for val in &self.u8_array {
            _tmp.put_u8(*val);
        }
        for val in &self.s8_array {
            _tmp.put_i8(*val);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    TEST_TYPES(TEST_TYPES_DATA),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => TEST_TYPES_DATA::deser(version, payload).map(|s| MavMessage::TEST_TYPES(s)),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::TEST_TYPES(..) => "TEST_TYPES",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::TEST_TYPES(..) => 0,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "TEST_TYPES" => Ok(0),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::TEST_TYPES(TEST_TYPES_DATA::default())),
            _ => {
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::TEST_TYPES(ref body) => body.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 103,
            _ => 0,
        }
    }
}
