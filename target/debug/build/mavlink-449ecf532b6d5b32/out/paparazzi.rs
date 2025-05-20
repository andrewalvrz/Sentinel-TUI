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
pub struct SCRIPT_ITEM_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub name: Vec<char>, /* 50 elements */
}
impl SCRIPT_ITEM_DATA {
    pub const ENCODED_LEN: usize = 54usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCRIPT_ITEM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCRIPT_ITEM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.seq = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..50usize {
            let val = buf.get_u8() as char;
            _struct.name.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCRIPT_REQUEST_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl SCRIPT_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCRIPT_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCRIPT_REQUEST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.seq = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCRIPT_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl SCRIPT_REQUEST_LIST_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCRIPT_REQUEST_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCRIPT_REQUEST_LIST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCRIPT_COUNT_DATA {
    pub count: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl SCRIPT_COUNT_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCRIPT_COUNT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCRIPT_COUNT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.count = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.count);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCRIPT_CURRENT_DATA {
    pub seq: u16,
}
impl SCRIPT_CURRENT_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCRIPT_CURRENT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCRIPT_CURRENT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.seq = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seq);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    SCRIPT_ITEM(SCRIPT_ITEM_DATA),
    SCRIPT_REQUEST(SCRIPT_REQUEST_DATA),
    SCRIPT_REQUEST_LIST(SCRIPT_REQUEST_LIST_DATA),
    SCRIPT_COUNT(SCRIPT_COUNT_DATA),
    SCRIPT_CURRENT(SCRIPT_CURRENT_DATA),
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
            180 => SCRIPT_ITEM_DATA::deser(version, payload).map(|s| MavMessage::SCRIPT_ITEM(s)),
            181 => {
                SCRIPT_REQUEST_DATA::deser(version, payload).map(|s| MavMessage::SCRIPT_REQUEST(s))
            }
            182 => SCRIPT_REQUEST_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::SCRIPT_REQUEST_LIST(s)),
            183 => SCRIPT_COUNT_DATA::deser(version, payload).map(|s| MavMessage::SCRIPT_COUNT(s)),
            184 => {
                SCRIPT_CURRENT_DATA::deser(version, payload).map(|s| MavMessage::SCRIPT_CURRENT(s))
            }
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
            MavMessage::SCRIPT_ITEM(..) => "SCRIPT_ITEM",
            MavMessage::SCRIPT_REQUEST(..) => "SCRIPT_REQUEST",
            MavMessage::SCRIPT_REQUEST_LIST(..) => "SCRIPT_REQUEST_LIST",
            MavMessage::SCRIPT_COUNT(..) => "SCRIPT_COUNT",
            MavMessage::SCRIPT_CURRENT(..) => "SCRIPT_CURRENT",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::SCRIPT_ITEM(..) => 180,
            MavMessage::SCRIPT_REQUEST(..) => 181,
            MavMessage::SCRIPT_REQUEST_LIST(..) => 182,
            MavMessage::SCRIPT_COUNT(..) => 183,
            MavMessage::SCRIPT_CURRENT(..) => 184,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "SCRIPT_ITEM" => Ok(180),
            "SCRIPT_REQUEST" => Ok(181),
            "SCRIPT_REQUEST_LIST" => Ok(182),
            "SCRIPT_COUNT" => Ok(183),
            "SCRIPT_CURRENT" => Ok(184),
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
            180 => Ok(MavMessage::SCRIPT_ITEM(SCRIPT_ITEM_DATA::default())),
            181 => Ok(MavMessage::SCRIPT_REQUEST(SCRIPT_REQUEST_DATA::default())),
            182 => Ok(MavMessage::SCRIPT_REQUEST_LIST(
                SCRIPT_REQUEST_LIST_DATA::default(),
            )),
            183 => Ok(MavMessage::SCRIPT_COUNT(SCRIPT_COUNT_DATA::default())),
            184 => Ok(MavMessage::SCRIPT_CURRENT(SCRIPT_CURRENT_DATA::default())),
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
            &MavMessage::SCRIPT_ITEM(ref body) => body.ser(),
            &MavMessage::SCRIPT_REQUEST(ref body) => body.ser(),
            &MavMessage::SCRIPT_REQUEST_LIST(ref body) => body.ser(),
            &MavMessage::SCRIPT_COUNT(ref body) => body.ser(),
            &MavMessage::SCRIPT_CURRENT(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            180 => 231,
            181 => 129,
            182 => 115,
            183 => 186,
            184 => 40,
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
