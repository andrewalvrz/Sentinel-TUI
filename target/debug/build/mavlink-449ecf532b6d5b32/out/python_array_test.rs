// This file was automatically generated, do not edit
#[allow(unused_imports)]
use crate::common::*;
use crate::MavlinkVersion;
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
pub struct ARRAY_TEST_0_DATA {
    pub ar_u32: [u32; 4],
    pub ar_u16: [u16; 4],
    pub v1: u8,
    pub ar_i8: [i8; 4],
    pub ar_u8: [u8; 4],
}
impl ARRAY_TEST_0_DATA {
    pub const ENCODED_LEN: usize = 33usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_0_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_0_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.ar_u16[idx] = val;
        }
        _struct.v1 = buf.get_u8();
        for idx in 0..4usize {
            let val = buf.get_i8();
            _struct.ar_i8[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.ar_u8[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val);
        }
        _tmp.put_u8(self.v1);
        for val in &self.ar_i8 {
            _tmp.put_i8(*val);
        }
        for val in &self.ar_u8 {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_1_DATA {
    pub ar_u32: [u32; 4],
}
impl ARRAY_TEST_1_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_1_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_1_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_3_DATA {
    pub ar_u32: [u32; 4],
    pub v: u8,
}
impl ARRAY_TEST_3_DATA {
    pub const ENCODED_LEN: usize = 17usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_3_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_3_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        _struct.v = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        _tmp.put_u8(self.v);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_4_DATA {
    pub ar_u32: [u32; 4],
    pub v: u8,
}
impl ARRAY_TEST_4_DATA {
    pub const ENCODED_LEN: usize = 17usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_4_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_4_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        _struct.v = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        _tmp.put_u8(self.v);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_5_DATA {
    pub c1: [char; 5],
    pub c2: [char; 5],
}
impl ARRAY_TEST_5_DATA {
    pub const ENCODED_LEN: usize = 10usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_5_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_5_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..5usize {
            let val = buf.get_u8() as char;
            _struct.c1[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_u8() as char;
            _struct.c2[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.c1 {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.c2 {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_6_DATA {
    pub ar_d: [f64; 2],
    pub v3: u32,
    pub ar_u32: [u32; 2],
    pub ar_i32: [i32; 2],
    pub ar_f: [f32; 2],
    pub v2: u16,
    pub ar_u16: [u16; 2],
    pub ar_i16: [i16; 2],
    pub v1: u8,
    pub ar_u8: [u8; 2],
    pub ar_i8: [i8; 2],
    pub ar_c: [char; 32],
}
impl ARRAY_TEST_6_DATA {
    pub const ENCODED_LEN: usize = 91usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_6_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_6_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..2usize {
            let val = buf.get_f64_le();
            _struct.ar_d[idx] = val;
        }
        _struct.v3 = buf.get_u32_le();
        for idx in 0..2usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i32_le();
            _struct.ar_i32[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_f32_le();
            _struct.ar_f[idx] = val;
        }
        _struct.v2 = buf.get_u16_le();
        for idx in 0..2usize {
            let val = buf.get_u16_le();
            _struct.ar_u16[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i16_le();
            _struct.ar_i16[idx] = val;
        }
        _struct.v1 = buf.get_u8();
        for idx in 0..2usize {
            let val = buf.get_u8();
            _struct.ar_u8[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i8();
            _struct.ar_i8[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.ar_c[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val);
        }
        _tmp.put_u32_le(self.v3);
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        for val in &self.ar_i32 {
            _tmp.put_i32_le(*val);
        }
        for val in &self.ar_f {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u16_le(self.v2);
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val);
        }
        for val in &self.ar_i16 {
            _tmp.put_i16_le(*val);
        }
        _tmp.put_u8(self.v1);
        for val in &self.ar_u8 {
            _tmp.put_u8(*val);
        }
        for val in &self.ar_i8 {
            _tmp.put_i8(*val);
        }
        for val in &self.ar_c {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_7_DATA {
    pub ar_d: [f64; 2],
    pub ar_f: [f32; 2],
    pub ar_u32: [u32; 2],
    pub ar_i32: [i32; 2],
    pub ar_u16: [u16; 2],
    pub ar_i16: [i16; 2],
    pub ar_u8: [u8; 2],
    pub ar_i8: [i8; 2],
    pub ar_c: [char; 32],
}
impl ARRAY_TEST_7_DATA {
    pub const ENCODED_LEN: usize = 84usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_7_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_7_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..2usize {
            let val = buf.get_f64_le();
            _struct.ar_d[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_f32_le();
            _struct.ar_f[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_u32_le();
            _struct.ar_u32[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i32_le();
            _struct.ar_i32[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_u16_le();
            _struct.ar_u16[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i16_le();
            _struct.ar_i16[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_u8();
            _struct.ar_u8[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_i8();
            _struct.ar_i8[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.ar_c[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val);
        }
        for val in &self.ar_f {
            _tmp.put_f32_le(*val);
        }
        for val in &self.ar_u32 {
            _tmp.put_u32_le(*val);
        }
        for val in &self.ar_i32 {
            _tmp.put_i32_le(*val);
        }
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val);
        }
        for val in &self.ar_i16 {
            _tmp.put_i16_le(*val);
        }
        for val in &self.ar_u8 {
            _tmp.put_u8(*val);
        }
        for val in &self.ar_i8 {
            _tmp.put_i8(*val);
        }
        for val in &self.ar_c {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ARRAY_TEST_8_DATA {
    pub ar_d: [f64; 2],
    pub v3: u32,
    pub ar_u16: [u16; 2],
}
impl ARRAY_TEST_8_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ARRAY_TEST_8_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ARRAY_TEST_8_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..2usize {
            let val = buf.get_f64_le();
            _struct.ar_d[idx] = val;
        }
        _struct.v3 = buf.get_u32_le();
        for idx in 0..2usize {
            let val = buf.get_u16_le();
            _struct.ar_u16[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ar_d {
            _tmp.put_f64_le(*val);
        }
        _tmp.put_u32_le(self.v3);
        for val in &self.ar_u16 {
            _tmp.put_u16_le(*val);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    ARRAY_TEST_0(ARRAY_TEST_0_DATA),
    ARRAY_TEST_1(ARRAY_TEST_1_DATA),
    ARRAY_TEST_3(ARRAY_TEST_3_DATA),
    ARRAY_TEST_4(ARRAY_TEST_4_DATA),
    ARRAY_TEST_5(ARRAY_TEST_5_DATA),
    ARRAY_TEST_6(ARRAY_TEST_6_DATA),
    ARRAY_TEST_7(ARRAY_TEST_7_DATA),
    ARRAY_TEST_8(ARRAY_TEST_8_DATA),
    common(crate::common::MavMessage),
}
impl From<crate::common::MavMessage> for MavMessage {
    fn from(message: crate::common::MavMessage) -> Self {
        MavMessage::common(message)
    }
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            150 => ARRAY_TEST_0_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_0(s)),
            151 => ARRAY_TEST_1_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_1(s)),
            153 => ARRAY_TEST_3_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_3(s)),
            154 => ARRAY_TEST_4_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_4(s)),
            155 => ARRAY_TEST_5_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_5(s)),
            156 => ARRAY_TEST_6_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_6(s)),
            157 => ARRAY_TEST_7_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_7(s)),
            158 => ARRAY_TEST_8_DATA::deser(version, payload).map(|s| MavMessage::ARRAY_TEST_8(s)),
            _ => {
                if let Ok(msg) = crate::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::common(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::ARRAY_TEST_0(..) => "ARRAY_TEST_0",
            MavMessage::ARRAY_TEST_1(..) => "ARRAY_TEST_1",
            MavMessage::ARRAY_TEST_3(..) => "ARRAY_TEST_3",
            MavMessage::ARRAY_TEST_4(..) => "ARRAY_TEST_4",
            MavMessage::ARRAY_TEST_5(..) => "ARRAY_TEST_5",
            MavMessage::ARRAY_TEST_6(..) => "ARRAY_TEST_6",
            MavMessage::ARRAY_TEST_7(..) => "ARRAY_TEST_7",
            MavMessage::ARRAY_TEST_8(..) => "ARRAY_TEST_8",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::ARRAY_TEST_0(..) => 150,
            MavMessage::ARRAY_TEST_1(..) => 151,
            MavMessage::ARRAY_TEST_3(..) => 153,
            MavMessage::ARRAY_TEST_4(..) => 154,
            MavMessage::ARRAY_TEST_5(..) => 155,
            MavMessage::ARRAY_TEST_6(..) => 156,
            MavMessage::ARRAY_TEST_7(..) => 157,
            MavMessage::ARRAY_TEST_8(..) => 158,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "ARRAY_TEST_0" => Ok(150),
            "ARRAY_TEST_1" => Ok(151),
            "ARRAY_TEST_3" => Ok(153),
            "ARRAY_TEST_4" => Ok(154),
            "ARRAY_TEST_5" => Ok(155),
            "ARRAY_TEST_6" => Ok(156),
            "ARRAY_TEST_7" => Ok(157),
            "ARRAY_TEST_8" => Ok(158),
            _ => {
                match crate::common::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                Err("Invalid message name.")
            }
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            150 => Ok(MavMessage::ARRAY_TEST_0(ARRAY_TEST_0_DATA::default())),
            151 => Ok(MavMessage::ARRAY_TEST_1(ARRAY_TEST_1_DATA::default())),
            153 => Ok(MavMessage::ARRAY_TEST_3(ARRAY_TEST_3_DATA::default())),
            154 => Ok(MavMessage::ARRAY_TEST_4(ARRAY_TEST_4_DATA::default())),
            155 => Ok(MavMessage::ARRAY_TEST_5(ARRAY_TEST_5_DATA::default())),
            156 => Ok(MavMessage::ARRAY_TEST_6(ARRAY_TEST_6_DATA::default())),
            157 => Ok(MavMessage::ARRAY_TEST_7(ARRAY_TEST_7_DATA::default())),
            158 => Ok(MavMessage::ARRAY_TEST_8(ARRAY_TEST_8_DATA::default())),
            _ => {
                if let Ok(msg) = crate::common::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::common(msg));
                }
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::ARRAY_TEST_0(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_1(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_3(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_4(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_5(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_6(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_7(ref body) => body.ser(),
            &MavMessage::ARRAY_TEST_8(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 26,
            151 => 72,
            153 => 19,
            154 => 89,
            155 => 27,
            156 => 14,
            157 => 187,
            158 => 106,
            _ => {
                match crate::common::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                0
            }
        }
    }
}
