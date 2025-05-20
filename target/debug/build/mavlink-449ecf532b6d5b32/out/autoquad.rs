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
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AutoquadMavlinkDefsVersion {
    AQ_MAVLINK_DEFS_VERSION_1 = 0,
}
impl Default for AutoquadMavlinkDefsVersion {
    fn default() -> Self {
        AutoquadMavlinkDefsVersion::AQ_MAVLINK_DEFS_VERSION_1
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AutoquadNavStatus {
    AQ_NAV_STATUS_INIT = 0,
    AQ_NAV_STATUS_STANDBY = 1,
    AQ_NAV_STATUS_MANUAL = 2,
    AQ_NAV_STATUS_ALTHOLD = 4,
    AQ_NAV_STATUS_POSHOLD = 8,
    AQ_NAV_STATUS_GUIDED = 16,
    AQ_NAV_STATUS_MISSION = 32,
    AQ_NAV_STATUS_READY = 256,
    AQ_NAV_STATUS_CALIBRATING = 512,
    AQ_NAV_STATUS_NO_RC = 4096,
    AQ_NAV_STATUS_FUEL_LOW = 8192,
    AQ_NAV_STATUS_FUEL_CRITICAL = 16384,
    AQ_NAV_STATUS_DVH = 16777216,
    AQ_NAV_STATUS_DAO = 33554432,
    AQ_NAV_STATUS_CEILING_REACHED = 67108864,
    AQ_NAV_STATUS_CEILING = 134217728,
    AQ_NAV_STATUS_HF_DYNAMIC = 268435456,
    AQ_NAV_STATUS_HF_LOCKED = 536870912,
    AQ_NAV_STATUS_RTH = 1073741824,
    AQ_NAV_STATUS_FAILSAFE = 2147483648,
}
impl Default for AutoquadNavStatus {
    fn default() -> Self {
        AutoquadNavStatus::AQ_NAV_STATUS_INIT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmd {
    MAV_CMD_AQ_NAV_LEG_ORBIT = 1,
    MAV_CMD_AQ_TELEMETRY = 2,
    MAV_CMD_AQ_REQUEST_VERSION = 4,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_AQ_NAV_LEG_ORBIT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavDataStream {
    MAV_DATA_STREAM_PROPULSION = 0,
}
impl Default for MavDataStream {
    fn default() -> Self {
        MavDataStream::MAV_DATA_STREAM_PROPULSION
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AQ_TELEMETRY_F_DATA {
    pub value1: f32,
    pub value2: f32,
    pub value3: f32,
    pub value4: f32,
    pub value5: f32,
    pub value6: f32,
    pub value7: f32,
    pub value8: f32,
    pub value9: f32,
    pub value10: f32,
    pub value11: f32,
    pub value12: f32,
    pub value13: f32,
    pub value14: f32,
    pub value15: f32,
    pub value16: f32,
    pub value17: f32,
    pub value18: f32,
    pub value19: f32,
    pub value20: f32,
    pub Index: u16,
}
impl AQ_TELEMETRY_F_DATA {
    pub const ENCODED_LEN: usize = 82usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AQ_TELEMETRY_F_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AQ_TELEMETRY_F_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.value1 = buf.get_f32_le();
        _struct.value2 = buf.get_f32_le();
        _struct.value3 = buf.get_f32_le();
        _struct.value4 = buf.get_f32_le();
        _struct.value5 = buf.get_f32_le();
        _struct.value6 = buf.get_f32_le();
        _struct.value7 = buf.get_f32_le();
        _struct.value8 = buf.get_f32_le();
        _struct.value9 = buf.get_f32_le();
        _struct.value10 = buf.get_f32_le();
        _struct.value11 = buf.get_f32_le();
        _struct.value12 = buf.get_f32_le();
        _struct.value13 = buf.get_f32_le();
        _struct.value14 = buf.get_f32_le();
        _struct.value15 = buf.get_f32_le();
        _struct.value16 = buf.get_f32_le();
        _struct.value17 = buf.get_f32_le();
        _struct.value18 = buf.get_f32_le();
        _struct.value19 = buf.get_f32_le();
        _struct.value20 = buf.get_f32_le();
        _struct.Index = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.value1);
        _tmp.put_f32_le(self.value2);
        _tmp.put_f32_le(self.value3);
        _tmp.put_f32_le(self.value4);
        _tmp.put_f32_le(self.value5);
        _tmp.put_f32_le(self.value6);
        _tmp.put_f32_le(self.value7);
        _tmp.put_f32_le(self.value8);
        _tmp.put_f32_le(self.value9);
        _tmp.put_f32_le(self.value10);
        _tmp.put_f32_le(self.value11);
        _tmp.put_f32_le(self.value12);
        _tmp.put_f32_le(self.value13);
        _tmp.put_f32_le(self.value14);
        _tmp.put_f32_le(self.value15);
        _tmp.put_f32_le(self.value16);
        _tmp.put_f32_le(self.value17);
        _tmp.put_f32_le(self.value18);
        _tmp.put_f32_le(self.value19);
        _tmp.put_f32_le(self.value20);
        _tmp.put_u16_le(self.Index);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AQ_ESC_TELEMETRY_DATA {
    pub time_boot_ms: u32,
    pub data0: [u32; 4],
    pub data1: [u32; 4],
    pub status_age: [u16; 4],
    pub seq: u8,
    pub num_motors: u8,
    pub num_in_seq: u8,
    pub escid: [u8; 4],
    pub data_version: [u8; 4],
}
impl AQ_ESC_TELEMETRY_DATA {
    pub const ENCODED_LEN: usize = 55usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AQ_ESC_TELEMETRY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AQ_ESC_TELEMETRY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.data0[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.data1[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.status_age[idx] = val;
        }
        _struct.seq = buf.get_u8();
        _struct.num_motors = buf.get_u8();
        _struct.num_in_seq = buf.get_u8();
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.escid[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.data_version[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        for val in &self.data0 {
            _tmp.put_u32_le(*val);
        }
        for val in &self.data1 {
            _tmp.put_u32_le(*val);
        }
        for val in &self.status_age {
            _tmp.put_u16_le(*val);
        }
        _tmp.put_u8(self.seq);
        _tmp.put_u8(self.num_motors);
        _tmp.put_u8(self.num_in_seq);
        for val in &self.escid {
            _tmp.put_u8(*val);
        }
        for val in &self.data_version {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    AQ_TELEMETRY_F(AQ_TELEMETRY_F_DATA),
    AQ_ESC_TELEMETRY(AQ_ESC_TELEMETRY_DATA),
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
            150 => {
                AQ_TELEMETRY_F_DATA::deser(version, payload).map(|s| MavMessage::AQ_TELEMETRY_F(s))
            }
            152 => AQ_ESC_TELEMETRY_DATA::deser(version, payload)
                .map(|s| MavMessage::AQ_ESC_TELEMETRY(s)),
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
            MavMessage::AQ_TELEMETRY_F(..) => "AQ_TELEMETRY_F",
            MavMessage::AQ_ESC_TELEMETRY(..) => "AQ_ESC_TELEMETRY",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::AQ_TELEMETRY_F(..) => 150,
            MavMessage::AQ_ESC_TELEMETRY(..) => 152,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "AQ_TELEMETRY_F" => Ok(150),
            "AQ_ESC_TELEMETRY" => Ok(152),
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
            150 => Ok(MavMessage::AQ_TELEMETRY_F(AQ_TELEMETRY_F_DATA::default())),
            152 => Ok(MavMessage::AQ_ESC_TELEMETRY(
                AQ_ESC_TELEMETRY_DATA::default(),
            )),
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
            &MavMessage::AQ_TELEMETRY_F(ref body) => body.ser(),
            &MavMessage::AQ_ESC_TELEMETRY(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 241,
            152 => 115,
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
