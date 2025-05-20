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
pub enum MavCmd {
    MAV_CMD_RESET_MPPT = 40001,
    MAV_CMD_PAYLOAD_CONTROL = 40002,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_RESET_MPPT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GsmLinkType {
    GSM_LINK_TYPE_NONE = 0,
    GSM_LINK_TYPE_UNKNOWN = 1,
    GSM_LINK_TYPE_2G = 2,
    GSM_LINK_TYPE_3G = 3,
    GSM_LINK_TYPE_4G = 4,
}
impl Default for GsmLinkType {
    fn default() -> Self {
        GsmLinkType::GSM_LINK_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GsmModemType {
    GSM_MODEM_TYPE_UNKNOWN = 0,
    GSM_MODEM_TYPE_HUAWEI_E3372 = 1,
}
impl Default for GsmModemType {
    fn default() -> Self {
        GsmModemType::GSM_MODEM_TYPE_UNKNOWN
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMMAND_INT_STAMPED_DATA {
    pub vehicle_timestamp: u64,
    pub utc_time: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub command: MavCmd,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: MavFrame,
    pub current: u8,
    pub autocontinue: u8,
}
impl COMMAND_INT_STAMPED_DATA {
    pub const ENCODED_LEN: usize = 47usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_INT_STAMPED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_INT_STAMPED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.vehicle_timestamp = buf.get_u64_le();
        _struct.utc_time = buf.get_u32_le();
        _struct.param1 = buf.get_f32_le();
        _struct.param2 = buf.get_f32_le();
        _struct.param3 = buf.get_f32_le();
        _struct.param4 = buf.get_f32_le();
        _struct.x = buf.get_i32_le();
        _struct.y = buf.get_i32_le();
        _struct.z = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCmd".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        _struct.current = buf.get_u8();
        _struct.autocontinue = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.vehicle_timestamp);
        _tmp.put_u32_le(self.utc_time);
        _tmp.put_f32_le(self.param1);
        _tmp.put_f32_le(self.param2);
        _tmp.put_f32_le(self.param3);
        _tmp.put_f32_le(self.param4);
        _tmp.put_i32_le(self.x);
        _tmp.put_i32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.frame as u8);
        _tmp.put_u8(self.current);
        _tmp.put_u8(self.autocontinue);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMMAND_LONG_STAMPED_DATA {
    pub vehicle_timestamp: u64,
    pub utc_time: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub param5: f32,
    pub param6: f32,
    pub param7: f32,
    pub command: MavCmd,
    pub target_system: u8,
    pub target_component: u8,
    pub confirmation: u8,
}
impl COMMAND_LONG_STAMPED_DATA {
    pub const ENCODED_LEN: usize = 45usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_LONG_STAMPED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_LONG_STAMPED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.vehicle_timestamp = buf.get_u64_le();
        _struct.utc_time = buf.get_u32_le();
        _struct.param1 = buf.get_f32_le();
        _struct.param2 = buf.get_f32_le();
        _struct.param3 = buf.get_f32_le();
        _struct.param4 = buf.get_f32_le();
        _struct.param5 = buf.get_f32_le();
        _struct.param6 = buf.get_f32_le();
        _struct.param7 = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCmd".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.confirmation = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.vehicle_timestamp);
        _tmp.put_u32_le(self.utc_time);
        _tmp.put_f32_le(self.param1);
        _tmp.put_f32_le(self.param2);
        _tmp.put_f32_le(self.param3);
        _tmp.put_f32_le(self.param4);
        _tmp.put_f32_le(self.param5);
        _tmp.put_f32_le(self.param6);
        _tmp.put_f32_le(self.param7);
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.confirmation);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENS_POWER_DATA {
    pub adc121_vspb_volt: f32,
    pub adc121_cspb_amp: f32,
    pub adc121_cs1_amp: f32,
    pub adc121_cs2_amp: f32,
}
impl SENS_POWER_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENS_POWER_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENS_POWER_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.adc121_vspb_volt = buf.get_f32_le();
        _struct.adc121_cspb_amp = buf.get_f32_le();
        _struct.adc121_cs1_amp = buf.get_f32_le();
        _struct.adc121_cs2_amp = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.adc121_vspb_volt);
        _tmp.put_f32_le(self.adc121_cspb_amp);
        _tmp.put_f32_le(self.adc121_cs1_amp);
        _tmp.put_f32_le(self.adc121_cs2_amp);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENS_MPPT_DATA {
    pub mppt_timestamp: u64,
    pub mppt1_volt: f32,
    pub mppt1_amp: f32,
    pub mppt2_volt: f32,
    pub mppt2_amp: f32,
    pub mppt3_volt: f32,
    pub mppt3_amp: f32,
    pub mppt1_pwm: u16,
    pub mppt2_pwm: u16,
    pub mppt3_pwm: u16,
    pub mppt1_status: u8,
    pub mppt2_status: u8,
    pub mppt3_status: u8,
}
impl SENS_MPPT_DATA {
    pub const ENCODED_LEN: usize = 41usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENS_MPPT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENS_MPPT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mppt_timestamp = buf.get_u64_le();
        _struct.mppt1_volt = buf.get_f32_le();
        _struct.mppt1_amp = buf.get_f32_le();
        _struct.mppt2_volt = buf.get_f32_le();
        _struct.mppt2_amp = buf.get_f32_le();
        _struct.mppt3_volt = buf.get_f32_le();
        _struct.mppt3_amp = buf.get_f32_le();
        _struct.mppt1_pwm = buf.get_u16_le();
        _struct.mppt2_pwm = buf.get_u16_le();
        _struct.mppt3_pwm = buf.get_u16_le();
        _struct.mppt1_status = buf.get_u8();
        _struct.mppt2_status = buf.get_u8();
        _struct.mppt3_status = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.mppt_timestamp);
        _tmp.put_f32_le(self.mppt1_volt);
        _tmp.put_f32_le(self.mppt1_amp);
        _tmp.put_f32_le(self.mppt2_volt);
        _tmp.put_f32_le(self.mppt2_amp);
        _tmp.put_f32_le(self.mppt3_volt);
        _tmp.put_f32_le(self.mppt3_amp);
        _tmp.put_u16_le(self.mppt1_pwm);
        _tmp.put_u16_le(self.mppt2_pwm);
        _tmp.put_u16_le(self.mppt3_pwm);
        _tmp.put_u8(self.mppt1_status);
        _tmp.put_u8(self.mppt2_status);
        _tmp.put_u8(self.mppt3_status);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASLCTRL_DATA_DATA {
    pub timestamp: u64,
    pub h: f32,
    pub hRef: f32,
    pub hRef_t: f32,
    pub PitchAngle: f32,
    pub PitchAngleRef: f32,
    pub q: f32,
    pub qRef: f32,
    pub uElev: f32,
    pub uThrot: f32,
    pub uThrot2: f32,
    pub nZ: f32,
    pub AirspeedRef: f32,
    pub YawAngle: f32,
    pub YawAngleRef: f32,
    pub RollAngle: f32,
    pub RollAngleRef: f32,
    pub p: f32,
    pub pRef: f32,
    pub r: f32,
    pub rRef: f32,
    pub uAil: f32,
    pub uRud: f32,
    pub aslctrl_mode: u8,
    pub SpoilersEngaged: u8,
}
impl ASLCTRL_DATA_DATA {
    pub const ENCODED_LEN: usize = 98usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ASLCTRL_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ASLCTRL_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.h = buf.get_f32_le();
        _struct.hRef = buf.get_f32_le();
        _struct.hRef_t = buf.get_f32_le();
        _struct.PitchAngle = buf.get_f32_le();
        _struct.PitchAngleRef = buf.get_f32_le();
        _struct.q = buf.get_f32_le();
        _struct.qRef = buf.get_f32_le();
        _struct.uElev = buf.get_f32_le();
        _struct.uThrot = buf.get_f32_le();
        _struct.uThrot2 = buf.get_f32_le();
        _struct.nZ = buf.get_f32_le();
        _struct.AirspeedRef = buf.get_f32_le();
        _struct.YawAngle = buf.get_f32_le();
        _struct.YawAngleRef = buf.get_f32_le();
        _struct.RollAngle = buf.get_f32_le();
        _struct.RollAngleRef = buf.get_f32_le();
        _struct.p = buf.get_f32_le();
        _struct.pRef = buf.get_f32_le();
        _struct.r = buf.get_f32_le();
        _struct.rRef = buf.get_f32_le();
        _struct.uAil = buf.get_f32_le();
        _struct.uRud = buf.get_f32_le();
        _struct.aslctrl_mode = buf.get_u8();
        _struct.SpoilersEngaged = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_f32_le(self.h);
        _tmp.put_f32_le(self.hRef);
        _tmp.put_f32_le(self.hRef_t);
        _tmp.put_f32_le(self.PitchAngle);
        _tmp.put_f32_le(self.PitchAngleRef);
        _tmp.put_f32_le(self.q);
        _tmp.put_f32_le(self.qRef);
        _tmp.put_f32_le(self.uElev);
        _tmp.put_f32_le(self.uThrot);
        _tmp.put_f32_le(self.uThrot2);
        _tmp.put_f32_le(self.nZ);
        _tmp.put_f32_le(self.AirspeedRef);
        _tmp.put_f32_le(self.YawAngle);
        _tmp.put_f32_le(self.YawAngleRef);
        _tmp.put_f32_le(self.RollAngle);
        _tmp.put_f32_le(self.RollAngleRef);
        _tmp.put_f32_le(self.p);
        _tmp.put_f32_le(self.pRef);
        _tmp.put_f32_le(self.r);
        _tmp.put_f32_le(self.rRef);
        _tmp.put_f32_le(self.uAil);
        _tmp.put_f32_le(self.uRud);
        _tmp.put_u8(self.aslctrl_mode);
        _tmp.put_u8(self.SpoilersEngaged);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASLCTRL_DEBUG_DATA {
    pub i32_1: u32,
    pub f_1: f32,
    pub f_2: f32,
    pub f_3: f32,
    pub f_4: f32,
    pub f_5: f32,
    pub f_6: f32,
    pub f_7: f32,
    pub f_8: f32,
    pub i8_1: u8,
    pub i8_2: u8,
}
impl ASLCTRL_DEBUG_DATA {
    pub const ENCODED_LEN: usize = 38usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ASLCTRL_DEBUG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ASLCTRL_DEBUG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.i32_1 = buf.get_u32_le();
        _struct.f_1 = buf.get_f32_le();
        _struct.f_2 = buf.get_f32_le();
        _struct.f_3 = buf.get_f32_le();
        _struct.f_4 = buf.get_f32_le();
        _struct.f_5 = buf.get_f32_le();
        _struct.f_6 = buf.get_f32_le();
        _struct.f_7 = buf.get_f32_le();
        _struct.f_8 = buf.get_f32_le();
        _struct.i8_1 = buf.get_u8();
        _struct.i8_2 = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.i32_1);
        _tmp.put_f32_le(self.f_1);
        _tmp.put_f32_le(self.f_2);
        _tmp.put_f32_le(self.f_3);
        _tmp.put_f32_le(self.f_4);
        _tmp.put_f32_le(self.f_5);
        _tmp.put_f32_le(self.f_6);
        _tmp.put_f32_le(self.f_7);
        _tmp.put_f32_le(self.f_8);
        _tmp.put_u8(self.i8_1);
        _tmp.put_u8(self.i8_2);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASLUAV_STATUS_DATA {
    pub Motor_rpm: f32,
    pub LED_status: u8,
    pub SATCOM_status: u8,
    pub Servo_status: [u8; 8],
}
impl ASLUAV_STATUS_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ASLUAV_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ASLUAV_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.Motor_rpm = buf.get_f32_le();
        _struct.LED_status = buf.get_u8();
        _struct.SATCOM_status = buf.get_u8();
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.Servo_status[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.Motor_rpm);
        _tmp.put_u8(self.LED_status);
        _tmp.put_u8(self.SATCOM_status);
        for val in &self.Servo_status {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EKF_EXT_DATA {
    pub timestamp: u64,
    pub Windspeed: f32,
    pub WindDir: f32,
    pub WindZ: f32,
    pub Airspeed: f32,
    pub beta: f32,
    pub alpha: f32,
}
impl EKF_EXT_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < EKF_EXT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; EKF_EXT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.Windspeed = buf.get_f32_le();
        _struct.WindDir = buf.get_f32_le();
        _struct.WindZ = buf.get_f32_le();
        _struct.Airspeed = buf.get_f32_le();
        _struct.beta = buf.get_f32_le();
        _struct.alpha = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_f32_le(self.Windspeed);
        _tmp.put_f32_le(self.WindDir);
        _tmp.put_f32_le(self.WindZ);
        _tmp.put_f32_le(self.Airspeed);
        _tmp.put_f32_le(self.beta);
        _tmp.put_f32_le(self.alpha);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASL_OBCTRL_DATA {
    pub timestamp: u64,
    pub uElev: f32,
    pub uThrot: f32,
    pub uThrot2: f32,
    pub uAilL: f32,
    pub uAilR: f32,
    pub uRud: f32,
    pub obctrl_status: u8,
}
impl ASL_OBCTRL_DATA {
    pub const ENCODED_LEN: usize = 33usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ASL_OBCTRL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ASL_OBCTRL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.uElev = buf.get_f32_le();
        _struct.uThrot = buf.get_f32_le();
        _struct.uThrot2 = buf.get_f32_le();
        _struct.uAilL = buf.get_f32_le();
        _struct.uAilR = buf.get_f32_le();
        _struct.uRud = buf.get_f32_le();
        _struct.obctrl_status = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_f32_le(self.uElev);
        _tmp.put_f32_le(self.uThrot);
        _tmp.put_f32_le(self.uThrot2);
        _tmp.put_f32_le(self.uAilL);
        _tmp.put_f32_le(self.uAilR);
        _tmp.put_f32_le(self.uRud);
        _tmp.put_u8(self.obctrl_status);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENS_ATMOS_DATA {
    pub timestamp: u64,
    pub TempAmbient: f32,
    pub Humidity: f32,
}
impl SENS_ATMOS_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENS_ATMOS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENS_ATMOS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.TempAmbient = buf.get_f32_le();
        _struct.Humidity = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_f32_le(self.TempAmbient);
        _tmp.put_f32_le(self.Humidity);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENS_BATMON_DATA {
    pub batmon_timestamp: u64,
    pub temperature: f32,
    pub safetystatus: u32,
    pub operationstatus: u32,
    pub voltage: u16,
    pub current: i16,
    pub batterystatus: u16,
    pub serialnumber: u16,
    pub cellvoltage1: u16,
    pub cellvoltage2: u16,
    pub cellvoltage3: u16,
    pub cellvoltage4: u16,
    pub cellvoltage5: u16,
    pub cellvoltage6: u16,
    pub SoC: u8,
}
impl SENS_BATMON_DATA {
    pub const ENCODED_LEN: usize = 41usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENS_BATMON_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENS_BATMON_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.batmon_timestamp = buf.get_u64_le();
        _struct.temperature = buf.get_f32_le();
        _struct.safetystatus = buf.get_u32_le();
        _struct.operationstatus = buf.get_u32_le();
        _struct.voltage = buf.get_u16_le();
        _struct.current = buf.get_i16_le();
        _struct.batterystatus = buf.get_u16_le();
        _struct.serialnumber = buf.get_u16_le();
        _struct.cellvoltage1 = buf.get_u16_le();
        _struct.cellvoltage2 = buf.get_u16_le();
        _struct.cellvoltage3 = buf.get_u16_le();
        _struct.cellvoltage4 = buf.get_u16_le();
        _struct.cellvoltage5 = buf.get_u16_le();
        _struct.cellvoltage6 = buf.get_u16_le();
        _struct.SoC = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.batmon_timestamp);
        _tmp.put_f32_le(self.temperature);
        _tmp.put_u32_le(self.safetystatus);
        _tmp.put_u32_le(self.operationstatus);
        _tmp.put_u16_le(self.voltage);
        _tmp.put_i16_le(self.current);
        _tmp.put_u16_le(self.batterystatus);
        _tmp.put_u16_le(self.serialnumber);
        _tmp.put_u16_le(self.cellvoltage1);
        _tmp.put_u16_le(self.cellvoltage2);
        _tmp.put_u16_le(self.cellvoltage3);
        _tmp.put_u16_le(self.cellvoltage4);
        _tmp.put_u16_le(self.cellvoltage5);
        _tmp.put_u16_le(self.cellvoltage6);
        _tmp.put_u8(self.SoC);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FW_SOARING_DATA_DATA {
    pub timestamp: u64,
    pub timestampModeChanged: u64,
    pub xW: f32,
    pub xR: f32,
    pub xLat: f32,
    pub xLon: f32,
    pub VarW: f32,
    pub VarR: f32,
    pub VarLat: f32,
    pub VarLon: f32,
    pub LoiterRadius: f32,
    pub LoiterDirection: f32,
    pub DistToSoarPoint: f32,
    pub vSinkExp: f32,
    pub z1_LocalUpdraftSpeed: f32,
    pub z2_DeltaRoll: f32,
    pub z1_exp: f32,
    pub z2_exp: f32,
    pub ThermalGSNorth: f32,
    pub ThermalGSEast: f32,
    pub TSE_dot: f32,
    pub DebugVar1: f32,
    pub DebugVar2: f32,
    pub ControlMode: u8,
    pub valid: u8,
}
impl FW_SOARING_DATA_DATA {
    pub const ENCODED_LEN: usize = 102usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FW_SOARING_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FW_SOARING_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.timestampModeChanged = buf.get_u64_le();
        _struct.xW = buf.get_f32_le();
        _struct.xR = buf.get_f32_le();
        _struct.xLat = buf.get_f32_le();
        _struct.xLon = buf.get_f32_le();
        _struct.VarW = buf.get_f32_le();
        _struct.VarR = buf.get_f32_le();
        _struct.VarLat = buf.get_f32_le();
        _struct.VarLon = buf.get_f32_le();
        _struct.LoiterRadius = buf.get_f32_le();
        _struct.LoiterDirection = buf.get_f32_le();
        _struct.DistToSoarPoint = buf.get_f32_le();
        _struct.vSinkExp = buf.get_f32_le();
        _struct.z1_LocalUpdraftSpeed = buf.get_f32_le();
        _struct.z2_DeltaRoll = buf.get_f32_le();
        _struct.z1_exp = buf.get_f32_le();
        _struct.z2_exp = buf.get_f32_le();
        _struct.ThermalGSNorth = buf.get_f32_le();
        _struct.ThermalGSEast = buf.get_f32_le();
        _struct.TSE_dot = buf.get_f32_le();
        _struct.DebugVar1 = buf.get_f32_le();
        _struct.DebugVar2 = buf.get_f32_le();
        _struct.ControlMode = buf.get_u8();
        _struct.valid = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u64_le(self.timestampModeChanged);
        _tmp.put_f32_le(self.xW);
        _tmp.put_f32_le(self.xR);
        _tmp.put_f32_le(self.xLat);
        _tmp.put_f32_le(self.xLon);
        _tmp.put_f32_le(self.VarW);
        _tmp.put_f32_le(self.VarR);
        _tmp.put_f32_le(self.VarLat);
        _tmp.put_f32_le(self.VarLon);
        _tmp.put_f32_le(self.LoiterRadius);
        _tmp.put_f32_le(self.LoiterDirection);
        _tmp.put_f32_le(self.DistToSoarPoint);
        _tmp.put_f32_le(self.vSinkExp);
        _tmp.put_f32_le(self.z1_LocalUpdraftSpeed);
        _tmp.put_f32_le(self.z2_DeltaRoll);
        _tmp.put_f32_le(self.z1_exp);
        _tmp.put_f32_le(self.z2_exp);
        _tmp.put_f32_le(self.ThermalGSNorth);
        _tmp.put_f32_le(self.ThermalGSEast);
        _tmp.put_f32_le(self.TSE_dot);
        _tmp.put_f32_le(self.DebugVar1);
        _tmp.put_f32_le(self.DebugVar2);
        _tmp.put_u8(self.ControlMode);
        _tmp.put_u8(self.valid);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENSORPOD_STATUS_DATA {
    pub timestamp: u64,
    pub free_space: u16,
    pub visensor_rate_1: u8,
    pub visensor_rate_2: u8,
    pub visensor_rate_3: u8,
    pub visensor_rate_4: u8,
    pub recording_nodes_count: u8,
    pub cpu_temp: u8,
}
impl SENSORPOD_STATUS_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENSORPOD_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENSORPOD_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.free_space = buf.get_u16_le();
        _struct.visensor_rate_1 = buf.get_u8();
        _struct.visensor_rate_2 = buf.get_u8();
        _struct.visensor_rate_3 = buf.get_u8();
        _struct.visensor_rate_4 = buf.get_u8();
        _struct.recording_nodes_count = buf.get_u8();
        _struct.cpu_temp = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u16_le(self.free_space);
        _tmp.put_u8(self.visensor_rate_1);
        _tmp.put_u8(self.visensor_rate_2);
        _tmp.put_u8(self.visensor_rate_3);
        _tmp.put_u8(self.visensor_rate_4);
        _tmp.put_u8(self.recording_nodes_count);
        _tmp.put_u8(self.cpu_temp);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENS_POWER_BOARD_DATA {
    pub timestamp: u64,
    pub pwr_brd_system_volt: f32,
    pub pwr_brd_servo_volt: f32,
    pub pwr_brd_digital_volt: f32,
    pub pwr_brd_mot_l_amp: f32,
    pub pwr_brd_mot_r_amp: f32,
    pub pwr_brd_analog_amp: f32,
    pub pwr_brd_digital_amp: f32,
    pub pwr_brd_ext_amp: f32,
    pub pwr_brd_aux_amp: f32,
    pub pwr_brd_status: u8,
    pub pwr_brd_led_status: u8,
}
impl SENS_POWER_BOARD_DATA {
    pub const ENCODED_LEN: usize = 46usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENS_POWER_BOARD_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENS_POWER_BOARD_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.pwr_brd_system_volt = buf.get_f32_le();
        _struct.pwr_brd_servo_volt = buf.get_f32_le();
        _struct.pwr_brd_digital_volt = buf.get_f32_le();
        _struct.pwr_brd_mot_l_amp = buf.get_f32_le();
        _struct.pwr_brd_mot_r_amp = buf.get_f32_le();
        _struct.pwr_brd_analog_amp = buf.get_f32_le();
        _struct.pwr_brd_digital_amp = buf.get_f32_le();
        _struct.pwr_brd_ext_amp = buf.get_f32_le();
        _struct.pwr_brd_aux_amp = buf.get_f32_le();
        _struct.pwr_brd_status = buf.get_u8();
        _struct.pwr_brd_led_status = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_f32_le(self.pwr_brd_system_volt);
        _tmp.put_f32_le(self.pwr_brd_servo_volt);
        _tmp.put_f32_le(self.pwr_brd_digital_volt);
        _tmp.put_f32_le(self.pwr_brd_mot_l_amp);
        _tmp.put_f32_le(self.pwr_brd_mot_r_amp);
        _tmp.put_f32_le(self.pwr_brd_analog_amp);
        _tmp.put_f32_le(self.pwr_brd_digital_amp);
        _tmp.put_f32_le(self.pwr_brd_ext_amp);
        _tmp.put_f32_le(self.pwr_brd_aux_amp);
        _tmp.put_u8(self.pwr_brd_status);
        _tmp.put_u8(self.pwr_brd_led_status);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GSM_LINK_STATUS_DATA {
    pub timestamp: u64,
    pub gsm_modem_type: GsmModemType,
    pub gsm_link_type: GsmLinkType,
    pub rssi: u8,
    pub rsrp_rscp: u8,
    pub sinr_ecio: u8,
    pub rsrq: u8,
}
impl GSM_LINK_STATUS_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GSM_LINK_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GSM_LINK_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        let tmp = buf.get_u8();
        _struct.gsm_modem_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GsmModemType".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.gsm_link_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GsmLinkType".to_string(),
            value: tmp as u32,
        })?;
        _struct.rssi = buf.get_u8();
        _struct.rsrp_rscp = buf.get_u8();
        _struct.sinr_ecio = buf.get_u8();
        _struct.rsrq = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u8(self.gsm_modem_type as u8);
        _tmp.put_u8(self.gsm_link_type as u8);
        _tmp.put_u8(self.rssi);
        _tmp.put_u8(self.rsrp_rscp);
        _tmp.put_u8(self.sinr_ecio);
        _tmp.put_u8(self.rsrq);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    COMMAND_INT_STAMPED(COMMAND_INT_STAMPED_DATA),
    COMMAND_LONG_STAMPED(COMMAND_LONG_STAMPED_DATA),
    SENS_POWER(SENS_POWER_DATA),
    SENS_MPPT(SENS_MPPT_DATA),
    ASLCTRL_DATA(ASLCTRL_DATA_DATA),
    ASLCTRL_DEBUG(ASLCTRL_DEBUG_DATA),
    ASLUAV_STATUS(ASLUAV_STATUS_DATA),
    EKF_EXT(EKF_EXT_DATA),
    ASL_OBCTRL(ASL_OBCTRL_DATA),
    SENS_ATMOS(SENS_ATMOS_DATA),
    SENS_BATMON(SENS_BATMON_DATA),
    FW_SOARING_DATA(FW_SOARING_DATA_DATA),
    SENSORPOD_STATUS(SENSORPOD_STATUS_DATA),
    SENS_POWER_BOARD(SENS_POWER_BOARD_DATA),
    GSM_LINK_STATUS(GSM_LINK_STATUS_DATA),
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
            78 => COMMAND_INT_STAMPED_DATA::deser(version, payload)
                .map(|s| MavMessage::COMMAND_INT_STAMPED(s)),
            79 => COMMAND_LONG_STAMPED_DATA::deser(version, payload)
                .map(|s| MavMessage::COMMAND_LONG_STAMPED(s)),
            201 => SENS_POWER_DATA::deser(version, payload).map(|s| MavMessage::SENS_POWER(s)),
            202 => SENS_MPPT_DATA::deser(version, payload).map(|s| MavMessage::SENS_MPPT(s)),
            203 => ASLCTRL_DATA_DATA::deser(version, payload).map(|s| MavMessage::ASLCTRL_DATA(s)),
            204 => {
                ASLCTRL_DEBUG_DATA::deser(version, payload).map(|s| MavMessage::ASLCTRL_DEBUG(s))
            }
            205 => {
                ASLUAV_STATUS_DATA::deser(version, payload).map(|s| MavMessage::ASLUAV_STATUS(s))
            }
            206 => EKF_EXT_DATA::deser(version, payload).map(|s| MavMessage::EKF_EXT(s)),
            207 => ASL_OBCTRL_DATA::deser(version, payload).map(|s| MavMessage::ASL_OBCTRL(s)),
            208 => SENS_ATMOS_DATA::deser(version, payload).map(|s| MavMessage::SENS_ATMOS(s)),
            209 => SENS_BATMON_DATA::deser(version, payload).map(|s| MavMessage::SENS_BATMON(s)),
            210 => FW_SOARING_DATA_DATA::deser(version, payload)
                .map(|s| MavMessage::FW_SOARING_DATA(s)),
            211 => SENSORPOD_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::SENSORPOD_STATUS(s)),
            212 => SENS_POWER_BOARD_DATA::deser(version, payload)
                .map(|s| MavMessage::SENS_POWER_BOARD(s)),
            213 => GSM_LINK_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::GSM_LINK_STATUS(s)),
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
            MavMessage::COMMAND_INT_STAMPED(..) => "COMMAND_INT_STAMPED",
            MavMessage::COMMAND_LONG_STAMPED(..) => "COMMAND_LONG_STAMPED",
            MavMessage::SENS_POWER(..) => "SENS_POWER",
            MavMessage::SENS_MPPT(..) => "SENS_MPPT",
            MavMessage::ASLCTRL_DATA(..) => "ASLCTRL_DATA",
            MavMessage::ASLCTRL_DEBUG(..) => "ASLCTRL_DEBUG",
            MavMessage::ASLUAV_STATUS(..) => "ASLUAV_STATUS",
            MavMessage::EKF_EXT(..) => "EKF_EXT",
            MavMessage::ASL_OBCTRL(..) => "ASL_OBCTRL",
            MavMessage::SENS_ATMOS(..) => "SENS_ATMOS",
            MavMessage::SENS_BATMON(..) => "SENS_BATMON",
            MavMessage::FW_SOARING_DATA(..) => "FW_SOARING_DATA",
            MavMessage::SENSORPOD_STATUS(..) => "SENSORPOD_STATUS",
            MavMessage::SENS_POWER_BOARD(..) => "SENS_POWER_BOARD",
            MavMessage::GSM_LINK_STATUS(..) => "GSM_LINK_STATUS",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::COMMAND_INT_STAMPED(..) => 78,
            MavMessage::COMMAND_LONG_STAMPED(..) => 79,
            MavMessage::SENS_POWER(..) => 201,
            MavMessage::SENS_MPPT(..) => 202,
            MavMessage::ASLCTRL_DATA(..) => 203,
            MavMessage::ASLCTRL_DEBUG(..) => 204,
            MavMessage::ASLUAV_STATUS(..) => 205,
            MavMessage::EKF_EXT(..) => 206,
            MavMessage::ASL_OBCTRL(..) => 207,
            MavMessage::SENS_ATMOS(..) => 208,
            MavMessage::SENS_BATMON(..) => 209,
            MavMessage::FW_SOARING_DATA(..) => 210,
            MavMessage::SENSORPOD_STATUS(..) => 211,
            MavMessage::SENS_POWER_BOARD(..) => 212,
            MavMessage::GSM_LINK_STATUS(..) => 213,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "COMMAND_INT_STAMPED" => Ok(78),
            "COMMAND_LONG_STAMPED" => Ok(79),
            "SENS_POWER" => Ok(201),
            "SENS_MPPT" => Ok(202),
            "ASLCTRL_DATA" => Ok(203),
            "ASLCTRL_DEBUG" => Ok(204),
            "ASLUAV_STATUS" => Ok(205),
            "EKF_EXT" => Ok(206),
            "ASL_OBCTRL" => Ok(207),
            "SENS_ATMOS" => Ok(208),
            "SENS_BATMON" => Ok(209),
            "FW_SOARING_DATA" => Ok(210),
            "SENSORPOD_STATUS" => Ok(211),
            "SENS_POWER_BOARD" => Ok(212),
            "GSM_LINK_STATUS" => Ok(213),
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
            78 => Ok(MavMessage::COMMAND_INT_STAMPED(
                COMMAND_INT_STAMPED_DATA::default(),
            )),
            79 => Ok(MavMessage::COMMAND_LONG_STAMPED(
                COMMAND_LONG_STAMPED_DATA::default(),
            )),
            201 => Ok(MavMessage::SENS_POWER(SENS_POWER_DATA::default())),
            202 => Ok(MavMessage::SENS_MPPT(SENS_MPPT_DATA::default())),
            203 => Ok(MavMessage::ASLCTRL_DATA(ASLCTRL_DATA_DATA::default())),
            204 => Ok(MavMessage::ASLCTRL_DEBUG(ASLCTRL_DEBUG_DATA::default())),
            205 => Ok(MavMessage::ASLUAV_STATUS(ASLUAV_STATUS_DATA::default())),
            206 => Ok(MavMessage::EKF_EXT(EKF_EXT_DATA::default())),
            207 => Ok(MavMessage::ASL_OBCTRL(ASL_OBCTRL_DATA::default())),
            208 => Ok(MavMessage::SENS_ATMOS(SENS_ATMOS_DATA::default())),
            209 => Ok(MavMessage::SENS_BATMON(SENS_BATMON_DATA::default())),
            210 => Ok(MavMessage::FW_SOARING_DATA(FW_SOARING_DATA_DATA::default())),
            211 => Ok(MavMessage::SENSORPOD_STATUS(
                SENSORPOD_STATUS_DATA::default(),
            )),
            212 => Ok(MavMessage::SENS_POWER_BOARD(
                SENS_POWER_BOARD_DATA::default(),
            )),
            213 => Ok(MavMessage::GSM_LINK_STATUS(GSM_LINK_STATUS_DATA::default())),
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
            &MavMessage::COMMAND_INT_STAMPED(ref body) => body.ser(),
            &MavMessage::COMMAND_LONG_STAMPED(ref body) => body.ser(),
            &MavMessage::SENS_POWER(ref body) => body.ser(),
            &MavMessage::SENS_MPPT(ref body) => body.ser(),
            &MavMessage::ASLCTRL_DATA(ref body) => body.ser(),
            &MavMessage::ASLCTRL_DEBUG(ref body) => body.ser(),
            &MavMessage::ASLUAV_STATUS(ref body) => body.ser(),
            &MavMessage::EKF_EXT(ref body) => body.ser(),
            &MavMessage::ASL_OBCTRL(ref body) => body.ser(),
            &MavMessage::SENS_ATMOS(ref body) => body.ser(),
            &MavMessage::SENS_BATMON(ref body) => body.ser(),
            &MavMessage::FW_SOARING_DATA(ref body) => body.ser(),
            &MavMessage::SENSORPOD_STATUS(ref body) => body.ser(),
            &MavMessage::SENS_POWER_BOARD(ref body) => body.ser(),
            &MavMessage::GSM_LINK_STATUS(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            78 => 119,
            79 => 102,
            201 => 218,
            202 => 231,
            203 => 172,
            204 => 251,
            205 => 97,
            206 => 64,
            207 => 234,
            208 => 144,
            209 => 155,
            210 => 20,
            211 => 54,
            212 => 222,
            213 => 200,
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
