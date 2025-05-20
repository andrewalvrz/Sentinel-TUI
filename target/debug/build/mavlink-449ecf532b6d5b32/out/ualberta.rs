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
pub enum UalbertaAutopilotMode {
    MODE_MANUAL_DIRECT = 0,
    MODE_MANUAL_SCALED = 1,
    MODE_AUTO_PID_ATT = 2,
    MODE_AUTO_PID_VEL = 3,
    MODE_AUTO_PID_POS = 4,
}
impl Default for UalbertaAutopilotMode {
    fn default() -> Self {
        UalbertaAutopilotMode::MODE_MANUAL_DIRECT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UalbertaNavMode {
    NAV_AHRS_INIT = 0,
    NAV_AHRS = 1,
    NAV_INS_GPS_INIT = 2,
    NAV_INS_GPS = 3,
}
impl Default for UalbertaNavMode {
    fn default() -> Self {
        UalbertaNavMode::NAV_AHRS_INIT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UalbertaPilotMode {
    PILOT_MANUAL = 0,
    PILOT_AUTO = 1,
    PILOT_ROTO = 2,
}
impl Default for UalbertaPilotMode {
    fn default() -> Self {
        UalbertaPilotMode::PILOT_MANUAL
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NAV_FILTER_BIAS_DATA {
    pub usec: u64,
    pub accel_0: f32,
    pub accel_1: f32,
    pub accel_2: f32,
    pub gyro_0: f32,
    pub gyro_1: f32,
    pub gyro_2: f32,
}
impl NAV_FILTER_BIAS_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < NAV_FILTER_BIAS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; NAV_FILTER_BIAS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.usec = buf.get_u64_le();
        _struct.accel_0 = buf.get_f32_le();
        _struct.accel_1 = buf.get_f32_le();
        _struct.accel_2 = buf.get_f32_le();
        _struct.gyro_0 = buf.get_f32_le();
        _struct.gyro_1 = buf.get_f32_le();
        _struct.gyro_2 = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec);
        _tmp.put_f32_le(self.accel_0);
        _tmp.put_f32_le(self.accel_1);
        _tmp.put_f32_le(self.accel_2);
        _tmp.put_f32_le(self.gyro_0);
        _tmp.put_f32_le(self.gyro_1);
        _tmp.put_f32_le(self.gyro_2);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RADIO_CALIBRATION_DATA {
    pub aileron: [u16; 3],
    pub elevator: [u16; 3],
    pub rudder: [u16; 3],
    pub gyro: [u16; 2],
    pub pitch: [u16; 5],
    pub throttle: [u16; 5],
}
impl RADIO_CALIBRATION_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RADIO_CALIBRATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RADIO_CALIBRATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..3usize {
            let val = buf.get_u16_le();
            _struct.aileron[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_u16_le();
            _struct.elevator[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_u16_le();
            _struct.rudder[idx] = val;
        }
        for idx in 0..2usize {
            let val = buf.get_u16_le();
            _struct.gyro[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_u16_le();
            _struct.pitch[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_u16_le();
            _struct.throttle[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.aileron {
            _tmp.put_u16_le(*val);
        }
        for val in &self.elevator {
            _tmp.put_u16_le(*val);
        }
        for val in &self.rudder {
            _tmp.put_u16_le(*val);
        }
        for val in &self.gyro {
            _tmp.put_u16_le(*val);
        }
        for val in &self.pitch {
            _tmp.put_u16_le(*val);
        }
        for val in &self.throttle {
            _tmp.put_u16_le(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UALBERTA_SYS_STATUS_DATA {
    pub mode: u8,
    pub nav_mode: u8,
    pub pilot: u8,
}
impl UALBERTA_SYS_STATUS_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UALBERTA_SYS_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UALBERTA_SYS_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mode = buf.get_u8();
        _struct.nav_mode = buf.get_u8();
        _struct.pilot = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mode);
        _tmp.put_u8(self.nav_mode);
        _tmp.put_u8(self.pilot);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    NAV_FILTER_BIAS(NAV_FILTER_BIAS_DATA),
    RADIO_CALIBRATION(RADIO_CALIBRATION_DATA),
    UALBERTA_SYS_STATUS(UALBERTA_SYS_STATUS_DATA),
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
            220 => NAV_FILTER_BIAS_DATA::deser(version, payload)
                .map(|s| MavMessage::NAV_FILTER_BIAS(s)),
            221 => RADIO_CALIBRATION_DATA::deser(version, payload)
                .map(|s| MavMessage::RADIO_CALIBRATION(s)),
            222 => UALBERTA_SYS_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::UALBERTA_SYS_STATUS(s)),
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
            MavMessage::NAV_FILTER_BIAS(..) => "NAV_FILTER_BIAS",
            MavMessage::RADIO_CALIBRATION(..) => "RADIO_CALIBRATION",
            MavMessage::UALBERTA_SYS_STATUS(..) => "UALBERTA_SYS_STATUS",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::NAV_FILTER_BIAS(..) => 220,
            MavMessage::RADIO_CALIBRATION(..) => 221,
            MavMessage::UALBERTA_SYS_STATUS(..) => 222,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "NAV_FILTER_BIAS" => Ok(220),
            "RADIO_CALIBRATION" => Ok(221),
            "UALBERTA_SYS_STATUS" => Ok(222),
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
            220 => Ok(MavMessage::NAV_FILTER_BIAS(NAV_FILTER_BIAS_DATA::default())),
            221 => Ok(MavMessage::RADIO_CALIBRATION(
                RADIO_CALIBRATION_DATA::default(),
            )),
            222 => Ok(MavMessage::UALBERTA_SYS_STATUS(
                UALBERTA_SYS_STATUS_DATA::default(),
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
            &MavMessage::NAV_FILTER_BIAS(ref body) => body.ser(),
            &MavMessage::RADIO_CALIBRATION(ref body) => body.ser(),
            &MavMessage::UALBERTA_SYS_STATUS(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            220 => 34,
            221 => 71,
            222 => 15,
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
