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
pub enum MavPreflightStorageAction {
    MAV_PFS_CMD_READ_ALL = 0,
    MAV_PFS_CMD_WRITE_ALL = 1,
    MAV_PFS_CMD_CLEAR_ALL = 2,
    MAV_PFS_CMD_READ_SPECIFIC = 3,
    MAV_PFS_CMD_WRITE_SPECIFIC = 4,
    MAV_PFS_CMD_CLEAR_SPECIFIC = 5,
    MAV_PFS_CMD_DO_NOTHING = 6,
}
impl Default for MavPreflightStorageAction {
    fn default() -> Self {
        MavPreflightStorageAction::MAV_PFS_CMD_READ_ALL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmd {
    MAV_CMD_PREFLIGHT_STORAGE_ADVANCED = 0,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_PREFLIGHT_STORAGE_ADVANCED
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_SET_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl FLEXIFUNCTION_SET_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_SET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_SET_DATA::ENCODED_LEN];
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
pub struct FLEXIFUNCTION_READ_REQ_DATA {
    pub read_req_type: i16,
    pub data_index: i16,
    pub target_system: u8,
    pub target_component: u8,
}
impl FLEXIFUNCTION_READ_REQ_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_READ_REQ_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_READ_REQ_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.read_req_type = buf.get_i16_le();
        _struct.data_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.read_req_type);
        _tmp.put_i16_le(self.data_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_BUFFER_FUNCTION_DATA {
    pub func_index: u16,
    pub func_count: u16,
    pub data_address: u16,
    pub data_size: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub data: Vec<i8>, /* 48 elements */
}
impl FLEXIFUNCTION_BUFFER_FUNCTION_DATA {
    pub const ENCODED_LEN: usize = 58usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_BUFFER_FUNCTION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_BUFFER_FUNCTION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.func_index = buf.get_u16_le();
        _struct.func_count = buf.get_u16_le();
        _struct.data_address = buf.get_u16_le();
        _struct.data_size = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..48usize {
            let val = buf.get_i8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.func_index);
        _tmp.put_u16_le(self.func_count);
        _tmp.put_u16_le(self.data_address);
        _tmp.put_u16_le(self.data_size);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.data {
            _tmp.put_i8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA {
    pub func_index: u16,
    pub result: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.func_index = buf.get_u16_le();
        _struct.result = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.func_index);
        _tmp.put_u16_le(self.result);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_DIRECTORY_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub directory_type: u8,
    pub start_index: u8,
    pub count: u8,
    pub directory_data: Vec<i8>, /* 48 elements */
}
impl FLEXIFUNCTION_DIRECTORY_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_DIRECTORY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_DIRECTORY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.directory_type = buf.get_u8();
        _struct.start_index = buf.get_u8();
        _struct.count = buf.get_u8();
        for _ in 0..48usize {
            let val = buf.get_i8();
            _struct.directory_data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.directory_type);
        _tmp.put_u8(self.start_index);
        _tmp.put_u8(self.count);
        for val in &self.directory_data {
            _tmp.put_i8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_DIRECTORY_ACK_DATA {
    pub result: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub directory_type: u8,
    pub start_index: u8,
    pub count: u8,
}
impl FLEXIFUNCTION_DIRECTORY_ACK_DATA {
    pub const ENCODED_LEN: usize = 7usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_DIRECTORY_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_DIRECTORY_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.result = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.directory_type = buf.get_u8();
        _struct.start_index = buf.get_u8();
        _struct.count = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.result);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.directory_type);
        _tmp.put_u8(self.start_index);
        _tmp.put_u8(self.count);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_COMMAND_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub command_type: u8,
}
impl FLEXIFUNCTION_COMMAND_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_COMMAND_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_COMMAND_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.command_type = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.command_type);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLEXIFUNCTION_COMMAND_ACK_DATA {
    pub command_type: u16,
    pub result: u16,
}
impl FLEXIFUNCTION_COMMAND_ACK_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLEXIFUNCTION_COMMAND_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLEXIFUNCTION_COMMAND_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.command_type = buf.get_u16_le();
        _struct.result = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command_type);
        _tmp.put_u16_le(self.result);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F2_A_DATA {
    pub sue_time: u32,
    pub sue_latitude: i32,
    pub sue_longitude: i32,
    pub sue_altitude: i32,
    pub sue_waypoint_index: u16,
    pub sue_rmat0: i16,
    pub sue_rmat1: i16,
    pub sue_rmat2: i16,
    pub sue_rmat3: i16,
    pub sue_rmat4: i16,
    pub sue_rmat5: i16,
    pub sue_rmat6: i16,
    pub sue_rmat7: i16,
    pub sue_rmat8: i16,
    pub sue_cog: u16,
    pub sue_sog: i16,
    pub sue_cpu_load: u16,
    pub sue_air_speed_3DIMU: u16,
    pub sue_estimated_wind_0: i16,
    pub sue_estimated_wind_1: i16,
    pub sue_estimated_wind_2: i16,
    pub sue_magFieldEarth0: i16,
    pub sue_magFieldEarth1: i16,
    pub sue_magFieldEarth2: i16,
    pub sue_svs: i16,
    pub sue_hdop: i16,
    pub sue_status: u8,
}
impl SERIAL_UDB_EXTRA_F2_A_DATA {
    pub const ENCODED_LEN: usize = 61usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F2_A_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F2_A_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_time = buf.get_u32_le();
        _struct.sue_latitude = buf.get_i32_le();
        _struct.sue_longitude = buf.get_i32_le();
        _struct.sue_altitude = buf.get_i32_le();
        _struct.sue_waypoint_index = buf.get_u16_le();
        _struct.sue_rmat0 = buf.get_i16_le();
        _struct.sue_rmat1 = buf.get_i16_le();
        _struct.sue_rmat2 = buf.get_i16_le();
        _struct.sue_rmat3 = buf.get_i16_le();
        _struct.sue_rmat4 = buf.get_i16_le();
        _struct.sue_rmat5 = buf.get_i16_le();
        _struct.sue_rmat6 = buf.get_i16_le();
        _struct.sue_rmat7 = buf.get_i16_le();
        _struct.sue_rmat8 = buf.get_i16_le();
        _struct.sue_cog = buf.get_u16_le();
        _struct.sue_sog = buf.get_i16_le();
        _struct.sue_cpu_load = buf.get_u16_le();
        _struct.sue_air_speed_3DIMU = buf.get_u16_le();
        _struct.sue_estimated_wind_0 = buf.get_i16_le();
        _struct.sue_estimated_wind_1 = buf.get_i16_le();
        _struct.sue_estimated_wind_2 = buf.get_i16_le();
        _struct.sue_magFieldEarth0 = buf.get_i16_le();
        _struct.sue_magFieldEarth1 = buf.get_i16_le();
        _struct.sue_magFieldEarth2 = buf.get_i16_le();
        _struct.sue_svs = buf.get_i16_le();
        _struct.sue_hdop = buf.get_i16_le();
        _struct.sue_status = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_time);
        _tmp.put_i32_le(self.sue_latitude);
        _tmp.put_i32_le(self.sue_longitude);
        _tmp.put_i32_le(self.sue_altitude);
        _tmp.put_u16_le(self.sue_waypoint_index);
        _tmp.put_i16_le(self.sue_rmat0);
        _tmp.put_i16_le(self.sue_rmat1);
        _tmp.put_i16_le(self.sue_rmat2);
        _tmp.put_i16_le(self.sue_rmat3);
        _tmp.put_i16_le(self.sue_rmat4);
        _tmp.put_i16_le(self.sue_rmat5);
        _tmp.put_i16_le(self.sue_rmat6);
        _tmp.put_i16_le(self.sue_rmat7);
        _tmp.put_i16_le(self.sue_rmat8);
        _tmp.put_u16_le(self.sue_cog);
        _tmp.put_i16_le(self.sue_sog);
        _tmp.put_u16_le(self.sue_cpu_load);
        _tmp.put_u16_le(self.sue_air_speed_3DIMU);
        _tmp.put_i16_le(self.sue_estimated_wind_0);
        _tmp.put_i16_le(self.sue_estimated_wind_1);
        _tmp.put_i16_le(self.sue_estimated_wind_2);
        _tmp.put_i16_le(self.sue_magFieldEarth0);
        _tmp.put_i16_le(self.sue_magFieldEarth1);
        _tmp.put_i16_le(self.sue_magFieldEarth2);
        _tmp.put_i16_le(self.sue_svs);
        _tmp.put_i16_le(self.sue_hdop);
        _tmp.put_u8(self.sue_status);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F2_B_DATA {
    pub sue_time: u32,
    pub sue_flags: u32,
    pub sue_barom_press: i32,
    pub sue_barom_alt: i32,
    pub sue_pwm_input_1: i16,
    pub sue_pwm_input_2: i16,
    pub sue_pwm_input_3: i16,
    pub sue_pwm_input_4: i16,
    pub sue_pwm_input_5: i16,
    pub sue_pwm_input_6: i16,
    pub sue_pwm_input_7: i16,
    pub sue_pwm_input_8: i16,
    pub sue_pwm_input_9: i16,
    pub sue_pwm_input_10: i16,
    pub sue_pwm_input_11: i16,
    pub sue_pwm_input_12: i16,
    pub sue_pwm_output_1: i16,
    pub sue_pwm_output_2: i16,
    pub sue_pwm_output_3: i16,
    pub sue_pwm_output_4: i16,
    pub sue_pwm_output_5: i16,
    pub sue_pwm_output_6: i16,
    pub sue_pwm_output_7: i16,
    pub sue_pwm_output_8: i16,
    pub sue_pwm_output_9: i16,
    pub sue_pwm_output_10: i16,
    pub sue_pwm_output_11: i16,
    pub sue_pwm_output_12: i16,
    pub sue_imu_location_x: i16,
    pub sue_imu_location_y: i16,
    pub sue_imu_location_z: i16,
    pub sue_location_error_earth_x: i16,
    pub sue_location_error_earth_y: i16,
    pub sue_location_error_earth_z: i16,
    pub sue_osc_fails: i16,
    pub sue_imu_velocity_x: i16,
    pub sue_imu_velocity_y: i16,
    pub sue_imu_velocity_z: i16,
    pub sue_waypoint_goal_x: i16,
    pub sue_waypoint_goal_y: i16,
    pub sue_waypoint_goal_z: i16,
    pub sue_aero_x: i16,
    pub sue_aero_y: i16,
    pub sue_aero_z: i16,
    pub sue_barom_temp: i16,
    pub sue_bat_volt: i16,
    pub sue_bat_amp: i16,
    pub sue_bat_amp_hours: i16,
    pub sue_desired_height: i16,
    pub sue_memory_stack_free: i16,
}
impl SERIAL_UDB_EXTRA_F2_B_DATA {
    pub const ENCODED_LEN: usize = 108usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F2_B_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F2_B_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_time = buf.get_u32_le();
        _struct.sue_flags = buf.get_u32_le();
        _struct.sue_barom_press = buf.get_i32_le();
        _struct.sue_barom_alt = buf.get_i32_le();
        _struct.sue_pwm_input_1 = buf.get_i16_le();
        _struct.sue_pwm_input_2 = buf.get_i16_le();
        _struct.sue_pwm_input_3 = buf.get_i16_le();
        _struct.sue_pwm_input_4 = buf.get_i16_le();
        _struct.sue_pwm_input_5 = buf.get_i16_le();
        _struct.sue_pwm_input_6 = buf.get_i16_le();
        _struct.sue_pwm_input_7 = buf.get_i16_le();
        _struct.sue_pwm_input_8 = buf.get_i16_le();
        _struct.sue_pwm_input_9 = buf.get_i16_le();
        _struct.sue_pwm_input_10 = buf.get_i16_le();
        _struct.sue_pwm_input_11 = buf.get_i16_le();
        _struct.sue_pwm_input_12 = buf.get_i16_le();
        _struct.sue_pwm_output_1 = buf.get_i16_le();
        _struct.sue_pwm_output_2 = buf.get_i16_le();
        _struct.sue_pwm_output_3 = buf.get_i16_le();
        _struct.sue_pwm_output_4 = buf.get_i16_le();
        _struct.sue_pwm_output_5 = buf.get_i16_le();
        _struct.sue_pwm_output_6 = buf.get_i16_le();
        _struct.sue_pwm_output_7 = buf.get_i16_le();
        _struct.sue_pwm_output_8 = buf.get_i16_le();
        _struct.sue_pwm_output_9 = buf.get_i16_le();
        _struct.sue_pwm_output_10 = buf.get_i16_le();
        _struct.sue_pwm_output_11 = buf.get_i16_le();
        _struct.sue_pwm_output_12 = buf.get_i16_le();
        _struct.sue_imu_location_x = buf.get_i16_le();
        _struct.sue_imu_location_y = buf.get_i16_le();
        _struct.sue_imu_location_z = buf.get_i16_le();
        _struct.sue_location_error_earth_x = buf.get_i16_le();
        _struct.sue_location_error_earth_y = buf.get_i16_le();
        _struct.sue_location_error_earth_z = buf.get_i16_le();
        _struct.sue_osc_fails = buf.get_i16_le();
        _struct.sue_imu_velocity_x = buf.get_i16_le();
        _struct.sue_imu_velocity_y = buf.get_i16_le();
        _struct.sue_imu_velocity_z = buf.get_i16_le();
        _struct.sue_waypoint_goal_x = buf.get_i16_le();
        _struct.sue_waypoint_goal_y = buf.get_i16_le();
        _struct.sue_waypoint_goal_z = buf.get_i16_le();
        _struct.sue_aero_x = buf.get_i16_le();
        _struct.sue_aero_y = buf.get_i16_le();
        _struct.sue_aero_z = buf.get_i16_le();
        _struct.sue_barom_temp = buf.get_i16_le();
        _struct.sue_bat_volt = buf.get_i16_le();
        _struct.sue_bat_amp = buf.get_i16_le();
        _struct.sue_bat_amp_hours = buf.get_i16_le();
        _struct.sue_desired_height = buf.get_i16_le();
        _struct.sue_memory_stack_free = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_time);
        _tmp.put_u32_le(self.sue_flags);
        _tmp.put_i32_le(self.sue_barom_press);
        _tmp.put_i32_le(self.sue_barom_alt);
        _tmp.put_i16_le(self.sue_pwm_input_1);
        _tmp.put_i16_le(self.sue_pwm_input_2);
        _tmp.put_i16_le(self.sue_pwm_input_3);
        _tmp.put_i16_le(self.sue_pwm_input_4);
        _tmp.put_i16_le(self.sue_pwm_input_5);
        _tmp.put_i16_le(self.sue_pwm_input_6);
        _tmp.put_i16_le(self.sue_pwm_input_7);
        _tmp.put_i16_le(self.sue_pwm_input_8);
        _tmp.put_i16_le(self.sue_pwm_input_9);
        _tmp.put_i16_le(self.sue_pwm_input_10);
        _tmp.put_i16_le(self.sue_pwm_input_11);
        _tmp.put_i16_le(self.sue_pwm_input_12);
        _tmp.put_i16_le(self.sue_pwm_output_1);
        _tmp.put_i16_le(self.sue_pwm_output_2);
        _tmp.put_i16_le(self.sue_pwm_output_3);
        _tmp.put_i16_le(self.sue_pwm_output_4);
        _tmp.put_i16_le(self.sue_pwm_output_5);
        _tmp.put_i16_le(self.sue_pwm_output_6);
        _tmp.put_i16_le(self.sue_pwm_output_7);
        _tmp.put_i16_le(self.sue_pwm_output_8);
        _tmp.put_i16_le(self.sue_pwm_output_9);
        _tmp.put_i16_le(self.sue_pwm_output_10);
        _tmp.put_i16_le(self.sue_pwm_output_11);
        _tmp.put_i16_le(self.sue_pwm_output_12);
        _tmp.put_i16_le(self.sue_imu_location_x);
        _tmp.put_i16_le(self.sue_imu_location_y);
        _tmp.put_i16_le(self.sue_imu_location_z);
        _tmp.put_i16_le(self.sue_location_error_earth_x);
        _tmp.put_i16_le(self.sue_location_error_earth_y);
        _tmp.put_i16_le(self.sue_location_error_earth_z);
        _tmp.put_i16_le(self.sue_osc_fails);
        _tmp.put_i16_le(self.sue_imu_velocity_x);
        _tmp.put_i16_le(self.sue_imu_velocity_y);
        _tmp.put_i16_le(self.sue_imu_velocity_z);
        _tmp.put_i16_le(self.sue_waypoint_goal_x);
        _tmp.put_i16_le(self.sue_waypoint_goal_y);
        _tmp.put_i16_le(self.sue_waypoint_goal_z);
        _tmp.put_i16_le(self.sue_aero_x);
        _tmp.put_i16_le(self.sue_aero_y);
        _tmp.put_i16_le(self.sue_aero_z);
        _tmp.put_i16_le(self.sue_barom_temp);
        _tmp.put_i16_le(self.sue_bat_volt);
        _tmp.put_i16_le(self.sue_bat_amp);
        _tmp.put_i16_le(self.sue_bat_amp_hours);
        _tmp.put_i16_le(self.sue_desired_height);
        _tmp.put_i16_le(self.sue_memory_stack_free);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F4_DATA {
    pub sue_ROLL_STABILIZATION_AILERONS: u8,
    pub sue_ROLL_STABILIZATION_RUDDER: u8,
    pub sue_PITCH_STABILIZATION: u8,
    pub sue_YAW_STABILIZATION_RUDDER: u8,
    pub sue_YAW_STABILIZATION_AILERON: u8,
    pub sue_AILERON_NAVIGATION: u8,
    pub sue_RUDDER_NAVIGATION: u8,
    pub sue_ALTITUDEHOLD_STABILIZED: u8,
    pub sue_ALTITUDEHOLD_WAYPOINT: u8,
    pub sue_RACING_MODE: u8,
}
impl SERIAL_UDB_EXTRA_F4_DATA {
    pub const ENCODED_LEN: usize = 10usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F4_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F4_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_ROLL_STABILIZATION_AILERONS = buf.get_u8();
        _struct.sue_ROLL_STABILIZATION_RUDDER = buf.get_u8();
        _struct.sue_PITCH_STABILIZATION = buf.get_u8();
        _struct.sue_YAW_STABILIZATION_RUDDER = buf.get_u8();
        _struct.sue_YAW_STABILIZATION_AILERON = buf.get_u8();
        _struct.sue_AILERON_NAVIGATION = buf.get_u8();
        _struct.sue_RUDDER_NAVIGATION = buf.get_u8();
        _struct.sue_ALTITUDEHOLD_STABILIZED = buf.get_u8();
        _struct.sue_ALTITUDEHOLD_WAYPOINT = buf.get_u8();
        _struct.sue_RACING_MODE = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.sue_ROLL_STABILIZATION_AILERONS);
        _tmp.put_u8(self.sue_ROLL_STABILIZATION_RUDDER);
        _tmp.put_u8(self.sue_PITCH_STABILIZATION);
        _tmp.put_u8(self.sue_YAW_STABILIZATION_RUDDER);
        _tmp.put_u8(self.sue_YAW_STABILIZATION_AILERON);
        _tmp.put_u8(self.sue_AILERON_NAVIGATION);
        _tmp.put_u8(self.sue_RUDDER_NAVIGATION);
        _tmp.put_u8(self.sue_ALTITUDEHOLD_STABILIZED);
        _tmp.put_u8(self.sue_ALTITUDEHOLD_WAYPOINT);
        _tmp.put_u8(self.sue_RACING_MODE);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F5_DATA {
    pub sue_YAWKP_AILERON: f32,
    pub sue_YAWKD_AILERON: f32,
    pub sue_ROLLKP: f32,
    pub sue_ROLLKD: f32,
}
impl SERIAL_UDB_EXTRA_F5_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F5_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F5_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_YAWKP_AILERON = buf.get_f32_le();
        _struct.sue_YAWKD_AILERON = buf.get_f32_le();
        _struct.sue_ROLLKP = buf.get_f32_le();
        _struct.sue_ROLLKD = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_YAWKP_AILERON);
        _tmp.put_f32_le(self.sue_YAWKD_AILERON);
        _tmp.put_f32_le(self.sue_ROLLKP);
        _tmp.put_f32_le(self.sue_ROLLKD);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F6_DATA {
    pub sue_PITCHGAIN: f32,
    pub sue_PITCHKD: f32,
    pub sue_RUDDER_ELEV_MIX: f32,
    pub sue_ROLL_ELEV_MIX: f32,
    pub sue_ELEVATOR_BOOST: f32,
}
impl SERIAL_UDB_EXTRA_F6_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F6_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F6_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_PITCHGAIN = buf.get_f32_le();
        _struct.sue_PITCHKD = buf.get_f32_le();
        _struct.sue_RUDDER_ELEV_MIX = buf.get_f32_le();
        _struct.sue_ROLL_ELEV_MIX = buf.get_f32_le();
        _struct.sue_ELEVATOR_BOOST = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_PITCHGAIN);
        _tmp.put_f32_le(self.sue_PITCHKD);
        _tmp.put_f32_le(self.sue_RUDDER_ELEV_MIX);
        _tmp.put_f32_le(self.sue_ROLL_ELEV_MIX);
        _tmp.put_f32_le(self.sue_ELEVATOR_BOOST);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F7_DATA {
    pub sue_YAWKP_RUDDER: f32,
    pub sue_YAWKD_RUDDER: f32,
    pub sue_ROLLKP_RUDDER: f32,
    pub sue_ROLLKD_RUDDER: f32,
    pub sue_RUDDER_BOOST: f32,
    pub sue_RTL_PITCH_DOWN: f32,
}
impl SERIAL_UDB_EXTRA_F7_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F7_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F7_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_YAWKP_RUDDER = buf.get_f32_le();
        _struct.sue_YAWKD_RUDDER = buf.get_f32_le();
        _struct.sue_ROLLKP_RUDDER = buf.get_f32_le();
        _struct.sue_ROLLKD_RUDDER = buf.get_f32_le();
        _struct.sue_RUDDER_BOOST = buf.get_f32_le();
        _struct.sue_RTL_PITCH_DOWN = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_YAWKP_RUDDER);
        _tmp.put_f32_le(self.sue_YAWKD_RUDDER);
        _tmp.put_f32_le(self.sue_ROLLKP_RUDDER);
        _tmp.put_f32_le(self.sue_ROLLKD_RUDDER);
        _tmp.put_f32_le(self.sue_RUDDER_BOOST);
        _tmp.put_f32_le(self.sue_RTL_PITCH_DOWN);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F8_DATA {
    pub sue_HEIGHT_TARGET_MAX: f32,
    pub sue_HEIGHT_TARGET_MIN: f32,
    pub sue_ALT_HOLD_THROTTLE_MIN: f32,
    pub sue_ALT_HOLD_THROTTLE_MAX: f32,
    pub sue_ALT_HOLD_PITCH_MIN: f32,
    pub sue_ALT_HOLD_PITCH_MAX: f32,
    pub sue_ALT_HOLD_PITCH_HIGH: f32,
}
impl SERIAL_UDB_EXTRA_F8_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F8_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F8_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_HEIGHT_TARGET_MAX = buf.get_f32_le();
        _struct.sue_HEIGHT_TARGET_MIN = buf.get_f32_le();
        _struct.sue_ALT_HOLD_THROTTLE_MIN = buf.get_f32_le();
        _struct.sue_ALT_HOLD_THROTTLE_MAX = buf.get_f32_le();
        _struct.sue_ALT_HOLD_PITCH_MIN = buf.get_f32_le();
        _struct.sue_ALT_HOLD_PITCH_MAX = buf.get_f32_le();
        _struct.sue_ALT_HOLD_PITCH_HIGH = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_HEIGHT_TARGET_MAX);
        _tmp.put_f32_le(self.sue_HEIGHT_TARGET_MIN);
        _tmp.put_f32_le(self.sue_ALT_HOLD_THROTTLE_MIN);
        _tmp.put_f32_le(self.sue_ALT_HOLD_THROTTLE_MAX);
        _tmp.put_f32_le(self.sue_ALT_HOLD_PITCH_MIN);
        _tmp.put_f32_le(self.sue_ALT_HOLD_PITCH_MAX);
        _tmp.put_f32_le(self.sue_ALT_HOLD_PITCH_HIGH);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F13_DATA {
    pub sue_lat_origin: i32,
    pub sue_lon_origin: i32,
    pub sue_alt_origin: i32,
    pub sue_week_no: i16,
}
impl SERIAL_UDB_EXTRA_F13_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F13_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F13_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_lat_origin = buf.get_i32_le();
        _struct.sue_lon_origin = buf.get_i32_le();
        _struct.sue_alt_origin = buf.get_i32_le();
        _struct.sue_week_no = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.sue_lat_origin);
        _tmp.put_i32_le(self.sue_lon_origin);
        _tmp.put_i32_le(self.sue_alt_origin);
        _tmp.put_i16_le(self.sue_week_no);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F14_DATA {
    pub sue_TRAP_SOURCE: u32,
    pub sue_RCON: i16,
    pub sue_TRAP_FLAGS: i16,
    pub sue_osc_fail_count: i16,
    pub sue_WIND_ESTIMATION: u8,
    pub sue_GPS_TYPE: u8,
    pub sue_DR: u8,
    pub sue_BOARD_TYPE: u8,
    pub sue_AIRFRAME: u8,
    pub sue_CLOCK_CONFIG: u8,
    pub sue_FLIGHT_PLAN_TYPE: u8,
}
impl SERIAL_UDB_EXTRA_F14_DATA {
    pub const ENCODED_LEN: usize = 17usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F14_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F14_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_TRAP_SOURCE = buf.get_u32_le();
        _struct.sue_RCON = buf.get_i16_le();
        _struct.sue_TRAP_FLAGS = buf.get_i16_le();
        _struct.sue_osc_fail_count = buf.get_i16_le();
        _struct.sue_WIND_ESTIMATION = buf.get_u8();
        _struct.sue_GPS_TYPE = buf.get_u8();
        _struct.sue_DR = buf.get_u8();
        _struct.sue_BOARD_TYPE = buf.get_u8();
        _struct.sue_AIRFRAME = buf.get_u8();
        _struct.sue_CLOCK_CONFIG = buf.get_u8();
        _struct.sue_FLIGHT_PLAN_TYPE = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.sue_TRAP_SOURCE);
        _tmp.put_i16_le(self.sue_RCON);
        _tmp.put_i16_le(self.sue_TRAP_FLAGS);
        _tmp.put_i16_le(self.sue_osc_fail_count);
        _tmp.put_u8(self.sue_WIND_ESTIMATION);
        _tmp.put_u8(self.sue_GPS_TYPE);
        _tmp.put_u8(self.sue_DR);
        _tmp.put_u8(self.sue_BOARD_TYPE);
        _tmp.put_u8(self.sue_AIRFRAME);
        _tmp.put_u8(self.sue_CLOCK_CONFIG);
        _tmp.put_u8(self.sue_FLIGHT_PLAN_TYPE);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F15_DATA {
    pub sue_ID_VEHICLE_MODEL_NAME: Vec<u8>, /* 40 elements */
    pub sue_ID_VEHICLE_REGISTRATION: [u8; 20],
}
impl SERIAL_UDB_EXTRA_F15_DATA {
    pub const ENCODED_LEN: usize = 60usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F15_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F15_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for _ in 0..40usize {
            let val = buf.get_u8();
            _struct.sue_ID_VEHICLE_MODEL_NAME.push(val);
        }
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.sue_ID_VEHICLE_REGISTRATION[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.sue_ID_VEHICLE_MODEL_NAME {
            _tmp.put_u8(*val);
        }
        for val in &self.sue_ID_VEHICLE_REGISTRATION {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F16_DATA {
    pub sue_ID_LEAD_PILOT: Vec<u8>,     /* 40 elements */
    pub sue_ID_DIY_DRONES_URL: Vec<u8>, /* 70 elements */
}
impl SERIAL_UDB_EXTRA_F16_DATA {
    pub const ENCODED_LEN: usize = 110usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F16_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F16_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for _ in 0..40usize {
            let val = buf.get_u8();
            _struct.sue_ID_LEAD_PILOT.push(val);
        }
        for _ in 0..70usize {
            let val = buf.get_u8();
            _struct.sue_ID_DIY_DRONES_URL.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.sue_ID_LEAD_PILOT {
            _tmp.put_u8(*val);
        }
        for val in &self.sue_ID_DIY_DRONES_URL {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ALTITUDES_DATA {
    pub time_boot_ms: u32,
    pub alt_gps: i32,
    pub alt_imu: i32,
    pub alt_barometric: i32,
    pub alt_optical_flow: i32,
    pub alt_range_finder: i32,
    pub alt_extra: i32,
}
impl ALTITUDES_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ALTITUDES_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ALTITUDES_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.alt_gps = buf.get_i32_le();
        _struct.alt_imu = buf.get_i32_le();
        _struct.alt_barometric = buf.get_i32_le();
        _struct.alt_optical_flow = buf.get_i32_le();
        _struct.alt_range_finder = buf.get_i32_le();
        _struct.alt_extra = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.alt_gps);
        _tmp.put_i32_le(self.alt_imu);
        _tmp.put_i32_le(self.alt_barometric);
        _tmp.put_i32_le(self.alt_optical_flow);
        _tmp.put_i32_le(self.alt_range_finder);
        _tmp.put_i32_le(self.alt_extra);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AIRSPEEDS_DATA {
    pub time_boot_ms: u32,
    pub airspeed_imu: i16,
    pub airspeed_pitot: i16,
    pub airspeed_hot_wire: i16,
    pub airspeed_ultrasonic: i16,
    pub aoa: i16,
    pub aoy: i16,
}
impl AIRSPEEDS_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AIRSPEEDS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AIRSPEEDS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.airspeed_imu = buf.get_i16_le();
        _struct.airspeed_pitot = buf.get_i16_le();
        _struct.airspeed_hot_wire = buf.get_i16_le();
        _struct.airspeed_ultrasonic = buf.get_i16_le();
        _struct.aoa = buf.get_i16_le();
        _struct.aoy = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i16_le(self.airspeed_imu);
        _tmp.put_i16_le(self.airspeed_pitot);
        _tmp.put_i16_le(self.airspeed_hot_wire);
        _tmp.put_i16_le(self.airspeed_ultrasonic);
        _tmp.put_i16_le(self.aoa);
        _tmp.put_i16_le(self.aoy);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F17_DATA {
    pub sue_feed_forward: f32,
    pub sue_turn_rate_nav: f32,
    pub sue_turn_rate_fbw: f32,
}
impl SERIAL_UDB_EXTRA_F17_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F17_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F17_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_feed_forward = buf.get_f32_le();
        _struct.sue_turn_rate_nav = buf.get_f32_le();
        _struct.sue_turn_rate_fbw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.sue_feed_forward);
        _tmp.put_f32_le(self.sue_turn_rate_nav);
        _tmp.put_f32_le(self.sue_turn_rate_fbw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F18_DATA {
    pub angle_of_attack_normal: f32,
    pub angle_of_attack_inverted: f32,
    pub elevator_trim_normal: f32,
    pub elevator_trim_inverted: f32,
    pub reference_speed: f32,
}
impl SERIAL_UDB_EXTRA_F18_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F18_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F18_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.angle_of_attack_normal = buf.get_f32_le();
        _struct.angle_of_attack_inverted = buf.get_f32_le();
        _struct.elevator_trim_normal = buf.get_f32_le();
        _struct.elevator_trim_inverted = buf.get_f32_le();
        _struct.reference_speed = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.angle_of_attack_normal);
        _tmp.put_f32_le(self.angle_of_attack_inverted);
        _tmp.put_f32_le(self.elevator_trim_normal);
        _tmp.put_f32_le(self.elevator_trim_inverted);
        _tmp.put_f32_le(self.reference_speed);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F19_DATA {
    pub sue_aileron_output_channel: u8,
    pub sue_aileron_reversed: u8,
    pub sue_elevator_output_channel: u8,
    pub sue_elevator_reversed: u8,
    pub sue_throttle_output_channel: u8,
    pub sue_throttle_reversed: u8,
    pub sue_rudder_output_channel: u8,
    pub sue_rudder_reversed: u8,
}
impl SERIAL_UDB_EXTRA_F19_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F19_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F19_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_aileron_output_channel = buf.get_u8();
        _struct.sue_aileron_reversed = buf.get_u8();
        _struct.sue_elevator_output_channel = buf.get_u8();
        _struct.sue_elevator_reversed = buf.get_u8();
        _struct.sue_throttle_output_channel = buf.get_u8();
        _struct.sue_throttle_reversed = buf.get_u8();
        _struct.sue_rudder_output_channel = buf.get_u8();
        _struct.sue_rudder_reversed = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.sue_aileron_output_channel);
        _tmp.put_u8(self.sue_aileron_reversed);
        _tmp.put_u8(self.sue_elevator_output_channel);
        _tmp.put_u8(self.sue_elevator_reversed);
        _tmp.put_u8(self.sue_throttle_output_channel);
        _tmp.put_u8(self.sue_throttle_reversed);
        _tmp.put_u8(self.sue_rudder_output_channel);
        _tmp.put_u8(self.sue_rudder_reversed);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F20_DATA {
    pub sue_trim_value_input_1: i16,
    pub sue_trim_value_input_2: i16,
    pub sue_trim_value_input_3: i16,
    pub sue_trim_value_input_4: i16,
    pub sue_trim_value_input_5: i16,
    pub sue_trim_value_input_6: i16,
    pub sue_trim_value_input_7: i16,
    pub sue_trim_value_input_8: i16,
    pub sue_trim_value_input_9: i16,
    pub sue_trim_value_input_10: i16,
    pub sue_trim_value_input_11: i16,
    pub sue_trim_value_input_12: i16,
    pub sue_number_of_inputs: u8,
}
impl SERIAL_UDB_EXTRA_F20_DATA {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F20_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F20_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_trim_value_input_1 = buf.get_i16_le();
        _struct.sue_trim_value_input_2 = buf.get_i16_le();
        _struct.sue_trim_value_input_3 = buf.get_i16_le();
        _struct.sue_trim_value_input_4 = buf.get_i16_le();
        _struct.sue_trim_value_input_5 = buf.get_i16_le();
        _struct.sue_trim_value_input_6 = buf.get_i16_le();
        _struct.sue_trim_value_input_7 = buf.get_i16_le();
        _struct.sue_trim_value_input_8 = buf.get_i16_le();
        _struct.sue_trim_value_input_9 = buf.get_i16_le();
        _struct.sue_trim_value_input_10 = buf.get_i16_le();
        _struct.sue_trim_value_input_11 = buf.get_i16_le();
        _struct.sue_trim_value_input_12 = buf.get_i16_le();
        _struct.sue_number_of_inputs = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_trim_value_input_1);
        _tmp.put_i16_le(self.sue_trim_value_input_2);
        _tmp.put_i16_le(self.sue_trim_value_input_3);
        _tmp.put_i16_le(self.sue_trim_value_input_4);
        _tmp.put_i16_le(self.sue_trim_value_input_5);
        _tmp.put_i16_le(self.sue_trim_value_input_6);
        _tmp.put_i16_le(self.sue_trim_value_input_7);
        _tmp.put_i16_le(self.sue_trim_value_input_8);
        _tmp.put_i16_le(self.sue_trim_value_input_9);
        _tmp.put_i16_le(self.sue_trim_value_input_10);
        _tmp.put_i16_le(self.sue_trim_value_input_11);
        _tmp.put_i16_le(self.sue_trim_value_input_12);
        _tmp.put_u8(self.sue_number_of_inputs);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F21_DATA {
    pub sue_accel_x_offset: i16,
    pub sue_accel_y_offset: i16,
    pub sue_accel_z_offset: i16,
    pub sue_gyro_x_offset: i16,
    pub sue_gyro_y_offset: i16,
    pub sue_gyro_z_offset: i16,
}
impl SERIAL_UDB_EXTRA_F21_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F21_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F21_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_accel_x_offset = buf.get_i16_le();
        _struct.sue_accel_y_offset = buf.get_i16_le();
        _struct.sue_accel_z_offset = buf.get_i16_le();
        _struct.sue_gyro_x_offset = buf.get_i16_le();
        _struct.sue_gyro_y_offset = buf.get_i16_le();
        _struct.sue_gyro_z_offset = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_accel_x_offset);
        _tmp.put_i16_le(self.sue_accel_y_offset);
        _tmp.put_i16_le(self.sue_accel_z_offset);
        _tmp.put_i16_le(self.sue_gyro_x_offset);
        _tmp.put_i16_le(self.sue_gyro_y_offset);
        _tmp.put_i16_le(self.sue_gyro_z_offset);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_UDB_EXTRA_F22_DATA {
    pub sue_accel_x_at_calibration: i16,
    pub sue_accel_y_at_calibration: i16,
    pub sue_accel_z_at_calibration: i16,
    pub sue_gyro_x_at_calibration: i16,
    pub sue_gyro_y_at_calibration: i16,
    pub sue_gyro_z_at_calibration: i16,
}
impl SERIAL_UDB_EXTRA_F22_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_UDB_EXTRA_F22_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_UDB_EXTRA_F22_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sue_accel_x_at_calibration = buf.get_i16_le();
        _struct.sue_accel_y_at_calibration = buf.get_i16_le();
        _struct.sue_accel_z_at_calibration = buf.get_i16_le();
        _struct.sue_gyro_x_at_calibration = buf.get_i16_le();
        _struct.sue_gyro_y_at_calibration = buf.get_i16_le();
        _struct.sue_gyro_z_at_calibration = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.sue_accel_x_at_calibration);
        _tmp.put_i16_le(self.sue_accel_y_at_calibration);
        _tmp.put_i16_le(self.sue_accel_z_at_calibration);
        _tmp.put_i16_le(self.sue_gyro_x_at_calibration);
        _tmp.put_i16_le(self.sue_gyro_y_at_calibration);
        _tmp.put_i16_le(self.sue_gyro_z_at_calibration);
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    FLEXIFUNCTION_SET(FLEXIFUNCTION_SET_DATA),
    FLEXIFUNCTION_READ_REQ(FLEXIFUNCTION_READ_REQ_DATA),
    FLEXIFUNCTION_BUFFER_FUNCTION(FLEXIFUNCTION_BUFFER_FUNCTION_DATA),
    FLEXIFUNCTION_BUFFER_FUNCTION_ACK(FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA),
    FLEXIFUNCTION_DIRECTORY(FLEXIFUNCTION_DIRECTORY_DATA),
    FLEXIFUNCTION_DIRECTORY_ACK(FLEXIFUNCTION_DIRECTORY_ACK_DATA),
    FLEXIFUNCTION_COMMAND(FLEXIFUNCTION_COMMAND_DATA),
    FLEXIFUNCTION_COMMAND_ACK(FLEXIFUNCTION_COMMAND_ACK_DATA),
    SERIAL_UDB_EXTRA_F2_A(SERIAL_UDB_EXTRA_F2_A_DATA),
    SERIAL_UDB_EXTRA_F2_B(SERIAL_UDB_EXTRA_F2_B_DATA),
    SERIAL_UDB_EXTRA_F4(SERIAL_UDB_EXTRA_F4_DATA),
    SERIAL_UDB_EXTRA_F5(SERIAL_UDB_EXTRA_F5_DATA),
    SERIAL_UDB_EXTRA_F6(SERIAL_UDB_EXTRA_F6_DATA),
    SERIAL_UDB_EXTRA_F7(SERIAL_UDB_EXTRA_F7_DATA),
    SERIAL_UDB_EXTRA_F8(SERIAL_UDB_EXTRA_F8_DATA),
    SERIAL_UDB_EXTRA_F13(SERIAL_UDB_EXTRA_F13_DATA),
    SERIAL_UDB_EXTRA_F14(SERIAL_UDB_EXTRA_F14_DATA),
    SERIAL_UDB_EXTRA_F15(SERIAL_UDB_EXTRA_F15_DATA),
    SERIAL_UDB_EXTRA_F16(SERIAL_UDB_EXTRA_F16_DATA),
    ALTITUDES(ALTITUDES_DATA),
    AIRSPEEDS(AIRSPEEDS_DATA),
    SERIAL_UDB_EXTRA_F17(SERIAL_UDB_EXTRA_F17_DATA),
    SERIAL_UDB_EXTRA_F18(SERIAL_UDB_EXTRA_F18_DATA),
    SERIAL_UDB_EXTRA_F19(SERIAL_UDB_EXTRA_F19_DATA),
    SERIAL_UDB_EXTRA_F20(SERIAL_UDB_EXTRA_F20_DATA),
    SERIAL_UDB_EXTRA_F21(SERIAL_UDB_EXTRA_F21_DATA),
    SERIAL_UDB_EXTRA_F22(SERIAL_UDB_EXTRA_F22_DATA),
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
            150 => FLEXIFUNCTION_SET_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_SET(s)),
            151 => FLEXIFUNCTION_READ_REQ_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_READ_REQ(s)),
            152 => FLEXIFUNCTION_BUFFER_FUNCTION_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION(s)),
            153 => FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION_ACK(s)),
            155 => FLEXIFUNCTION_DIRECTORY_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_DIRECTORY(s)),
            156 => FLEXIFUNCTION_DIRECTORY_ACK_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_DIRECTORY_ACK(s)),
            157 => FLEXIFUNCTION_COMMAND_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_COMMAND(s)),
            158 => FLEXIFUNCTION_COMMAND_ACK_DATA::deser(version, payload)
                .map(|s| MavMessage::FLEXIFUNCTION_COMMAND_ACK(s)),
            170 => SERIAL_UDB_EXTRA_F2_A_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F2_A(s)),
            171 => SERIAL_UDB_EXTRA_F2_B_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F2_B(s)),
            172 => SERIAL_UDB_EXTRA_F4_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F4(s)),
            173 => SERIAL_UDB_EXTRA_F5_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F5(s)),
            174 => SERIAL_UDB_EXTRA_F6_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F6(s)),
            175 => SERIAL_UDB_EXTRA_F7_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F7(s)),
            176 => SERIAL_UDB_EXTRA_F8_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F8(s)),
            177 => SERIAL_UDB_EXTRA_F13_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F13(s)),
            178 => SERIAL_UDB_EXTRA_F14_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F14(s)),
            179 => SERIAL_UDB_EXTRA_F15_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F15(s)),
            180 => SERIAL_UDB_EXTRA_F16_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F16(s)),
            181 => ALTITUDES_DATA::deser(version, payload).map(|s| MavMessage::ALTITUDES(s)),
            182 => AIRSPEEDS_DATA::deser(version, payload).map(|s| MavMessage::AIRSPEEDS(s)),
            183 => SERIAL_UDB_EXTRA_F17_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F17(s)),
            184 => SERIAL_UDB_EXTRA_F18_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F18(s)),
            185 => SERIAL_UDB_EXTRA_F19_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F19(s)),
            186 => SERIAL_UDB_EXTRA_F20_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F20(s)),
            187 => SERIAL_UDB_EXTRA_F21_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F21(s)),
            188 => SERIAL_UDB_EXTRA_F22_DATA::deser(version, payload)
                .map(|s| MavMessage::SERIAL_UDB_EXTRA_F22(s)),
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
            MavMessage::FLEXIFUNCTION_SET(..) => "FLEXIFUNCTION_SET",
            MavMessage::FLEXIFUNCTION_READ_REQ(..) => "FLEXIFUNCTION_READ_REQ",
            MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION(..) => "FLEXIFUNCTION_BUFFER_FUNCTION",
            MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION_ACK(..) => {
                "FLEXIFUNCTION_BUFFER_FUNCTION_ACK"
            }
            MavMessage::FLEXIFUNCTION_DIRECTORY(..) => "FLEXIFUNCTION_DIRECTORY",
            MavMessage::FLEXIFUNCTION_DIRECTORY_ACK(..) => "FLEXIFUNCTION_DIRECTORY_ACK",
            MavMessage::FLEXIFUNCTION_COMMAND(..) => "FLEXIFUNCTION_COMMAND",
            MavMessage::FLEXIFUNCTION_COMMAND_ACK(..) => "FLEXIFUNCTION_COMMAND_ACK",
            MavMessage::SERIAL_UDB_EXTRA_F2_A(..) => "SERIAL_UDB_EXTRA_F2_A",
            MavMessage::SERIAL_UDB_EXTRA_F2_B(..) => "SERIAL_UDB_EXTRA_F2_B",
            MavMessage::SERIAL_UDB_EXTRA_F4(..) => "SERIAL_UDB_EXTRA_F4",
            MavMessage::SERIAL_UDB_EXTRA_F5(..) => "SERIAL_UDB_EXTRA_F5",
            MavMessage::SERIAL_UDB_EXTRA_F6(..) => "SERIAL_UDB_EXTRA_F6",
            MavMessage::SERIAL_UDB_EXTRA_F7(..) => "SERIAL_UDB_EXTRA_F7",
            MavMessage::SERIAL_UDB_EXTRA_F8(..) => "SERIAL_UDB_EXTRA_F8",
            MavMessage::SERIAL_UDB_EXTRA_F13(..) => "SERIAL_UDB_EXTRA_F13",
            MavMessage::SERIAL_UDB_EXTRA_F14(..) => "SERIAL_UDB_EXTRA_F14",
            MavMessage::SERIAL_UDB_EXTRA_F15(..) => "SERIAL_UDB_EXTRA_F15",
            MavMessage::SERIAL_UDB_EXTRA_F16(..) => "SERIAL_UDB_EXTRA_F16",
            MavMessage::ALTITUDES(..) => "ALTITUDES",
            MavMessage::AIRSPEEDS(..) => "AIRSPEEDS",
            MavMessage::SERIAL_UDB_EXTRA_F17(..) => "SERIAL_UDB_EXTRA_F17",
            MavMessage::SERIAL_UDB_EXTRA_F18(..) => "SERIAL_UDB_EXTRA_F18",
            MavMessage::SERIAL_UDB_EXTRA_F19(..) => "SERIAL_UDB_EXTRA_F19",
            MavMessage::SERIAL_UDB_EXTRA_F20(..) => "SERIAL_UDB_EXTRA_F20",
            MavMessage::SERIAL_UDB_EXTRA_F21(..) => "SERIAL_UDB_EXTRA_F21",
            MavMessage::SERIAL_UDB_EXTRA_F22(..) => "SERIAL_UDB_EXTRA_F22",
            MavMessage::common(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::FLEXIFUNCTION_SET(..) => 150,
            MavMessage::FLEXIFUNCTION_READ_REQ(..) => 151,
            MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION(..) => 152,
            MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION_ACK(..) => 153,
            MavMessage::FLEXIFUNCTION_DIRECTORY(..) => 155,
            MavMessage::FLEXIFUNCTION_DIRECTORY_ACK(..) => 156,
            MavMessage::FLEXIFUNCTION_COMMAND(..) => 157,
            MavMessage::FLEXIFUNCTION_COMMAND_ACK(..) => 158,
            MavMessage::SERIAL_UDB_EXTRA_F2_A(..) => 170,
            MavMessage::SERIAL_UDB_EXTRA_F2_B(..) => 171,
            MavMessage::SERIAL_UDB_EXTRA_F4(..) => 172,
            MavMessage::SERIAL_UDB_EXTRA_F5(..) => 173,
            MavMessage::SERIAL_UDB_EXTRA_F6(..) => 174,
            MavMessage::SERIAL_UDB_EXTRA_F7(..) => 175,
            MavMessage::SERIAL_UDB_EXTRA_F8(..) => 176,
            MavMessage::SERIAL_UDB_EXTRA_F13(..) => 177,
            MavMessage::SERIAL_UDB_EXTRA_F14(..) => 178,
            MavMessage::SERIAL_UDB_EXTRA_F15(..) => 179,
            MavMessage::SERIAL_UDB_EXTRA_F16(..) => 180,
            MavMessage::ALTITUDES(..) => 181,
            MavMessage::AIRSPEEDS(..) => 182,
            MavMessage::SERIAL_UDB_EXTRA_F17(..) => 183,
            MavMessage::SERIAL_UDB_EXTRA_F18(..) => 184,
            MavMessage::SERIAL_UDB_EXTRA_F19(..) => 185,
            MavMessage::SERIAL_UDB_EXTRA_F20(..) => 186,
            MavMessage::SERIAL_UDB_EXTRA_F21(..) => 187,
            MavMessage::SERIAL_UDB_EXTRA_F22(..) => 188,
            MavMessage::common(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "FLEXIFUNCTION_SET" => Ok(150),
            "FLEXIFUNCTION_READ_REQ" => Ok(151),
            "FLEXIFUNCTION_BUFFER_FUNCTION" => Ok(152),
            "FLEXIFUNCTION_BUFFER_FUNCTION_ACK" => Ok(153),
            "FLEXIFUNCTION_DIRECTORY" => Ok(155),
            "FLEXIFUNCTION_DIRECTORY_ACK" => Ok(156),
            "FLEXIFUNCTION_COMMAND" => Ok(157),
            "FLEXIFUNCTION_COMMAND_ACK" => Ok(158),
            "SERIAL_UDB_EXTRA_F2_A" => Ok(170),
            "SERIAL_UDB_EXTRA_F2_B" => Ok(171),
            "SERIAL_UDB_EXTRA_F4" => Ok(172),
            "SERIAL_UDB_EXTRA_F5" => Ok(173),
            "SERIAL_UDB_EXTRA_F6" => Ok(174),
            "SERIAL_UDB_EXTRA_F7" => Ok(175),
            "SERIAL_UDB_EXTRA_F8" => Ok(176),
            "SERIAL_UDB_EXTRA_F13" => Ok(177),
            "SERIAL_UDB_EXTRA_F14" => Ok(178),
            "SERIAL_UDB_EXTRA_F15" => Ok(179),
            "SERIAL_UDB_EXTRA_F16" => Ok(180),
            "ALTITUDES" => Ok(181),
            "AIRSPEEDS" => Ok(182),
            "SERIAL_UDB_EXTRA_F17" => Ok(183),
            "SERIAL_UDB_EXTRA_F18" => Ok(184),
            "SERIAL_UDB_EXTRA_F19" => Ok(185),
            "SERIAL_UDB_EXTRA_F20" => Ok(186),
            "SERIAL_UDB_EXTRA_F21" => Ok(187),
            "SERIAL_UDB_EXTRA_F22" => Ok(188),
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
            150 => Ok(MavMessage::FLEXIFUNCTION_SET(
                FLEXIFUNCTION_SET_DATA::default(),
            )),
            151 => Ok(MavMessage::FLEXIFUNCTION_READ_REQ(
                FLEXIFUNCTION_READ_REQ_DATA::default(),
            )),
            152 => Ok(MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION(
                FLEXIFUNCTION_BUFFER_FUNCTION_DATA::default(),
            )),
            153 => Ok(MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION_ACK(
                FLEXIFUNCTION_BUFFER_FUNCTION_ACK_DATA::default(),
            )),
            155 => Ok(MavMessage::FLEXIFUNCTION_DIRECTORY(
                FLEXIFUNCTION_DIRECTORY_DATA::default(),
            )),
            156 => Ok(MavMessage::FLEXIFUNCTION_DIRECTORY_ACK(
                FLEXIFUNCTION_DIRECTORY_ACK_DATA::default(),
            )),
            157 => Ok(MavMessage::FLEXIFUNCTION_COMMAND(
                FLEXIFUNCTION_COMMAND_DATA::default(),
            )),
            158 => Ok(MavMessage::FLEXIFUNCTION_COMMAND_ACK(
                FLEXIFUNCTION_COMMAND_ACK_DATA::default(),
            )),
            170 => Ok(MavMessage::SERIAL_UDB_EXTRA_F2_A(
                SERIAL_UDB_EXTRA_F2_A_DATA::default(),
            )),
            171 => Ok(MavMessage::SERIAL_UDB_EXTRA_F2_B(
                SERIAL_UDB_EXTRA_F2_B_DATA::default(),
            )),
            172 => Ok(MavMessage::SERIAL_UDB_EXTRA_F4(
                SERIAL_UDB_EXTRA_F4_DATA::default(),
            )),
            173 => Ok(MavMessage::SERIAL_UDB_EXTRA_F5(
                SERIAL_UDB_EXTRA_F5_DATA::default(),
            )),
            174 => Ok(MavMessage::SERIAL_UDB_EXTRA_F6(
                SERIAL_UDB_EXTRA_F6_DATA::default(),
            )),
            175 => Ok(MavMessage::SERIAL_UDB_EXTRA_F7(
                SERIAL_UDB_EXTRA_F7_DATA::default(),
            )),
            176 => Ok(MavMessage::SERIAL_UDB_EXTRA_F8(
                SERIAL_UDB_EXTRA_F8_DATA::default(),
            )),
            177 => Ok(MavMessage::SERIAL_UDB_EXTRA_F13(
                SERIAL_UDB_EXTRA_F13_DATA::default(),
            )),
            178 => Ok(MavMessage::SERIAL_UDB_EXTRA_F14(
                SERIAL_UDB_EXTRA_F14_DATA::default(),
            )),
            179 => Ok(MavMessage::SERIAL_UDB_EXTRA_F15(
                SERIAL_UDB_EXTRA_F15_DATA::default(),
            )),
            180 => Ok(MavMessage::SERIAL_UDB_EXTRA_F16(
                SERIAL_UDB_EXTRA_F16_DATA::default(),
            )),
            181 => Ok(MavMessage::ALTITUDES(ALTITUDES_DATA::default())),
            182 => Ok(MavMessage::AIRSPEEDS(AIRSPEEDS_DATA::default())),
            183 => Ok(MavMessage::SERIAL_UDB_EXTRA_F17(
                SERIAL_UDB_EXTRA_F17_DATA::default(),
            )),
            184 => Ok(MavMessage::SERIAL_UDB_EXTRA_F18(
                SERIAL_UDB_EXTRA_F18_DATA::default(),
            )),
            185 => Ok(MavMessage::SERIAL_UDB_EXTRA_F19(
                SERIAL_UDB_EXTRA_F19_DATA::default(),
            )),
            186 => Ok(MavMessage::SERIAL_UDB_EXTRA_F20(
                SERIAL_UDB_EXTRA_F20_DATA::default(),
            )),
            187 => Ok(MavMessage::SERIAL_UDB_EXTRA_F21(
                SERIAL_UDB_EXTRA_F21_DATA::default(),
            )),
            188 => Ok(MavMessage::SERIAL_UDB_EXTRA_F22(
                SERIAL_UDB_EXTRA_F22_DATA::default(),
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
            &MavMessage::FLEXIFUNCTION_SET(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_READ_REQ(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_BUFFER_FUNCTION_ACK(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_DIRECTORY(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_DIRECTORY_ACK(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_COMMAND(ref body) => body.ser(),
            &MavMessage::FLEXIFUNCTION_COMMAND_ACK(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F2_A(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F2_B(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F4(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F5(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F6(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F7(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F8(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F13(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F14(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F15(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F16(ref body) => body.ser(),
            &MavMessage::ALTITUDES(ref body) => body.ser(),
            &MavMessage::AIRSPEEDS(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F17(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F18(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F19(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F20(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F21(ref body) => body.ser(),
            &MavMessage::SERIAL_UDB_EXTRA_F22(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 181,
            151 => 26,
            152 => 101,
            153 => 109,
            155 => 12,
            156 => 218,
            157 => 133,
            158 => 208,
            170 => 103,
            171 => 245,
            172 => 191,
            173 => 54,
            174 => 54,
            175 => 171,
            176 => 142,
            177 => 249,
            178 => 123,
            179 => 7,
            180 => 222,
            181 => 55,
            182 => 154,
            183 => 175,
            184 => 41,
            185 => 87,
            186 => 144,
            187 => 134,
            188 => 91,
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
