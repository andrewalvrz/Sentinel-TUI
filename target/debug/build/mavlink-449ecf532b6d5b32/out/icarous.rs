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
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum IcarousTrackBandTypes {
    ICAROUS_TRACK_BAND_TYPE_NONE = 0,
    ICAROUS_TRACK_BAND_TYPE_NEAR = 1,
    ICAROUS_TRACK_BAND_TYPE_RECOVERY = 2,
}
impl Default for IcarousTrackBandTypes {
    fn default() -> Self {
        IcarousTrackBandTypes::ICAROUS_TRACK_BAND_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum IcarousFmsState {
    ICAROUS_FMS_STATE_IDLE = 0,
    ICAROUS_FMS_STATE_TAKEOFF = 1,
    ICAROUS_FMS_STATE_CLIMB = 2,
    ICAROUS_FMS_STATE_CRUISE = 3,
    ICAROUS_FMS_STATE_APPROACH = 4,
    ICAROUS_FMS_STATE_LAND = 5,
}
impl Default for IcarousFmsState {
    fn default() -> Self {
        IcarousFmsState::ICAROUS_FMS_STATE_IDLE
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ICAROUS_HEARTBEAT_DATA {
    pub status: IcarousFmsState,
}
impl ICAROUS_HEARTBEAT_DATA {
    pub const ENCODED_LEN: usize = 1usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ICAROUS_HEARTBEAT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ICAROUS_HEARTBEAT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousFmsState".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ICAROUS_KINEMATIC_BANDS_DATA {
    pub min1: f32,
    pub max1: f32,
    pub min2: f32,
    pub max2: f32,
    pub min3: f32,
    pub max3: f32,
    pub min4: f32,
    pub max4: f32,
    pub min5: f32,
    pub max5: f32,
    pub numBands: i8,
    pub type1: IcarousTrackBandTypes,
    pub type2: IcarousTrackBandTypes,
    pub type3: IcarousTrackBandTypes,
    pub type4: IcarousTrackBandTypes,
    pub type5: IcarousTrackBandTypes,
}
impl ICAROUS_KINEMATIC_BANDS_DATA {
    pub const ENCODED_LEN: usize = 46usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ICAROUS_KINEMATIC_BANDS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ICAROUS_KINEMATIC_BANDS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.min1 = buf.get_f32_le();
        _struct.max1 = buf.get_f32_le();
        _struct.min2 = buf.get_f32_le();
        _struct.max2 = buf.get_f32_le();
        _struct.min3 = buf.get_f32_le();
        _struct.max3 = buf.get_f32_le();
        _struct.min4 = buf.get_f32_le();
        _struct.max4 = buf.get_f32_le();
        _struct.min5 = buf.get_f32_le();
        _struct.max5 = buf.get_f32_le();
        _struct.numBands = buf.get_i8();
        let tmp = buf.get_u8();
        _struct.type1 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousTrackBandTypes".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.type2 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousTrackBandTypes".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.type3 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousTrackBandTypes".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.type4 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousTrackBandTypes".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.type5 = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "IcarousTrackBandTypes".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.min1);
        _tmp.put_f32_le(self.max1);
        _tmp.put_f32_le(self.min2);
        _tmp.put_f32_le(self.max2);
        _tmp.put_f32_le(self.min3);
        _tmp.put_f32_le(self.max3);
        _tmp.put_f32_le(self.min4);
        _tmp.put_f32_le(self.max4);
        _tmp.put_f32_le(self.min5);
        _tmp.put_f32_le(self.max5);
        _tmp.put_i8(self.numBands);
        _tmp.put_u8(self.type1 as u8);
        _tmp.put_u8(self.type2 as u8);
        _tmp.put_u8(self.type3 as u8);
        _tmp.put_u8(self.type4 as u8);
        _tmp.put_u8(self.type5 as u8);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    ICAROUS_HEARTBEAT(ICAROUS_HEARTBEAT_DATA),
    ICAROUS_KINEMATIC_BANDS(ICAROUS_KINEMATIC_BANDS_DATA),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            42000 => ICAROUS_HEARTBEAT_DATA::deser(version, payload)
                .map(|s| MavMessage::ICAROUS_HEARTBEAT(s)),
            42001 => ICAROUS_KINEMATIC_BANDS_DATA::deser(version, payload)
                .map(|s| MavMessage::ICAROUS_KINEMATIC_BANDS(s)),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::ICAROUS_HEARTBEAT(..) => "ICAROUS_HEARTBEAT",
            MavMessage::ICAROUS_KINEMATIC_BANDS(..) => "ICAROUS_KINEMATIC_BANDS",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::ICAROUS_HEARTBEAT(..) => 42000,
            MavMessage::ICAROUS_KINEMATIC_BANDS(..) => 42001,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "ICAROUS_HEARTBEAT" => Ok(42000),
            "ICAROUS_KINEMATIC_BANDS" => Ok(42001),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            42000 => Ok(MavMessage::ICAROUS_HEARTBEAT(
                ICAROUS_HEARTBEAT_DATA::default(),
            )),
            42001 => Ok(MavMessage::ICAROUS_KINEMATIC_BANDS(
                ICAROUS_KINEMATIC_BANDS_DATA::default(),
            )),
            _ => {
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::ICAROUS_HEARTBEAT(ref body) => body.ser(),
            &MavMessage::ICAROUS_KINEMATIC_BANDS(ref body) => body.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            42000 => 227,
            42001 => 239,
            _ => 0,
        }
    }
}
