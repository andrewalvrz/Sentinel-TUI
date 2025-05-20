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
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct UavionixAdsbOutDynamicState : u16 { const UAVIONIX_ADSB_OUT_DYNAMIC_STATE_INTENT_CHANGE = 1 ; const UAVIONIX_ADSB_OUT_DYNAMIC_STATE_AUTOPILOT_ENABLED = 2 ; const UAVIONIX_ADSB_OUT_DYNAMIC_STATE_NICBARO_CROSSCHECKED = 4 ; const UAVIONIX_ADSB_OUT_DYNAMIC_STATE_ON_GROUND = 8 ; const UAVIONIX_ADSB_OUT_DYNAMIC_STATE_IDENT = 16 ; } }
impl Default for UavionixAdsbOutDynamicState {
    fn default() -> Self {
        UavionixAdsbOutDynamicState::UAVIONIX_ADSB_OUT_DYNAMIC_STATE_INTENT_CHANGE
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct UavionixAdsbOutRfSelect : u8 { const UAVIONIX_ADSB_OUT_RF_SELECT_STANDBY = 0 ; const UAVIONIX_ADSB_OUT_RF_SELECT_RX_ENABLED = 1 ; const UAVIONIX_ADSB_OUT_RF_SELECT_TX_ENABLED = 2 ; } }
impl Default for UavionixAdsbOutRfSelect {
    fn default() -> Self {
        UavionixAdsbOutRfSelect::UAVIONIX_ADSB_OUT_RF_SELECT_STANDBY
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavionixAdsbOutDynamicGpsFix {
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_NONE_0 = 0,
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_NONE_1 = 1,
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_2D = 2,
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_3D = 3,
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_DGPS = 4,
    UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_RTK = 5,
}
impl Default for UavionixAdsbOutDynamicGpsFix {
    fn default() -> Self {
        UavionixAdsbOutDynamicGpsFix::UAVIONIX_ADSB_OUT_DYNAMIC_GPS_FIX_NONE_0
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct UavionixAdsbRfHealth : u8 { const UAVIONIX_ADSB_RF_HEALTH_INITIALIZING = 0 ; const UAVIONIX_ADSB_RF_HEALTH_OK = 1 ; const UAVIONIX_ADSB_RF_HEALTH_FAIL_TX = 2 ; const UAVIONIX_ADSB_RF_HEALTH_FAIL_RX = 16 ; } }
impl Default for UavionixAdsbRfHealth {
    fn default() -> Self {
        UavionixAdsbRfHealth::UAVIONIX_ADSB_RF_HEALTH_INITIALIZING
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavionixAdsbOutCfgAircraftSize {
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_NO_DATA = 0,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L15M_W23M = 1,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L25M_W28P5M = 2,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L25_34M = 3,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L35_33M = 4,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L35_38M = 5,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L45_39P5M = 6,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L45_45M = 7,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L55_45M = 8,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L55_52M = 9,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L65_59P5M = 10,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L65_67M = 11,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L75_W72P5M = 12,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L75_W80M = 13,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L85_W80M = 14,
    UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_L85_W90M = 15,
}
impl Default for UavionixAdsbOutCfgAircraftSize {
    fn default() -> Self {
        UavionixAdsbOutCfgAircraftSize::UAVIONIX_ADSB_OUT_CFG_AIRCRAFT_SIZE_NO_DATA
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavionixAdsbOutCfgGpsOffsetLat {
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_NO_DATA = 0,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_LEFT_2M = 1,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_LEFT_4M = 2,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_LEFT_6M = 3,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_RIGHT_0M = 4,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_RIGHT_2M = 5,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_RIGHT_4M = 6,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_RIGHT_6M = 7,
}
impl Default for UavionixAdsbOutCfgGpsOffsetLat {
    fn default() -> Self {
        UavionixAdsbOutCfgGpsOffsetLat::UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LAT_NO_DATA
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavionixAdsbOutCfgGpsOffsetLon {
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LON_NO_DATA = 0,
    UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LON_APPLIED_BY_SENSOR = 1,
}
impl Default for UavionixAdsbOutCfgGpsOffsetLon {
    fn default() -> Self {
        UavionixAdsbOutCfgGpsOffsetLon::UAVIONIX_ADSB_OUT_CFG_GPS_OFFSET_LON_NO_DATA
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavionixAdsbEmergencyStatus {
    UAVIONIX_ADSB_OUT_NO_EMERGENCY = 0,
    UAVIONIX_ADSB_OUT_GENERAL_EMERGENCY = 1,
    UAVIONIX_ADSB_OUT_LIFEGUARD_EMERGENCY = 2,
    UAVIONIX_ADSB_OUT_MINIMUM_FUEL_EMERGENCY = 3,
    UAVIONIX_ADSB_OUT_NO_COMM_EMERGENCY = 4,
    UAVIONIX_ADSB_OUT_UNLAWFUL_INTERFERANCE_EMERGENCY = 5,
    UAVIONIX_ADSB_OUT_DOWNED_AIRCRAFT_EMERGENCY = 6,
    UAVIONIX_ADSB_OUT_RESERVED = 7,
}
impl Default for UavionixAdsbEmergencyStatus {
    fn default() -> Self {
        UavionixAdsbEmergencyStatus::UAVIONIX_ADSB_OUT_NO_EMERGENCY
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAVIONIX_ADSB_OUT_CFG_DATA {
    pub ICAO: u32,
    pub stallSpeed: u16,
    pub callsign: [char; 9],
    pub emitterType: AdsbEmitterType,
    pub aircraftSize: UavionixAdsbOutCfgAircraftSize,
    pub gpsOffsetLat: UavionixAdsbOutCfgGpsOffsetLat,
    pub gpsOffsetLon: UavionixAdsbOutCfgGpsOffsetLon,
    pub rfSelect: UavionixAdsbOutRfSelect,
}
impl UAVIONIX_ADSB_OUT_CFG_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAVIONIX_ADSB_OUT_CFG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAVIONIX_ADSB_OUT_CFG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.ICAO = buf.get_u32_le();
        _struct.stallSpeed = buf.get_u16_le();
        for idx in 0..9usize {
            let val = buf.get_u8() as char;
            _struct.callsign[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.emitterType = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "AdsbEmitterType".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.aircraftSize = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavionixAdsbOutCfgAircraftSize".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.gpsOffsetLat = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavionixAdsbOutCfgGpsOffsetLat".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.gpsOffsetLon = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavionixAdsbOutCfgGpsOffsetLon".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.rfSelect = UavionixAdsbOutRfSelect::from_bits(
            tmp & UavionixAdsbOutRfSelect::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "UavionixAdsbOutRfSelect".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ICAO);
        _tmp.put_u16_le(self.stallSpeed);
        for val in &self.callsign {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.emitterType as u8);
        _tmp.put_u8(self.aircraftSize as u8);
        _tmp.put_u8(self.gpsOffsetLat as u8);
        _tmp.put_u8(self.gpsOffsetLon as u8);
        _tmp.put_u8(self.rfSelect.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAVIONIX_ADSB_OUT_DYNAMIC_DATA {
    pub utcTime: u32,
    pub gpsLat: i32,
    pub gpsLon: i32,
    pub gpsAlt: i32,
    pub baroAltMSL: i32,
    pub accuracyHor: u32,
    pub accuracyVert: u16,
    pub accuracyVel: u16,
    pub velVert: i16,
    pub velNS: i16,
    pub VelEW: i16,
    pub state: UavionixAdsbOutDynamicState,
    pub squawk: u16,
    pub gpsFix: UavionixAdsbOutDynamicGpsFix,
    pub numSats: u8,
    pub emergencyStatus: UavionixAdsbEmergencyStatus,
}
impl UAVIONIX_ADSB_OUT_DYNAMIC_DATA {
    pub const ENCODED_LEN: usize = 41usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAVIONIX_ADSB_OUT_DYNAMIC_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAVIONIX_ADSB_OUT_DYNAMIC_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.utcTime = buf.get_u32_le();
        _struct.gpsLat = buf.get_i32_le();
        _struct.gpsLon = buf.get_i32_le();
        _struct.gpsAlt = buf.get_i32_le();
        _struct.baroAltMSL = buf.get_i32_le();
        _struct.accuracyHor = buf.get_u32_le();
        _struct.accuracyVert = buf.get_u16_le();
        _struct.accuracyVel = buf.get_u16_le();
        _struct.velVert = buf.get_i16_le();
        _struct.velNS = buf.get_i16_le();
        _struct.VelEW = buf.get_i16_le();
        let tmp = buf.get_u16_le();
        _struct.state =
            UavionixAdsbOutDynamicState::from_bits(tmp & UavionixAdsbOutDynamicState::all().bits())
                .ok_or(ParserError::InvalidFlag {
                    flag_type: "UavionixAdsbOutDynamicState".to_string(),
                    value: tmp as u32,
                })?;
        _struct.squawk = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.gpsFix = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavionixAdsbOutDynamicGpsFix".to_string(),
            value: tmp as u32,
        })?;
        _struct.numSats = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.emergencyStatus = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavionixAdsbEmergencyStatus".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.utcTime);
        _tmp.put_i32_le(self.gpsLat);
        _tmp.put_i32_le(self.gpsLon);
        _tmp.put_i32_le(self.gpsAlt);
        _tmp.put_i32_le(self.baroAltMSL);
        _tmp.put_u32_le(self.accuracyHor);
        _tmp.put_u16_le(self.accuracyVert);
        _tmp.put_u16_le(self.accuracyVel);
        _tmp.put_i16_le(self.velVert);
        _tmp.put_i16_le(self.velNS);
        _tmp.put_i16_le(self.VelEW);
        _tmp.put_u16_le(self.state.bits());
        _tmp.put_u16_le(self.squawk);
        _tmp.put_u8(self.gpsFix as u8);
        _tmp.put_u8(self.numSats);
        _tmp.put_u8(self.emergencyStatus as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA {
    pub rfHealth: UavionixAdsbRfHealth,
}
impl UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA {
    pub const ENCODED_LEN: usize = 1usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.rfHealth = UavionixAdsbRfHealth::from_bits(
            tmp & UavionixAdsbRfHealth::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "UavionixAdsbRfHealth".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.rfHealth.bits());
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    UAVIONIX_ADSB_OUT_CFG(UAVIONIX_ADSB_OUT_CFG_DATA),
    UAVIONIX_ADSB_OUT_DYNAMIC(UAVIONIX_ADSB_OUT_DYNAMIC_DATA),
    UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA),
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
            10001 => UAVIONIX_ADSB_OUT_CFG_DATA::deser(version, payload)
                .map(|s| MavMessage::UAVIONIX_ADSB_OUT_CFG(s)),
            10002 => UAVIONIX_ADSB_OUT_DYNAMIC_DATA::deser(version, payload)
                .map(|s| MavMessage::UAVIONIX_ADSB_OUT_DYNAMIC(s)),
            10003 => UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA::deser(version, payload)
                .map(|s| MavMessage::UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(s)),
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
            MavMessage::UAVIONIX_ADSB_OUT_CFG(..) => "UAVIONIX_ADSB_OUT_CFG",
            MavMessage::UAVIONIX_ADSB_OUT_DYNAMIC(..) => "UAVIONIX_ADSB_OUT_DYNAMIC",
            MavMessage::UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(..) => {
                "UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT"
            }
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::UAVIONIX_ADSB_OUT_CFG(..) => 10001,
            MavMessage::UAVIONIX_ADSB_OUT_DYNAMIC(..) => 10002,
            MavMessage::UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(..) => 10003,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "UAVIONIX_ADSB_OUT_CFG" => Ok(10001),
            "UAVIONIX_ADSB_OUT_DYNAMIC" => Ok(10002),
            "UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT" => Ok(10003),
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
            10001 => Ok(MavMessage::UAVIONIX_ADSB_OUT_CFG(
                UAVIONIX_ADSB_OUT_CFG_DATA::default(),
            )),
            10002 => Ok(MavMessage::UAVIONIX_ADSB_OUT_DYNAMIC(
                UAVIONIX_ADSB_OUT_DYNAMIC_DATA::default(),
            )),
            10003 => Ok(MavMessage::UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(
                UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT_DATA::default(),
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
            &MavMessage::UAVIONIX_ADSB_OUT_CFG(ref body) => body.ser(),
            &MavMessage::UAVIONIX_ADSB_OUT_DYNAMIC(ref body) => body.ser(),
            &MavMessage::UAVIONIX_ADSB_TRANSCEIVER_HEALTH_REPORT(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            10001 => 209,
            10002 => 186,
            10003 => 4,
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
