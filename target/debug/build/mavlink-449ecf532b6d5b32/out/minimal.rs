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
pub enum MavAutopilot {
    MAV_AUTOPILOT_GENERIC = 0,
    MAV_AUTOPILOT_RESERVED = 1,
    MAV_AUTOPILOT_SLUGS = 2,
    MAV_AUTOPILOT_ARDUPILOTMEGA = 3,
    MAV_AUTOPILOT_OPENPILOT = 4,
    MAV_AUTOPILOT_GENERIC_WAYPOINTS_ONLY = 5,
    MAV_AUTOPILOT_GENERIC_WAYPOINTS_AND_SIMPLE_NAVIGATION_ONLY = 6,
    MAV_AUTOPILOT_GENERIC_MISSION_FULL = 7,
    MAV_AUTOPILOT_INVALID = 8,
    MAV_AUTOPILOT_PPZ = 9,
    MAV_AUTOPILOT_UDB = 10,
    MAV_AUTOPILOT_FP = 11,
    MAV_AUTOPILOT_PX4 = 12,
    MAV_AUTOPILOT_SMACCMPILOT = 13,
    MAV_AUTOPILOT_AUTOQUAD = 14,
    MAV_AUTOPILOT_ARMAZILA = 15,
    MAV_AUTOPILOT_AEROB = 16,
    MAV_AUTOPILOT_ASLUAV = 17,
    MAV_AUTOPILOT_SMARTAP = 18,
    MAV_AUTOPILOT_AIRRAILS = 19,
}
impl Default for MavAutopilot {
    fn default() -> Self {
        MavAutopilot::MAV_AUTOPILOT_GENERIC
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavType {
    MAV_TYPE_GENERIC = 0,
    MAV_TYPE_FIXED_WING = 1,
    MAV_TYPE_QUADROTOR = 2,
    MAV_TYPE_COAXIAL = 3,
    MAV_TYPE_HELICOPTER = 4,
    MAV_TYPE_ANTENNA_TRACKER = 5,
    MAV_TYPE_GCS = 6,
    MAV_TYPE_AIRSHIP = 7,
    MAV_TYPE_FREE_BALLOON = 8,
    MAV_TYPE_ROCKET = 9,
    MAV_TYPE_GROUND_ROVER = 10,
    MAV_TYPE_SURFACE_BOAT = 11,
    MAV_TYPE_SUBMARINE = 12,
    MAV_TYPE_HEXAROTOR = 13,
    MAV_TYPE_OCTOROTOR = 14,
    MAV_TYPE_TRICOPTER = 15,
    MAV_TYPE_FLAPPING_WING = 16,
    MAV_TYPE_KITE = 17,
    MAV_TYPE_ONBOARD_CONTROLLER = 18,
    MAV_TYPE_VTOL_DUOROTOR = 19,
    MAV_TYPE_VTOL_QUADROTOR = 20,
    MAV_TYPE_VTOL_TILTROTOR = 21,
    MAV_TYPE_VTOL_RESERVED2 = 22,
    MAV_TYPE_VTOL_RESERVED3 = 23,
    MAV_TYPE_VTOL_RESERVED4 = 24,
    MAV_TYPE_VTOL_RESERVED5 = 25,
    MAV_TYPE_GIMBAL = 26,
    MAV_TYPE_ADSB = 27,
    MAV_TYPE_PARAFOIL = 28,
    MAV_TYPE_DODECAROTOR = 29,
    MAV_TYPE_CAMERA = 30,
    MAV_TYPE_CHARGING_STATION = 31,
    MAV_TYPE_FLARM = 32,
    MAV_TYPE_SERVO = 33,
}
impl Default for MavType {
    fn default() -> Self {
        MavType::MAV_TYPE_GENERIC
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavModeFlag : u8 { const MAV_MODE_FLAG_SAFETY_ARMED = 128 ; const MAV_MODE_FLAG_MANUAL_INPUT_ENABLED = 64 ; const MAV_MODE_FLAG_HIL_ENABLED = 32 ; const MAV_MODE_FLAG_STABILIZE_ENABLED = 16 ; const MAV_MODE_FLAG_GUIDED_ENABLED = 8 ; const MAV_MODE_FLAG_AUTO_ENABLED = 4 ; const MAV_MODE_FLAG_TEST_ENABLED = 2 ; const MAV_MODE_FLAG_CUSTOM_MODE_ENABLED = 1 ; } }
impl Default for MavModeFlag {
    fn default() -> Self {
        MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavModeFlagDecodePosition {
    MAV_MODE_FLAG_DECODE_POSITION_SAFETY = 128,
    MAV_MODE_FLAG_DECODE_POSITION_MANUAL = 64,
    MAV_MODE_FLAG_DECODE_POSITION_HIL = 32,
    MAV_MODE_FLAG_DECODE_POSITION_STABILIZE = 16,
    MAV_MODE_FLAG_DECODE_POSITION_GUIDED = 8,
    MAV_MODE_FLAG_DECODE_POSITION_AUTO = 4,
    MAV_MODE_FLAG_DECODE_POSITION_TEST = 2,
    MAV_MODE_FLAG_DECODE_POSITION_CUSTOM_MODE = 1,
}
impl Default for MavModeFlagDecodePosition {
    fn default() -> Self {
        MavModeFlagDecodePosition::MAV_MODE_FLAG_DECODE_POSITION_SAFETY
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavState {
    MAV_STATE_UNINIT = 0,
    MAV_STATE_BOOT = 1,
    MAV_STATE_CALIBRATING = 2,
    MAV_STATE_STANDBY = 3,
    MAV_STATE_ACTIVE = 4,
    MAV_STATE_CRITICAL = 5,
    MAV_STATE_EMERGENCY = 6,
    MAV_STATE_POWEROFF = 7,
    MAV_STATE_FLIGHT_TERMINATION = 8,
}
impl Default for MavState {
    fn default() -> Self {
        MavState::MAV_STATE_UNINIT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavComponent {
    MAV_COMP_ID_ALL = 0,
    MAV_COMP_ID_AUTOPILOT1 = 1,
    MAV_COMP_ID_USER1 = 25,
    MAV_COMP_ID_USER2 = 26,
    MAV_COMP_ID_USER3 = 27,
    MAV_COMP_ID_USER4 = 28,
    MAV_COMP_ID_USER5 = 29,
    MAV_COMP_ID_USER6 = 30,
    MAV_COMP_ID_USER7 = 31,
    MAV_COMP_ID_USER8 = 32,
    MAV_COMP_ID_USER9 = 33,
    MAV_COMP_ID_USER10 = 34,
    MAV_COMP_ID_USER11 = 35,
    MAV_COMP_ID_USER12 = 36,
    MAV_COMP_ID_USER13 = 37,
    MAV_COMP_ID_USER14 = 38,
    MAV_COMP_ID_USER15 = 39,
    MAV_COMP_ID_USER16 = 40,
    MAV_COMP_ID_USER17 = 41,
    MAV_COMP_ID_USER18 = 42,
    MAV_COMP_ID_USER19 = 43,
    MAV_COMP_ID_USER20 = 44,
    MAV_COMP_ID_USER21 = 45,
    MAV_COMP_ID_USER22 = 46,
    MAV_COMP_ID_USER23 = 47,
    MAV_COMP_ID_USER24 = 48,
    MAV_COMP_ID_USER25 = 49,
    MAV_COMP_ID_USER26 = 50,
    MAV_COMP_ID_USER27 = 51,
    MAV_COMP_ID_USER28 = 52,
    MAV_COMP_ID_USER29 = 53,
    MAV_COMP_ID_USER30 = 54,
    MAV_COMP_ID_USER31 = 55,
    MAV_COMP_ID_USER32 = 56,
    MAV_COMP_ID_USER33 = 57,
    MAV_COMP_ID_USER34 = 58,
    MAV_COMP_ID_USER35 = 59,
    MAV_COMP_ID_USER36 = 60,
    MAV_COMP_ID_USER37 = 61,
    MAV_COMP_ID_USER38 = 62,
    MAV_COMP_ID_USER39 = 63,
    MAV_COMP_ID_USER40 = 64,
    MAV_COMP_ID_USER41 = 65,
    MAV_COMP_ID_USER42 = 66,
    MAV_COMP_ID_USER43 = 67,
    MAV_COMP_ID_USER44 = 68,
    MAV_COMP_ID_USER45 = 69,
    MAV_COMP_ID_USER46 = 70,
    MAV_COMP_ID_USER47 = 71,
    MAV_COMP_ID_USER48 = 72,
    MAV_COMP_ID_USER49 = 73,
    MAV_COMP_ID_USER50 = 74,
    MAV_COMP_ID_USER51 = 75,
    MAV_COMP_ID_USER52 = 76,
    MAV_COMP_ID_USER53 = 77,
    MAV_COMP_ID_USER54 = 78,
    MAV_COMP_ID_USER55 = 79,
    MAV_COMP_ID_USER56 = 80,
    MAV_COMP_ID_USER57 = 81,
    MAV_COMP_ID_USER58 = 82,
    MAV_COMP_ID_USER59 = 83,
    MAV_COMP_ID_USER60 = 84,
    MAV_COMP_ID_USER61 = 85,
    MAV_COMP_ID_USER62 = 86,
    MAV_COMP_ID_USER63 = 87,
    MAV_COMP_ID_USER64 = 88,
    MAV_COMP_ID_USER65 = 89,
    MAV_COMP_ID_USER66 = 90,
    MAV_COMP_ID_USER67 = 91,
    MAV_COMP_ID_USER68 = 92,
    MAV_COMP_ID_USER69 = 93,
    MAV_COMP_ID_USER70 = 94,
    MAV_COMP_ID_USER71 = 95,
    MAV_COMP_ID_USER72 = 96,
    MAV_COMP_ID_USER73 = 97,
    MAV_COMP_ID_USER74 = 98,
    MAV_COMP_ID_USER75 = 99,
    MAV_COMP_ID_CAMERA = 100,
    MAV_COMP_ID_CAMERA2 = 101,
    MAV_COMP_ID_CAMERA3 = 102,
    MAV_COMP_ID_CAMERA4 = 103,
    MAV_COMP_ID_CAMERA5 = 104,
    MAV_COMP_ID_CAMERA6 = 105,
    MAV_COMP_ID_SERVO1 = 140,
    MAV_COMP_ID_SERVO2 = 141,
    MAV_COMP_ID_SERVO3 = 142,
    MAV_COMP_ID_SERVO4 = 143,
    MAV_COMP_ID_SERVO5 = 144,
    MAV_COMP_ID_SERVO6 = 145,
    MAV_COMP_ID_SERVO7 = 146,
    MAV_COMP_ID_SERVO8 = 147,
    MAV_COMP_ID_SERVO9 = 148,
    MAV_COMP_ID_SERVO10 = 149,
    MAV_COMP_ID_SERVO11 = 150,
    MAV_COMP_ID_SERVO12 = 151,
    MAV_COMP_ID_SERVO13 = 152,
    MAV_COMP_ID_SERVO14 = 153,
    MAV_COMP_ID_GIMBAL = 154,
    MAV_COMP_ID_LOG = 155,
    MAV_COMP_ID_ADSB = 156,
    MAV_COMP_ID_OSD = 157,
    MAV_COMP_ID_PERIPHERAL = 158,
    MAV_COMP_ID_QX1_GIMBAL = 159,
    MAV_COMP_ID_FLARM = 160,
    MAV_COMP_ID_MISSIONPLANNER = 190,
    MAV_COMP_ID_PATHPLANNER = 195,
    MAV_COMP_ID_OBSTACLE_AVOIDANCE = 196,
    MAV_COMP_ID_VISUAL_INERTIAL_ODOMETRY = 197,
    MAV_COMP_ID_PAIRING_MANAGER = 198,
    MAV_COMP_ID_IMU = 200,
    MAV_COMP_ID_IMU_2 = 201,
    MAV_COMP_ID_IMU_3 = 202,
    MAV_COMP_ID_GPS = 220,
    MAV_COMP_ID_GPS2 = 221,
    MAV_COMP_ID_UDP_BRIDGE = 240,
    MAV_COMP_ID_UART_BRIDGE = 241,
    MAV_COMP_ID_SYSTEM_CONTROL = 250,
}
impl Default for MavComponent {
    fn default() -> Self {
        MavComponent::MAV_COMP_ID_ALL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmd {
    MAV_CMD_REQUEST_MESSAGE = 512,
    MAV_CMD_REQUEST_PROTOCOL_VERSION = 519,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_REQUEST_MESSAGE
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HEARTBEAT_DATA {
    pub custom_mode: u32,
    pub mavtype: MavType,
    pub autopilot: MavAutopilot,
    pub base_mode: MavModeFlag,
    pub system_status: MavState,
    pub mavlink_version: u8,
}
impl HEARTBEAT_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HEARTBEAT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HEARTBEAT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.custom_mode = buf.get_u32_le();
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavType".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.autopilot = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavAutopilot".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.base_mode = MavModeFlag::from_bits(tmp & MavModeFlag::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            },
        )?;
        let tmp = buf.get_u8();
        _struct.system_status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavState".to_string(),
            value: tmp as u32,
        })?;
        _struct.mavlink_version = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.base_mode.bits());
        _tmp.put_u8(self.system_status as u8);
        _tmp.put_u8(self.mavlink_version);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PROTOCOL_VERSION_DATA {
    pub version: u16,
    pub min_version: u16,
    pub max_version: u16,
    pub spec_version_hash: [u8; 8],
    pub library_version_hash: [u8; 8],
}
impl PROTOCOL_VERSION_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PROTOCOL_VERSION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PROTOCOL_VERSION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.version = buf.get_u16_le();
        _struct.min_version = buf.get_u16_le();
        _struct.max_version = buf.get_u16_le();
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.spec_version_hash[idx] = val;
        }
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.library_version_hash[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.version);
        _tmp.put_u16_le(self.min_version);
        _tmp.put_u16_le(self.max_version);
        for val in &self.spec_version_hash {
            _tmp.put_u8(*val);
        }
        for val in &self.library_version_hash {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    HEARTBEAT(HEARTBEAT_DATA),
    PROTOCOL_VERSION(PROTOCOL_VERSION_DATA),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => HEARTBEAT_DATA::deser(version, payload).map(|s| MavMessage::HEARTBEAT(s)),
            300 => PROTOCOL_VERSION_DATA::deser(version, payload)
                .map(|s| MavMessage::PROTOCOL_VERSION(s)),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::HEARTBEAT(..) => "HEARTBEAT",
            MavMessage::PROTOCOL_VERSION(..) => "PROTOCOL_VERSION",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::HEARTBEAT(..) => 0,
            MavMessage::PROTOCOL_VERSION(..) => 300,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "HEARTBEAT" => Ok(0),
            "PROTOCOL_VERSION" => Ok(300),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::HEARTBEAT(HEARTBEAT_DATA::default())),
            300 => Ok(MavMessage::PROTOCOL_VERSION(
                PROTOCOL_VERSION_DATA::default(),
            )),
            _ => {
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::HEARTBEAT(ref body) => body.ser(),
            &MavMessage::PROTOCOL_VERSION(ref body) => body.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 50,
            300 => 217,
            _ => 0,
        }
    }
}
