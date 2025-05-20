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
    MAV_CMD_DO_NOTHING = 10001,
    MAV_CMD_RETURN_TO_BASE = 10011,
    MAV_CMD_STOP_RETURN_TO_BASE = 10012,
    MAV_CMD_TURN_LIGHT = 10013,
    MAV_CMD_GET_MID_LEVEL_COMMANDS = 10014,
    MAV_CMD_MIDLEVEL_STORAGE = 10015,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_DO_NOTHING
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum SlugsMode {
    SLUGS_MODE_NONE = 0,
    SLUGS_MODE_LIFTOFF = 1,
    SLUGS_MODE_PASSTHROUGH = 2,
    SLUGS_MODE_WAYPOINT = 3,
    SLUGS_MODE_MID_LEVEL = 4,
    SLUGS_MODE_RETURNING = 5,
    SLUGS_MODE_LANDING = 6,
    SLUGS_MODE_LOST = 7,
    SLUGS_MODE_SELECTIVE_PASSTHROUGH = 8,
    SLUGS_MODE_ISR = 9,
    SLUGS_MODE_LINE_PATROL = 10,
    SLUGS_MODE_GROUNDED = 11,
}
impl Default for SlugsMode {
    fn default() -> Self {
        SlugsMode::SLUGS_MODE_NONE
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct ControlSurfaceFlag : u16 { const CONTROL_SURFACE_FLAG_THROTTLE = 128 ; const CONTROL_SURFACE_FLAG_LEFT_AILERON = 64 ; const CONTROL_SURFACE_FLAG_RIGHT_AILERON = 32 ; const CONTROL_SURFACE_FLAG_RUDDER = 16 ; const CONTROL_SURFACE_FLAG_LEFT_ELEVATOR = 8 ; const CONTROL_SURFACE_FLAG_RIGHT_ELEVATOR = 4 ; const CONTROL_SURFACE_FLAG_LEFT_FLAP = 2 ; const CONTROL_SURFACE_FLAG_RIGHT_FLAP = 1 ; } }
impl Default for ControlSurfaceFlag {
    fn default() -> Self {
        ControlSurfaceFlag::CONTROL_SURFACE_FLAG_THROTTLE
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CPU_LOAD_DATA {
    pub batVolt: u16,
    pub sensLoad: u8,
    pub ctrlLoad: u8,
}
impl CPU_LOAD_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CPU_LOAD_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CPU_LOAD_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.batVolt = buf.get_u16_le();
        _struct.sensLoad = buf.get_u8();
        _struct.ctrlLoad = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.batVolt);
        _tmp.put_u8(self.sensLoad);
        _tmp.put_u8(self.ctrlLoad);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENSOR_BIAS_DATA {
    pub axBias: f32,
    pub ayBias: f32,
    pub azBias: f32,
    pub gxBias: f32,
    pub gyBias: f32,
    pub gzBias: f32,
}
impl SENSOR_BIAS_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENSOR_BIAS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENSOR_BIAS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.axBias = buf.get_f32_le();
        _struct.ayBias = buf.get_f32_le();
        _struct.azBias = buf.get_f32_le();
        _struct.gxBias = buf.get_f32_le();
        _struct.gyBias = buf.get_f32_le();
        _struct.gzBias = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.axBias);
        _tmp.put_f32_le(self.ayBias);
        _tmp.put_f32_le(self.azBias);
        _tmp.put_f32_le(self.gxBias);
        _tmp.put_f32_le(self.gyBias);
        _tmp.put_f32_le(self.gzBias);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DIAGNOSTIC_DATA {
    pub diagFl1: f32,
    pub diagFl2: f32,
    pub diagFl3: f32,
    pub diagSh1: i16,
    pub diagSh2: i16,
    pub diagSh3: i16,
}
impl DIAGNOSTIC_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DIAGNOSTIC_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DIAGNOSTIC_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.diagFl1 = buf.get_f32_le();
        _struct.diagFl2 = buf.get_f32_le();
        _struct.diagFl3 = buf.get_f32_le();
        _struct.diagSh1 = buf.get_i16_le();
        _struct.diagSh2 = buf.get_i16_le();
        _struct.diagSh3 = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.diagFl1);
        _tmp.put_f32_le(self.diagFl2);
        _tmp.put_f32_le(self.diagFl3);
        _tmp.put_i16_le(self.diagSh1);
        _tmp.put_i16_le(self.diagSh2);
        _tmp.put_i16_le(self.diagSh3);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SLUGS_NAVIGATION_DATA {
    pub u_m: f32,
    pub phi_c: f32,
    pub theta_c: f32,
    pub psiDot_c: f32,
    pub ay_body: f32,
    pub totalDist: f32,
    pub dist2Go: f32,
    pub h_c: u16,
    pub fromWP: u8,
    pub toWP: u8,
}
impl SLUGS_NAVIGATION_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SLUGS_NAVIGATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SLUGS_NAVIGATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.u_m = buf.get_f32_le();
        _struct.phi_c = buf.get_f32_le();
        _struct.theta_c = buf.get_f32_le();
        _struct.psiDot_c = buf.get_f32_le();
        _struct.ay_body = buf.get_f32_le();
        _struct.totalDist = buf.get_f32_le();
        _struct.dist2Go = buf.get_f32_le();
        _struct.h_c = buf.get_u16_le();
        _struct.fromWP = buf.get_u8();
        _struct.toWP = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.u_m);
        _tmp.put_f32_le(self.phi_c);
        _tmp.put_f32_le(self.theta_c);
        _tmp.put_f32_le(self.psiDot_c);
        _tmp.put_f32_le(self.ay_body);
        _tmp.put_f32_le(self.totalDist);
        _tmp.put_f32_le(self.dist2Go);
        _tmp.put_u16_le(self.h_c);
        _tmp.put_u8(self.fromWP);
        _tmp.put_u8(self.toWP);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA_LOG_DATA {
    pub fl_1: f32,
    pub fl_2: f32,
    pub fl_3: f32,
    pub fl_4: f32,
    pub fl_5: f32,
    pub fl_6: f32,
}
impl DATA_LOG_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA_LOG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA_LOG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.fl_1 = buf.get_f32_le();
        _struct.fl_2 = buf.get_f32_le();
        _struct.fl_3 = buf.get_f32_le();
        _struct.fl_4 = buf.get_f32_le();
        _struct.fl_5 = buf.get_f32_le();
        _struct.fl_6 = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.fl_1);
        _tmp.put_f32_le(self.fl_2);
        _tmp.put_f32_le(self.fl_3);
        _tmp.put_f32_le(self.fl_4);
        _tmp.put_f32_le(self.fl_5);
        _tmp.put_f32_le(self.fl_6);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_DATE_TIME_DATA {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub clockStat: u8,
    pub visSat: u8,
    pub useSat: u8,
    pub GppGl: u8,
    pub sigUsedMask: u8,
    pub percentUsed: u8,
}
impl GPS_DATE_TIME_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_DATE_TIME_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_DATE_TIME_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.year = buf.get_u8();
        _struct.month = buf.get_u8();
        _struct.day = buf.get_u8();
        _struct.hour = buf.get_u8();
        _struct.min = buf.get_u8();
        _struct.sec = buf.get_u8();
        _struct.clockStat = buf.get_u8();
        _struct.visSat = buf.get_u8();
        _struct.useSat = buf.get_u8();
        _struct.GppGl = buf.get_u8();
        _struct.sigUsedMask = buf.get_u8();
        _struct.percentUsed = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.year);
        _tmp.put_u8(self.month);
        _tmp.put_u8(self.day);
        _tmp.put_u8(self.hour);
        _tmp.put_u8(self.min);
        _tmp.put_u8(self.sec);
        _tmp.put_u8(self.clockStat);
        _tmp.put_u8(self.visSat);
        _tmp.put_u8(self.useSat);
        _tmp.put_u8(self.GppGl);
        _tmp.put_u8(self.sigUsedMask);
        _tmp.put_u8(self.percentUsed);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MID_LVL_CMDS_DATA {
    pub hCommand: f32,
    pub uCommand: f32,
    pub rCommand: f32,
    pub target: u8,
}
impl MID_LVL_CMDS_DATA {
    pub const ENCODED_LEN: usize = 13usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MID_LVL_CMDS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MID_LVL_CMDS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.hCommand = buf.get_f32_le();
        _struct.uCommand = buf.get_f32_le();
        _struct.rCommand = buf.get_f32_le();
        _struct.target = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.hCommand);
        _tmp.put_f32_le(self.uCommand);
        _tmp.put_f32_le(self.rCommand);
        _tmp.put_u8(self.target);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CTRL_SRFC_PT_DATA {
    pub bitfieldPt: ControlSurfaceFlag,
    pub target: u8,
}
impl CTRL_SRFC_PT_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CTRL_SRFC_PT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CTRL_SRFC_PT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u16_le();
        _struct.bitfieldPt = ControlSurfaceFlag::from_bits(tmp & ControlSurfaceFlag::all().bits())
            .ok_or(ParserError::InvalidFlag {
                flag_type: "ControlSurfaceFlag".to_string(),
                value: tmp as u32,
            })?;
        _struct.target = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.bitfieldPt.bits());
        _tmp.put_u8(self.target);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SLUGS_CAMERA_ORDER_DATA {
    pub target: u8,
    pub pan: i8,
    pub tilt: i8,
    pub zoom: i8,
    pub moveHome: i8,
}
impl SLUGS_CAMERA_ORDER_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SLUGS_CAMERA_ORDER_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SLUGS_CAMERA_ORDER_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target = buf.get_u8();
        _struct.pan = buf.get_i8();
        _struct.tilt = buf.get_i8();
        _struct.zoom = buf.get_i8();
        _struct.moveHome = buf.get_i8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target);
        _tmp.put_i8(self.pan);
        _tmp.put_i8(self.tilt);
        _tmp.put_i8(self.zoom);
        _tmp.put_i8(self.moveHome);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CONTROL_SURFACE_DATA {
    pub mControl: f32,
    pub bControl: f32,
    pub target: u8,
    pub idSurface: u8,
}
impl CONTROL_SURFACE_DATA {
    pub const ENCODED_LEN: usize = 10usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CONTROL_SURFACE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CONTROL_SURFACE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mControl = buf.get_f32_le();
        _struct.bControl = buf.get_f32_le();
        _struct.target = buf.get_u8();
        _struct.idSurface = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.mControl);
        _tmp.put_f32_le(self.bControl);
        _tmp.put_u8(self.target);
        _tmp.put_u8(self.idSurface);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SLUGS_MOBILE_LOCATION_DATA {
    pub latitude: f32,
    pub longitude: f32,
    pub target: u8,
}
impl SLUGS_MOBILE_LOCATION_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SLUGS_MOBILE_LOCATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SLUGS_MOBILE_LOCATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_f32_le();
        _struct.longitude = buf.get_f32_le();
        _struct.target = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude);
        _tmp.put_f32_le(self.longitude);
        _tmp.put_u8(self.target);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SLUGS_CONFIGURATION_CAMERA_DATA {
    pub target: u8,
    pub idOrder: u8,
    pub order: u8,
}
impl SLUGS_CONFIGURATION_CAMERA_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SLUGS_CONFIGURATION_CAMERA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SLUGS_CONFIGURATION_CAMERA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target = buf.get_u8();
        _struct.idOrder = buf.get_u8();
        _struct.order = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target);
        _tmp.put_u8(self.idOrder);
        _tmp.put_u8(self.order);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ISR_LOCATION_DATA {
    pub latitude: f32,
    pub longitude: f32,
    pub height: f32,
    pub target: u8,
    pub option1: u8,
    pub option2: u8,
    pub option3: u8,
}
impl ISR_LOCATION_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ISR_LOCATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ISR_LOCATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_f32_le();
        _struct.longitude = buf.get_f32_le();
        _struct.height = buf.get_f32_le();
        _struct.target = buf.get_u8();
        _struct.option1 = buf.get_u8();
        _struct.option2 = buf.get_u8();
        _struct.option3 = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude);
        _tmp.put_f32_le(self.longitude);
        _tmp.put_f32_le(self.height);
        _tmp.put_u8(self.target);
        _tmp.put_u8(self.option1);
        _tmp.put_u8(self.option2);
        _tmp.put_u8(self.option3);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VOLT_SENSOR_DATA {
    pub voltage: u16,
    pub reading2: u16,
    pub r2Type: u8,
}
impl VOLT_SENSOR_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VOLT_SENSOR_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VOLT_SENSOR_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.voltage = buf.get_u16_le();
        _struct.reading2 = buf.get_u16_le();
        _struct.r2Type = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.voltage);
        _tmp.put_u16_le(self.reading2);
        _tmp.put_u8(self.r2Type);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PTZ_STATUS_DATA {
    pub pan: i16,
    pub tilt: i16,
    pub zoom: u8,
}
impl PTZ_STATUS_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PTZ_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PTZ_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.pan = buf.get_i16_le();
        _struct.tilt = buf.get_i16_le();
        _struct.zoom = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.pan);
        _tmp.put_i16_le(self.tilt);
        _tmp.put_u8(self.zoom);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAV_STATUS_DATA {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub speed: f32,
    pub course: f32,
    pub target: u8,
}
impl UAV_STATUS_DATA {
    pub const ENCODED_LEN: usize = 21usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAV_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAV_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_f32_le();
        _struct.longitude = buf.get_f32_le();
        _struct.altitude = buf.get_f32_le();
        _struct.speed = buf.get_f32_le();
        _struct.course = buf.get_f32_le();
        _struct.target = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.latitude);
        _tmp.put_f32_le(self.longitude);
        _tmp.put_f32_le(self.altitude);
        _tmp.put_f32_le(self.speed);
        _tmp.put_f32_le(self.course);
        _tmp.put_u8(self.target);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct STATUS_GPS_DATA {
    pub magVar: f32,
    pub csFails: u16,
    pub gpsQuality: u8,
    pub msgsType: u8,
    pub posStatus: u8,
    pub magDir: i8,
    pub modeInd: u8,
}
impl STATUS_GPS_DATA {
    pub const ENCODED_LEN: usize = 11usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < STATUS_GPS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; STATUS_GPS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.magVar = buf.get_f32_le();
        _struct.csFails = buf.get_u16_le();
        _struct.gpsQuality = buf.get_u8();
        _struct.msgsType = buf.get_u8();
        _struct.posStatus = buf.get_u8();
        _struct.magDir = buf.get_i8();
        _struct.modeInd = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.magVar);
        _tmp.put_u16_le(self.csFails);
        _tmp.put_u8(self.gpsQuality);
        _tmp.put_u8(self.msgsType);
        _tmp.put_u8(self.posStatus);
        _tmp.put_i8(self.magDir);
        _tmp.put_u8(self.modeInd);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NOVATEL_DIAG_DATA {
    pub receiverStatus: u32,
    pub posSolAge: f32,
    pub csFails: u16,
    pub timeStatus: u8,
    pub solStatus: u8,
    pub posType: u8,
    pub velType: u8,
}
impl NOVATEL_DIAG_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < NOVATEL_DIAG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; NOVATEL_DIAG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.receiverStatus = buf.get_u32_le();
        _struct.posSolAge = buf.get_f32_le();
        _struct.csFails = buf.get_u16_le();
        _struct.timeStatus = buf.get_u8();
        _struct.solStatus = buf.get_u8();
        _struct.posType = buf.get_u8();
        _struct.velType = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.receiverStatus);
        _tmp.put_f32_le(self.posSolAge);
        _tmp.put_u16_le(self.csFails);
        _tmp.put_u8(self.timeStatus);
        _tmp.put_u8(self.solStatus);
        _tmp.put_u8(self.posType);
        _tmp.put_u8(self.velType);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENSOR_DIAG_DATA {
    pub float1: f32,
    pub float2: f32,
    pub int1: i16,
    pub char1: i8,
}
impl SENSOR_DIAG_DATA {
    pub const ENCODED_LEN: usize = 11usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENSOR_DIAG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENSOR_DIAG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.float1 = buf.get_f32_le();
        _struct.float2 = buf.get_f32_le();
        _struct.int1 = buf.get_i16_le();
        _struct.char1 = buf.get_i8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.float1);
        _tmp.put_f32_le(self.float2);
        _tmp.put_i16_le(self.int1);
        _tmp.put_i8(self.char1);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BOOT_DATA {
    pub version: u32,
}
impl BOOT_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < BOOT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; BOOT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.version = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.version);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    CPU_LOAD(CPU_LOAD_DATA),
    SENSOR_BIAS(SENSOR_BIAS_DATA),
    DIAGNOSTIC(DIAGNOSTIC_DATA),
    SLUGS_NAVIGATION(SLUGS_NAVIGATION_DATA),
    DATA_LOG(DATA_LOG_DATA),
    GPS_DATE_TIME(GPS_DATE_TIME_DATA),
    MID_LVL_CMDS(MID_LVL_CMDS_DATA),
    CTRL_SRFC_PT(CTRL_SRFC_PT_DATA),
    SLUGS_CAMERA_ORDER(SLUGS_CAMERA_ORDER_DATA),
    CONTROL_SURFACE(CONTROL_SURFACE_DATA),
    SLUGS_MOBILE_LOCATION(SLUGS_MOBILE_LOCATION_DATA),
    SLUGS_CONFIGURATION_CAMERA(SLUGS_CONFIGURATION_CAMERA_DATA),
    ISR_LOCATION(ISR_LOCATION_DATA),
    VOLT_SENSOR(VOLT_SENSOR_DATA),
    PTZ_STATUS(PTZ_STATUS_DATA),
    UAV_STATUS(UAV_STATUS_DATA),
    STATUS_GPS(STATUS_GPS_DATA),
    NOVATEL_DIAG(NOVATEL_DIAG_DATA),
    SENSOR_DIAG(SENSOR_DIAG_DATA),
    BOOT(BOOT_DATA),
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
            170 => CPU_LOAD_DATA::deser(version, payload).map(|s| MavMessage::CPU_LOAD(s)),
            172 => SENSOR_BIAS_DATA::deser(version, payload).map(|s| MavMessage::SENSOR_BIAS(s)),
            173 => DIAGNOSTIC_DATA::deser(version, payload).map(|s| MavMessage::DIAGNOSTIC(s)),
            176 => SLUGS_NAVIGATION_DATA::deser(version, payload)
                .map(|s| MavMessage::SLUGS_NAVIGATION(s)),
            177 => DATA_LOG_DATA::deser(version, payload).map(|s| MavMessage::DATA_LOG(s)),
            179 => {
                GPS_DATE_TIME_DATA::deser(version, payload).map(|s| MavMessage::GPS_DATE_TIME(s))
            }
            180 => MID_LVL_CMDS_DATA::deser(version, payload).map(|s| MavMessage::MID_LVL_CMDS(s)),
            181 => CTRL_SRFC_PT_DATA::deser(version, payload).map(|s| MavMessage::CTRL_SRFC_PT(s)),
            184 => SLUGS_CAMERA_ORDER_DATA::deser(version, payload)
                .map(|s| MavMessage::SLUGS_CAMERA_ORDER(s)),
            185 => CONTROL_SURFACE_DATA::deser(version, payload)
                .map(|s| MavMessage::CONTROL_SURFACE(s)),
            186 => SLUGS_MOBILE_LOCATION_DATA::deser(version, payload)
                .map(|s| MavMessage::SLUGS_MOBILE_LOCATION(s)),
            188 => SLUGS_CONFIGURATION_CAMERA_DATA::deser(version, payload)
                .map(|s| MavMessage::SLUGS_CONFIGURATION_CAMERA(s)),
            189 => ISR_LOCATION_DATA::deser(version, payload).map(|s| MavMessage::ISR_LOCATION(s)),
            191 => VOLT_SENSOR_DATA::deser(version, payload).map(|s| MavMessage::VOLT_SENSOR(s)),
            192 => PTZ_STATUS_DATA::deser(version, payload).map(|s| MavMessage::PTZ_STATUS(s)),
            193 => UAV_STATUS_DATA::deser(version, payload).map(|s| MavMessage::UAV_STATUS(s)),
            194 => STATUS_GPS_DATA::deser(version, payload).map(|s| MavMessage::STATUS_GPS(s)),
            195 => NOVATEL_DIAG_DATA::deser(version, payload).map(|s| MavMessage::NOVATEL_DIAG(s)),
            196 => SENSOR_DIAG_DATA::deser(version, payload).map(|s| MavMessage::SENSOR_DIAG(s)),
            197 => BOOT_DATA::deser(version, payload).map(|s| MavMessage::BOOT(s)),
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
            MavMessage::CPU_LOAD(..) => "CPU_LOAD",
            MavMessage::SENSOR_BIAS(..) => "SENSOR_BIAS",
            MavMessage::DIAGNOSTIC(..) => "DIAGNOSTIC",
            MavMessage::SLUGS_NAVIGATION(..) => "SLUGS_NAVIGATION",
            MavMessage::DATA_LOG(..) => "DATA_LOG",
            MavMessage::GPS_DATE_TIME(..) => "GPS_DATE_TIME",
            MavMessage::MID_LVL_CMDS(..) => "MID_LVL_CMDS",
            MavMessage::CTRL_SRFC_PT(..) => "CTRL_SRFC_PT",
            MavMessage::SLUGS_CAMERA_ORDER(..) => "SLUGS_CAMERA_ORDER",
            MavMessage::CONTROL_SURFACE(..) => "CONTROL_SURFACE",
            MavMessage::SLUGS_MOBILE_LOCATION(..) => "SLUGS_MOBILE_LOCATION",
            MavMessage::SLUGS_CONFIGURATION_CAMERA(..) => "SLUGS_CONFIGURATION_CAMERA",
            MavMessage::ISR_LOCATION(..) => "ISR_LOCATION",
            MavMessage::VOLT_SENSOR(..) => "VOLT_SENSOR",
            MavMessage::PTZ_STATUS(..) => "PTZ_STATUS",
            MavMessage::UAV_STATUS(..) => "UAV_STATUS",
            MavMessage::STATUS_GPS(..) => "STATUS_GPS",
            MavMessage::NOVATEL_DIAG(..) => "NOVATEL_DIAG",
            MavMessage::SENSOR_DIAG(..) => "SENSOR_DIAG",
            MavMessage::BOOT(..) => "BOOT",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::CPU_LOAD(..) => 170,
            MavMessage::SENSOR_BIAS(..) => 172,
            MavMessage::DIAGNOSTIC(..) => 173,
            MavMessage::SLUGS_NAVIGATION(..) => 176,
            MavMessage::DATA_LOG(..) => 177,
            MavMessage::GPS_DATE_TIME(..) => 179,
            MavMessage::MID_LVL_CMDS(..) => 180,
            MavMessage::CTRL_SRFC_PT(..) => 181,
            MavMessage::SLUGS_CAMERA_ORDER(..) => 184,
            MavMessage::CONTROL_SURFACE(..) => 185,
            MavMessage::SLUGS_MOBILE_LOCATION(..) => 186,
            MavMessage::SLUGS_CONFIGURATION_CAMERA(..) => 188,
            MavMessage::ISR_LOCATION(..) => 189,
            MavMessage::VOLT_SENSOR(..) => 191,
            MavMessage::PTZ_STATUS(..) => 192,
            MavMessage::UAV_STATUS(..) => 193,
            MavMessage::STATUS_GPS(..) => 194,
            MavMessage::NOVATEL_DIAG(..) => 195,
            MavMessage::SENSOR_DIAG(..) => 196,
            MavMessage::BOOT(..) => 197,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "CPU_LOAD" => Ok(170),
            "SENSOR_BIAS" => Ok(172),
            "DIAGNOSTIC" => Ok(173),
            "SLUGS_NAVIGATION" => Ok(176),
            "DATA_LOG" => Ok(177),
            "GPS_DATE_TIME" => Ok(179),
            "MID_LVL_CMDS" => Ok(180),
            "CTRL_SRFC_PT" => Ok(181),
            "SLUGS_CAMERA_ORDER" => Ok(184),
            "CONTROL_SURFACE" => Ok(185),
            "SLUGS_MOBILE_LOCATION" => Ok(186),
            "SLUGS_CONFIGURATION_CAMERA" => Ok(188),
            "ISR_LOCATION" => Ok(189),
            "VOLT_SENSOR" => Ok(191),
            "PTZ_STATUS" => Ok(192),
            "UAV_STATUS" => Ok(193),
            "STATUS_GPS" => Ok(194),
            "NOVATEL_DIAG" => Ok(195),
            "SENSOR_DIAG" => Ok(196),
            "BOOT" => Ok(197),
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
            170 => Ok(MavMessage::CPU_LOAD(CPU_LOAD_DATA::default())),
            172 => Ok(MavMessage::SENSOR_BIAS(SENSOR_BIAS_DATA::default())),
            173 => Ok(MavMessage::DIAGNOSTIC(DIAGNOSTIC_DATA::default())),
            176 => Ok(MavMessage::SLUGS_NAVIGATION(
                SLUGS_NAVIGATION_DATA::default(),
            )),
            177 => Ok(MavMessage::DATA_LOG(DATA_LOG_DATA::default())),
            179 => Ok(MavMessage::GPS_DATE_TIME(GPS_DATE_TIME_DATA::default())),
            180 => Ok(MavMessage::MID_LVL_CMDS(MID_LVL_CMDS_DATA::default())),
            181 => Ok(MavMessage::CTRL_SRFC_PT(CTRL_SRFC_PT_DATA::default())),
            184 => Ok(MavMessage::SLUGS_CAMERA_ORDER(
                SLUGS_CAMERA_ORDER_DATA::default(),
            )),
            185 => Ok(MavMessage::CONTROL_SURFACE(CONTROL_SURFACE_DATA::default())),
            186 => Ok(MavMessage::SLUGS_MOBILE_LOCATION(
                SLUGS_MOBILE_LOCATION_DATA::default(),
            )),
            188 => Ok(MavMessage::SLUGS_CONFIGURATION_CAMERA(
                SLUGS_CONFIGURATION_CAMERA_DATA::default(),
            )),
            189 => Ok(MavMessage::ISR_LOCATION(ISR_LOCATION_DATA::default())),
            191 => Ok(MavMessage::VOLT_SENSOR(VOLT_SENSOR_DATA::default())),
            192 => Ok(MavMessage::PTZ_STATUS(PTZ_STATUS_DATA::default())),
            193 => Ok(MavMessage::UAV_STATUS(UAV_STATUS_DATA::default())),
            194 => Ok(MavMessage::STATUS_GPS(STATUS_GPS_DATA::default())),
            195 => Ok(MavMessage::NOVATEL_DIAG(NOVATEL_DIAG_DATA::default())),
            196 => Ok(MavMessage::SENSOR_DIAG(SENSOR_DIAG_DATA::default())),
            197 => Ok(MavMessage::BOOT(BOOT_DATA::default())),
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
            &MavMessage::CPU_LOAD(ref body) => body.ser(),
            &MavMessage::SENSOR_BIAS(ref body) => body.ser(),
            &MavMessage::DIAGNOSTIC(ref body) => body.ser(),
            &MavMessage::SLUGS_NAVIGATION(ref body) => body.ser(),
            &MavMessage::DATA_LOG(ref body) => body.ser(),
            &MavMessage::GPS_DATE_TIME(ref body) => body.ser(),
            &MavMessage::MID_LVL_CMDS(ref body) => body.ser(),
            &MavMessage::CTRL_SRFC_PT(ref body) => body.ser(),
            &MavMessage::SLUGS_CAMERA_ORDER(ref body) => body.ser(),
            &MavMessage::CONTROL_SURFACE(ref body) => body.ser(),
            &MavMessage::SLUGS_MOBILE_LOCATION(ref body) => body.ser(),
            &MavMessage::SLUGS_CONFIGURATION_CAMERA(ref body) => body.ser(),
            &MavMessage::ISR_LOCATION(ref body) => body.ser(),
            &MavMessage::VOLT_SENSOR(ref body) => body.ser(),
            &MavMessage::PTZ_STATUS(ref body) => body.ser(),
            &MavMessage::UAV_STATUS(ref body) => body.ser(),
            &MavMessage::STATUS_GPS(ref body) => body.ser(),
            &MavMessage::NOVATEL_DIAG(ref body) => body.ser(),
            &MavMessage::SENSOR_DIAG(ref body) => body.ser(),
            &MavMessage::BOOT(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            170 => 75,
            172 => 168,
            173 => 2,
            176 => 228,
            177 => 167,
            179 => 132,
            180 => 146,
            181 => 104,
            184 => 45,
            185 => 113,
            186 => 101,
            188 => 5,
            189 => 246,
            191 => 17,
            192 => 187,
            193 => 160,
            194 => 51,
            195 => 59,
            196 => 129,
            197 => 39,
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
