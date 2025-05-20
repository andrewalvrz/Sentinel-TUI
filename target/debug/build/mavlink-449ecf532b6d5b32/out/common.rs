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
    MAV_TYPE_ODID = 34,
}
impl Default for MavType {
    fn default() -> Self {
        MavType::MAV_TYPE_GENERIC
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FirmwareVersionType {
    FIRMWARE_VERSION_TYPE_DEV = 0,
    FIRMWARE_VERSION_TYPE_ALPHA = 64,
    FIRMWARE_VERSION_TYPE_BETA = 128,
    FIRMWARE_VERSION_TYPE_RC = 192,
    FIRMWARE_VERSION_TYPE_OFFICIAL = 255,
}
impl Default for FirmwareVersionType {
    fn default() -> Self {
        FirmwareVersionType::FIRMWARE_VERSION_TYPE_DEV
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct HlFailureFlag : u16 { const HL_FAILURE_FLAG_GPS = 1 ; const HL_FAILURE_FLAG_DIFFERENTIAL_PRESSURE = 2 ; const HL_FAILURE_FLAG_ABSOLUTE_PRESSURE = 4 ; const HL_FAILURE_FLAG_3D_ACCEL = 8 ; const HL_FAILURE_FLAG_3D_GYRO = 16 ; const HL_FAILURE_FLAG_3D_MAG = 32 ; const HL_FAILURE_FLAG_TERRAIN = 64 ; const HL_FAILURE_FLAG_BATTERY = 128 ; const HL_FAILURE_FLAG_RC_RECEIVER = 256 ; const HL_FAILURE_FLAG_OFFBOARD_LINK = 512 ; const HL_FAILURE_FLAG_ENGINE = 1024 ; const HL_FAILURE_FLAG_GEOFENCE = 2048 ; const HL_FAILURE_FLAG_ESTIMATOR = 4096 ; const HL_FAILURE_FLAG_MISSION = 8192 ; } }
impl Default for HlFailureFlag {
    fn default() -> Self {
        HlFailureFlag::HL_FAILURE_FLAG_GPS
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
pub enum MavGoto {
    MAV_GOTO_DO_HOLD = 0,
    MAV_GOTO_DO_CONTINUE = 1,
    MAV_GOTO_HOLD_AT_CURRENT_POSITION = 2,
    MAV_GOTO_HOLD_AT_SPECIFIED_POSITION = 3,
}
impl Default for MavGoto {
    fn default() -> Self {
        MavGoto::MAV_GOTO_DO_HOLD
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMode {
    MAV_MODE_PREFLIGHT = 0,
    MAV_MODE_STABILIZE_DISARMED = 80,
    MAV_MODE_STABILIZE_ARMED = 208,
    MAV_MODE_MANUAL_DISARMED = 64,
    MAV_MODE_MANUAL_ARMED = 192,
    MAV_MODE_GUIDED_DISARMED = 88,
    MAV_MODE_GUIDED_ARMED = 216,
    MAV_MODE_AUTO_DISARMED = 92,
    MAV_MODE_AUTO_ARMED = 220,
    MAV_MODE_TEST_DISARMED = 66,
    MAV_MODE_TEST_ARMED = 194,
}
impl Default for MavMode {
    fn default() -> Self {
        MavMode::MAV_MODE_PREFLIGHT
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
    MAV_COMP_ID_TELEMETRY_RADIO = 68,
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
    MAV_COMP_ID_GIMBAL2 = 171,
    MAV_COMP_ID_GIMBAL3 = 172,
    MAV_COMP_ID_GIMBAL4 = 173,
    MAV_COMP_ID_GIMBAL5 = 174,
    MAV_COMP_ID_GIMBAL6 = 175,
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
    MAV_COMP_ID_ODID_TXRX_1 = 236,
    MAV_COMP_ID_ODID_TXRX_2 = 237,
    MAV_COMP_ID_ODID_TXRX_3 = 238,
    MAV_COMP_ID_UDP_BRIDGE = 240,
    MAV_COMP_ID_UART_BRIDGE = 241,
    MAV_COMP_ID_TUNNEL_NODE = 242,
    MAV_COMP_ID_SYSTEM_CONTROL = 250,
}
impl Default for MavComponent {
    fn default() -> Self {
        MavComponent::MAV_COMP_ID_ALL
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavSysStatusSensor : u32 { const MAV_SYS_STATUS_SENSOR_3D_GYRO = 1 ; const MAV_SYS_STATUS_SENSOR_3D_ACCEL = 2 ; const MAV_SYS_STATUS_SENSOR_3D_MAG = 4 ; const MAV_SYS_STATUS_SENSOR_ABSOLUTE_PRESSURE = 8 ; const MAV_SYS_STATUS_SENSOR_DIFFERENTIAL_PRESSURE = 16 ; const MAV_SYS_STATUS_SENSOR_GPS = 32 ; const MAV_SYS_STATUS_SENSOR_OPTICAL_FLOW = 64 ; const MAV_SYS_STATUS_SENSOR_VISION_POSITION = 128 ; const MAV_SYS_STATUS_SENSOR_LASER_POSITION = 256 ; const MAV_SYS_STATUS_SENSOR_EXTERNAL_GROUND_TRUTH = 512 ; const MAV_SYS_STATUS_SENSOR_ANGULAR_RATE_CONTROL = 1024 ; const MAV_SYS_STATUS_SENSOR_ATTITUDE_STABILIZATION = 2048 ; const MAV_SYS_STATUS_SENSOR_YAW_POSITION = 4096 ; const MAV_SYS_STATUS_SENSOR_Z_ALTITUDE_CONTROL = 8192 ; const MAV_SYS_STATUS_SENSOR_XY_POSITION_CONTROL = 16384 ; const MAV_SYS_STATUS_SENSOR_MOTOR_OUTPUTS = 32768 ; const MAV_SYS_STATUS_SENSOR_RC_RECEIVER = 65536 ; const MAV_SYS_STATUS_SENSOR_3D_GYRO2 = 131072 ; const MAV_SYS_STATUS_SENSOR_3D_ACCEL2 = 262144 ; const MAV_SYS_STATUS_SENSOR_3D_MAG2 = 524288 ; const MAV_SYS_STATUS_GEOFENCE = 1048576 ; const MAV_SYS_STATUS_AHRS = 2097152 ; const MAV_SYS_STATUS_TERRAIN = 4194304 ; const MAV_SYS_STATUS_REVERSE_MOTOR = 8388608 ; const MAV_SYS_STATUS_LOGGING = 16777216 ; const MAV_SYS_STATUS_SENSOR_BATTERY = 33554432 ; const MAV_SYS_STATUS_SENSOR_PROXIMITY = 67108864 ; const MAV_SYS_STATUS_SENSOR_SATCOM = 134217728 ; const MAV_SYS_STATUS_PREARM_CHECK = 268435456 ; const MAV_SYS_STATUS_OBSTACLE_AVOIDANCE = 536870912 ; } }
impl Default for MavSysStatusSensor {
    fn default() -> Self {
        MavSysStatusSensor::MAV_SYS_STATUS_SENSOR_3D_GYRO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavFrame {
    MAV_FRAME_GLOBAL = 0,
    MAV_FRAME_LOCAL_NED = 1,
    MAV_FRAME_MISSION = 2,
    MAV_FRAME_GLOBAL_RELATIVE_ALT = 3,
    MAV_FRAME_LOCAL_ENU = 4,
    MAV_FRAME_GLOBAL_INT = 5,
    MAV_FRAME_GLOBAL_RELATIVE_ALT_INT = 6,
    MAV_FRAME_LOCAL_OFFSET_NED = 7,
    MAV_FRAME_BODY_NED = 8,
    MAV_FRAME_BODY_OFFSET_NED = 9,
    MAV_FRAME_GLOBAL_TERRAIN_ALT = 10,
    MAV_FRAME_GLOBAL_TERRAIN_ALT_INT = 11,
    MAV_FRAME_BODY_FRD = 12,
    MAV_FRAME_RESERVED_13 = 13,
    MAV_FRAME_RESERVED_14 = 14,
    MAV_FRAME_RESERVED_15 = 15,
    MAV_FRAME_RESERVED_16 = 16,
    MAV_FRAME_RESERVED_17 = 17,
    MAV_FRAME_RESERVED_18 = 18,
    MAV_FRAME_RESERVED_19 = 19,
    MAV_FRAME_LOCAL_FRD = 20,
    MAV_FRAME_LOCAL_FLU = 21,
}
impl Default for MavFrame {
    fn default() -> Self {
        MavFrame::MAV_FRAME_GLOBAL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavlinkDataStreamType {
    MAVLINK_DATA_STREAM_IMG_JPEG = 0,
    MAVLINK_DATA_STREAM_IMG_BMP = 1,
    MAVLINK_DATA_STREAM_IMG_RAW8U = 2,
    MAVLINK_DATA_STREAM_IMG_RAW32U = 3,
    MAVLINK_DATA_STREAM_IMG_PGM = 4,
    MAVLINK_DATA_STREAM_IMG_PNG = 5,
}
impl Default for MavlinkDataStreamType {
    fn default() -> Self {
        MavlinkDataStreamType::MAVLINK_DATA_STREAM_IMG_JPEG
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FenceAction {
    FENCE_ACTION_NONE = 0,
    FENCE_ACTION_GUIDED = 1,
    FENCE_ACTION_REPORT = 2,
    FENCE_ACTION_GUIDED_THR_PASS = 3,
    FENCE_ACTION_RTL = 4,
}
impl Default for FenceAction {
    fn default() -> Self {
        FenceAction::FENCE_ACTION_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FenceBreach {
    FENCE_BREACH_NONE = 0,
    FENCE_BREACH_MINALT = 1,
    FENCE_BREACH_MAXALT = 2,
    FENCE_BREACH_BOUNDARY = 3,
}
impl Default for FenceBreach {
    fn default() -> Self {
        FenceBreach::FENCE_BREACH_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FenceMitigate {
    FENCE_MITIGATE_UNKNOWN = 0,
    FENCE_MITIGATE_NONE = 1,
    FENCE_MITIGATE_VEL_LIMIT = 2,
}
impl Default for FenceMitigate {
    fn default() -> Self {
        FenceMitigate::FENCE_MITIGATE_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMountMode {
    MAV_MOUNT_MODE_RETRACT = 0,
    MAV_MOUNT_MODE_NEUTRAL = 1,
    MAV_MOUNT_MODE_MAVLINK_TARGETING = 2,
    MAV_MOUNT_MODE_RC_TARGETING = 3,
    MAV_MOUNT_MODE_GPS_POINT = 4,
    MAV_MOUNT_MODE_SYSID_TARGET = 5,
}
impl Default for MavMountMode {
    fn default() -> Self {
        MavMountMode::MAV_MOUNT_MODE_RETRACT
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct GimbalDeviceCapFlags : u16 { const GIMBAL_DEVICE_CAP_FLAGS_HAS_RETRACT = 1 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_NEUTRAL = 2 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_AXIS = 4 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_FOLLOW = 8 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_ROLL_LOCK = 16 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_AXIS = 32 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_FOLLOW = 64 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_PITCH_LOCK = 128 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_AXIS = 256 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_FOLLOW = 512 ; const GIMBAL_DEVICE_CAP_FLAGS_HAS_YAW_LOCK = 1024 ; const GIMBAL_DEVICE_CAP_FLAGS_SUPPORTS_INFINITE_YAW = 2048 ; } }
impl Default for GimbalDeviceCapFlags {
    fn default() -> Self {
        GimbalDeviceCapFlags::GIMBAL_DEVICE_CAP_FLAGS_HAS_RETRACT
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct GimbalManagerCapFlags : u32 { const GIMBAL_MANAGER_CAP_FLAGS_HAS_RETRACT = 1 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_NEUTRAL = 2 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_ROLL_AXIS = 4 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_ROLL_FOLLOW = 8 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_ROLL_LOCK = 16 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_PITCH_AXIS = 32 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_PITCH_FOLLOW = 64 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_PITCH_LOCK = 128 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_YAW_AXIS = 256 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_YAW_FOLLOW = 512 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_YAW_LOCK = 1024 ; const GIMBAL_MANAGER_CAP_FLAGS_SUPPORTS_INFINITE_YAW = 2048 ; const GIMBAL_MANAGER_CAP_FLAGS_CAN_POINT_LOCATION_LOCAL = 65536 ; const GIMBAL_MANAGER_CAP_FLAGS_CAN_POINT_LOCATION_GLOBAL = 131072 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_TRACKING_POINT = 262144 ; const GIMBAL_MANAGER_CAP_FLAGS_HAS_TRACKING_RECTANGLE = 524288 ; const GIMBAL_MANAGER_CAP_FLAGS_SUPPORTS_FOCAL_LENGTH_SCALE = 1048576 ; const GIMBAL_MANAGER_CAP_FLAGS_SUPPORTS_NUDGING = 2097152 ; const GIMBAL_MANAGER_CAP_FLAGS_SUPPORTS_OVERRIDE = 4194304 ; } }
impl Default for GimbalManagerCapFlags {
    fn default() -> Self {
        GimbalManagerCapFlags::GIMBAL_MANAGER_CAP_FLAGS_HAS_RETRACT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalDeviceFlags {
    GIMBAL_DEVICE_FLAGS_RETRACT = 1,
    GIMBAL_DEVICE_FLAGS_NEUTRAL = 2,
    GIMBAL_DEVICE_FLAGS_ROLL_LOCK = 4,
    GIMBAL_DEVICE_FLAGS_PITCH_LOCK = 8,
    GIMBAL_DEVICE_FLAGS_YAW_LOCK = 16,
}
impl Default for GimbalDeviceFlags {
    fn default() -> Self {
        GimbalDeviceFlags::GIMBAL_DEVICE_FLAGS_RETRACT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalManagerFlags {
    GIMBAL_MANAGER_FLAGS_RETRACT = 1,
    GIMBAL_MANAGER_FLAGS_NEUTRAL = 2,
    GIMBAL_MANAGER_FLAGS_ROLL_LOCK = 4,
    GIMBAL_MANAGER_FLAGS_PITCH_LOCK = 8,
    GIMBAL_MANAGER_FLAGS_YAW_LOCK = 16,
    GIMBAL_MANAGER_FLAGS_ANGULAR_VELOCITY_RELATIVE_TO_FOCAL_LENGTH = 1048576,
    GIMBAL_MANAGER_FLAGS_NUDGE = 2097152,
    GIMBAL_MANAGER_FLAGS_OVERRIDE = 4194304,
    GIMBAL_MANAGER_FLAGS_NONE = 8388608,
}
impl Default for GimbalManagerFlags {
    fn default() -> Self {
        GimbalManagerFlags::GIMBAL_MANAGER_FLAGS_RETRACT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalDeviceErrorFlags {
    GIMBAL_DEVICE_ERROR_FLAGS_AT_ROLL_LIMIT = 1,
    GIMBAL_DEVICE_ERROR_FLAGS_AT_PITCH_LIMIT = 2,
    GIMBAL_DEVICE_ERROR_FLAGS_AT_YAW_LIMIT = 4,
    GIMBAL_DEVICE_ERROR_FLAGS_ENCODER_ERROR = 8,
    GIMBAL_DEVICE_ERROR_FLAGS_POWER_ERROR = 16,
    GIMBAL_DEVICE_ERROR_FLAGS_MOTOR_ERROR = 32,
    GIMBAL_DEVICE_ERROR_FLAGS_SOFTWARE_ERROR = 64,
    GIMBAL_DEVICE_ERROR_FLAGS_COMMS_ERROR = 128,
}
impl Default for GimbalDeviceErrorFlags {
    fn default() -> Self {
        GimbalDeviceErrorFlags::GIMBAL_DEVICE_ERROR_FLAGS_AT_ROLL_LIMIT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavcanNodeHealth {
    UAVCAN_NODE_HEALTH_OK = 0,
    UAVCAN_NODE_HEALTH_WARNING = 1,
    UAVCAN_NODE_HEALTH_ERROR = 2,
    UAVCAN_NODE_HEALTH_CRITICAL = 3,
}
impl Default for UavcanNodeHealth {
    fn default() -> Self {
        UavcanNodeHealth::UAVCAN_NODE_HEALTH_OK
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UavcanNodeMode {
    UAVCAN_NODE_MODE_OPERATIONAL = 0,
    UAVCAN_NODE_MODE_INITIALIZATION = 1,
    UAVCAN_NODE_MODE_MAINTENANCE = 2,
    UAVCAN_NODE_MODE_SOFTWARE_UPDATE = 3,
    UAVCAN_NODE_MODE_OFFLINE = 7,
}
impl Default for UavcanNodeMode {
    fn default() -> Self {
        UavcanNodeMode::UAVCAN_NODE_MODE_OPERATIONAL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum StorageStatus {
    STORAGE_STATUS_EMPTY = 0,
    STORAGE_STATUS_UNFORMATTED = 1,
    STORAGE_STATUS_READY = 2,
    STORAGE_STATUS_NOT_SUPPORTED = 3,
}
impl Default for StorageStatus {
    fn default() -> Self {
        StorageStatus::STORAGE_STATUS_EMPTY
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum OrbitYawBehaviour {
    ORBIT_YAW_BEHAVIOUR_HOLD_FRONT_TO_CIRCLE_CENTER = 0,
    ORBIT_YAW_BEHAVIOUR_HOLD_INITIAL_HEADING = 1,
    ORBIT_YAW_BEHAVIOUR_UNCONTROLLED = 2,
    ORBIT_YAW_BEHAVIOUR_HOLD_FRONT_TANGENT_TO_CIRCLE = 3,
    ORBIT_YAW_BEHAVIOUR_RC_CONTROLLED = 4,
}
impl Default for OrbitYawBehaviour {
    fn default() -> Self {
        OrbitYawBehaviour::ORBIT_YAW_BEHAVIOUR_HOLD_FRONT_TO_CIRCLE_CENTER
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum WifiConfigApResponse {
    WIFI_CONFIG_AP_RESPONSE_UNDEFINED = 0,
    WIFI_CONFIG_AP_RESPONSE_ACCEPTED = 1,
    WIFI_CONFIG_AP_RESPONSE_REJECTED = 2,
    WIFI_CONFIG_AP_RESPONSE_MODE_ERROR = 3,
    WIFI_CONFIG_AP_RESPONSE_SSID_ERROR = 4,
    WIFI_CONFIG_AP_RESPONSE_PASSWORD_ERROR = 5,
}
impl Default for WifiConfigApResponse {
    fn default() -> Self {
        WifiConfigApResponse::WIFI_CONFIG_AP_RESPONSE_UNDEFINED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CellularConfigResponse {
    CELLULAR_CONFIG_RESPONSE_ACCEPTED = 0,
    CELLULAR_CONFIG_RESPONSE_APN_ERROR = 1,
    CELLULAR_CONFIG_RESPONSE_PIN_ERROR = 2,
    CELLULAR_CONFIG_RESPONSE_REJECTED = 3,
}
impl Default for CellularConfigResponse {
    fn default() -> Self {
        CellularConfigResponse::CELLULAR_CONFIG_RESPONSE_ACCEPTED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum WifiConfigApMode {
    WIFI_CONFIG_AP_MODE_UNDEFINED = 0,
    WIFI_CONFIG_AP_MODE_AP = 1,
    WIFI_CONFIG_AP_MODE_STATION = 2,
    WIFI_CONFIG_AP_MODE_DISABLED = 3,
}
impl Default for WifiConfigApMode {
    fn default() -> Self {
        WifiConfigApMode::WIFI_CONFIG_AP_MODE_UNDEFINED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CompMetadataType {
    COMP_METADATA_TYPE_VERSION = 0,
    COMP_METADATA_TYPE_PARAMETER = 1,
}
impl Default for CompMetadataType {
    fn default() -> Self {
        CompMetadataType::COMP_METADATA_TYPE_VERSION
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmd {
    MAV_CMD_NAV_WAYPOINT = 16,
    MAV_CMD_NAV_LOITER_UNLIM = 17,
    MAV_CMD_NAV_LOITER_TURNS = 18,
    MAV_CMD_NAV_LOITER_TIME = 19,
    MAV_CMD_NAV_RETURN_TO_LAUNCH = 20,
    MAV_CMD_NAV_LAND = 21,
    MAV_CMD_NAV_TAKEOFF = 22,
    MAV_CMD_NAV_LAND_LOCAL = 23,
    MAV_CMD_NAV_TAKEOFF_LOCAL = 24,
    MAV_CMD_NAV_FOLLOW = 25,
    MAV_CMD_NAV_CONTINUE_AND_CHANGE_ALT = 30,
    MAV_CMD_NAV_LOITER_TO_ALT = 31,
    MAV_CMD_DO_FOLLOW = 32,
    MAV_CMD_DO_FOLLOW_REPOSITION = 33,
    MAV_CMD_DO_ORBIT = 34,
    MAV_CMD_NAV_ROI = 80,
    MAV_CMD_NAV_PATHPLANNING = 81,
    MAV_CMD_NAV_SPLINE_WAYPOINT = 82,
    MAV_CMD_NAV_VTOL_TAKEOFF = 84,
    MAV_CMD_NAV_VTOL_LAND = 85,
    MAV_CMD_NAV_GUIDED_ENABLE = 92,
    MAV_CMD_NAV_DELAY = 93,
    MAV_CMD_NAV_PAYLOAD_PLACE = 94,
    MAV_CMD_NAV_LAST = 95,
    MAV_CMD_CONDITION_DELAY = 112,
    MAV_CMD_CONDITION_CHANGE_ALT = 113,
    MAV_CMD_CONDITION_DISTANCE = 114,
    MAV_CMD_CONDITION_YAW = 115,
    MAV_CMD_CONDITION_LAST = 159,
    MAV_CMD_DO_SET_MODE = 176,
    MAV_CMD_DO_JUMP = 177,
    MAV_CMD_DO_CHANGE_SPEED = 178,
    MAV_CMD_DO_SET_HOME = 179,
    MAV_CMD_DO_SET_PARAMETER = 180,
    MAV_CMD_DO_SET_RELAY = 181,
    MAV_CMD_DO_REPEAT_RELAY = 182,
    MAV_CMD_DO_SET_SERVO = 183,
    MAV_CMD_DO_REPEAT_SERVO = 184,
    MAV_CMD_DO_FLIGHTTERMINATION = 185,
    MAV_CMD_DO_CHANGE_ALTITUDE = 186,
    MAV_CMD_DO_SET_ACTUATOR = 187,
    MAV_CMD_DO_LAND_START = 189,
    MAV_CMD_DO_RALLY_LAND = 190,
    MAV_CMD_DO_GO_AROUND = 191,
    MAV_CMD_DO_REPOSITION = 192,
    MAV_CMD_DO_PAUSE_CONTINUE = 193,
    MAV_CMD_DO_SET_REVERSE = 194,
    MAV_CMD_DO_SET_ROI_LOCATION = 195,
    MAV_CMD_DO_SET_ROI_WPNEXT_OFFSET = 196,
    MAV_CMD_DO_SET_ROI_NONE = 197,
    MAV_CMD_DO_SET_ROI_SYSID = 198,
    MAV_CMD_DO_CONTROL_VIDEO = 200,
    MAV_CMD_DO_SET_ROI = 201,
    MAV_CMD_DO_DIGICAM_CONFIGURE = 202,
    MAV_CMD_DO_DIGICAM_CONTROL = 203,
    MAV_CMD_DO_MOUNT_CONFIGURE = 204,
    MAV_CMD_DO_MOUNT_CONTROL = 205,
    MAV_CMD_DO_SET_CAM_TRIGG_DIST = 206,
    MAV_CMD_DO_FENCE_ENABLE = 207,
    MAV_CMD_DO_PARACHUTE = 208,
    MAV_CMD_DO_MOTOR_TEST = 209,
    MAV_CMD_DO_INVERTED_FLIGHT = 210,
    MAV_CMD_NAV_SET_YAW_SPEED = 213,
    MAV_CMD_DO_SET_CAM_TRIGG_INTERVAL = 214,
    MAV_CMD_DO_MOUNT_CONTROL_QUAT = 220,
    MAV_CMD_DO_GUIDED_MASTER = 221,
    MAV_CMD_DO_GUIDED_LIMITS = 222,
    MAV_CMD_DO_ENGINE_CONTROL = 223,
    MAV_CMD_DO_SET_MISSION_CURRENT = 224,
    MAV_CMD_DO_LAST = 240,
    MAV_CMD_PREFLIGHT_CALIBRATION = 241,
    MAV_CMD_PREFLIGHT_SET_SENSOR_OFFSETS = 242,
    MAV_CMD_PREFLIGHT_UAVCAN = 243,
    MAV_CMD_PREFLIGHT_STORAGE = 245,
    MAV_CMD_PREFLIGHT_REBOOT_SHUTDOWN = 246,
    MAV_CMD_DO_UPGRADE = 247,
    MAV_CMD_OVERRIDE_GOTO = 252,
    MAV_CMD_MISSION_START = 300,
    MAV_CMD_COMPONENT_ARM_DISARM = 400,
    MAV_CMD_ILLUMINATOR_ON_OFF = 405,
    MAV_CMD_GET_HOME_POSITION = 410,
    MAV_CMD_INJECT_FAILURE = 420,
    MAV_CMD_START_RX_PAIR = 500,
    MAV_CMD_GET_MESSAGE_INTERVAL = 510,
    MAV_CMD_SET_MESSAGE_INTERVAL = 511,
    MAV_CMD_REQUEST_MESSAGE = 512,
    MAV_CMD_REQUEST_PROTOCOL_VERSION = 519,
    MAV_CMD_REQUEST_AUTOPILOT_CAPABILITIES = 520,
    MAV_CMD_REQUEST_CAMERA_INFORMATION = 521,
    MAV_CMD_REQUEST_CAMERA_SETTINGS = 522,
    MAV_CMD_REQUEST_STORAGE_INFORMATION = 525,
    MAV_CMD_STORAGE_FORMAT = 526,
    MAV_CMD_REQUEST_CAMERA_CAPTURE_STATUS = 527,
    MAV_CMD_REQUEST_FLIGHT_INFORMATION = 528,
    MAV_CMD_RESET_CAMERA_SETTINGS = 529,
    MAV_CMD_SET_CAMERA_MODE = 530,
    MAV_CMD_SET_CAMERA_ZOOM = 531,
    MAV_CMD_SET_CAMERA_FOCUS = 532,
    MAV_CMD_JUMP_TAG = 600,
    MAV_CMD_DO_JUMP_TAG = 601,
    MAV_CMD_DO_GIMBAL_MANAGER_TILTPAN = 1000,
    MAV_CMD_DO_GIMBAL_MANAGER_TRACK_POINT = 1001,
    MAV_CMD_DO_GIMBAL_MANAGER_TRACK_RECTANGLE = 1002,
    MAV_CMD_IMAGE_START_CAPTURE = 2000,
    MAV_CMD_IMAGE_STOP_CAPTURE = 2001,
    MAV_CMD_REQUEST_CAMERA_IMAGE_CAPTURE = 2002,
    MAV_CMD_DO_TRIGGER_CONTROL = 2003,
    MAV_CMD_VIDEO_START_CAPTURE = 2500,
    MAV_CMD_VIDEO_STOP_CAPTURE = 2501,
    MAV_CMD_VIDEO_START_STREAMING = 2502,
    MAV_CMD_VIDEO_STOP_STREAMING = 2503,
    MAV_CMD_REQUEST_VIDEO_STREAM_INFORMATION = 2504,
    MAV_CMD_REQUEST_VIDEO_STREAM_STATUS = 2505,
    MAV_CMD_LOGGING_START = 2510,
    MAV_CMD_LOGGING_STOP = 2511,
    MAV_CMD_AIRFRAME_CONFIGURATION = 2520,
    MAV_CMD_CONTROL_HIGH_LATENCY = 2600,
    MAV_CMD_PANORAMA_CREATE = 2800,
    MAV_CMD_DO_VTOL_TRANSITION = 3000,
    MAV_CMD_ARM_AUTHORIZATION_REQUEST = 3001,
    MAV_CMD_SET_GUIDED_SUBMODE_STANDARD = 4000,
    MAV_CMD_SET_GUIDED_SUBMODE_CIRCLE = 4001,
    MAV_CMD_CONDITION_GATE = 4501,
    MAV_CMD_NAV_FENCE_RETURN_POINT = 5000,
    MAV_CMD_NAV_FENCE_POLYGON_VERTEX_INCLUSION = 5001,
    MAV_CMD_NAV_FENCE_POLYGON_VERTEX_EXCLUSION = 5002,
    MAV_CMD_NAV_FENCE_CIRCLE_INCLUSION = 5003,
    MAV_CMD_NAV_FENCE_CIRCLE_EXCLUSION = 5004,
    MAV_CMD_NAV_RALLY_POINT = 5100,
    MAV_CMD_UAVCAN_GET_NODE_INFO = 5200,
    MAV_CMD_PAYLOAD_PREPARE_DEPLOY = 30001,
    MAV_CMD_FIXED_MAG_CAL_YAW = 42006,
    MAV_CMD_PAYLOAD_CONTROL_DEPLOY = 30002,
    MAV_CMD_WAYPOINT_USER_1 = 31000,
    MAV_CMD_WAYPOINT_USER_2 = 31001,
    MAV_CMD_WAYPOINT_USER_3 = 31002,
    MAV_CMD_WAYPOINT_USER_4 = 31003,
    MAV_CMD_WAYPOINT_USER_5 = 31004,
    MAV_CMD_SPATIAL_USER_1 = 31005,
    MAV_CMD_SPATIAL_USER_2 = 31006,
    MAV_CMD_SPATIAL_USER_3 = 31007,
    MAV_CMD_SPATIAL_USER_4 = 31008,
    MAV_CMD_SPATIAL_USER_5 = 31009,
    MAV_CMD_USER_1 = 31010,
    MAV_CMD_USER_2 = 31011,
    MAV_CMD_USER_3 = 31012,
    MAV_CMD_USER_4 = 31013,
    MAV_CMD_USER_5 = 31014,
    MAV_CMD_FIXED_MAG_CAL = 42004,
    MAV_CMD_FIXED_MAG_CAL_FIELD = 42005,
    MAV_CMD_DO_START_MAG_CAL = 42424,
    MAV_CMD_DO_ACCEPT_MAG_CAL = 42425,
    MAV_CMD_DO_CANCEL_MAG_CAL = 42426,
    MAV_CMD_ACCELCAL_VEHICLE_POS = 42429,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_NAV_WAYPOINT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavDataStream {
    MAV_DATA_STREAM_ALL = 0,
    MAV_DATA_STREAM_RAW_SENSORS = 1,
    MAV_DATA_STREAM_EXTENDED_STATUS = 2,
    MAV_DATA_STREAM_RC_CHANNELS = 3,
    MAV_DATA_STREAM_RAW_CONTROLLER = 4,
    MAV_DATA_STREAM_POSITION = 6,
    MAV_DATA_STREAM_EXTRA1 = 10,
    MAV_DATA_STREAM_EXTRA2 = 11,
    MAV_DATA_STREAM_EXTRA3 = 12,
}
impl Default for MavDataStream {
    fn default() -> Self {
        MavDataStream::MAV_DATA_STREAM_ALL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavRoi {
    MAV_ROI_NONE = 0,
    MAV_ROI_WPNEXT = 1,
    MAV_ROI_WPINDEX = 2,
    MAV_ROI_LOCATION = 3,
    MAV_ROI_TARGET = 4,
}
impl Default for MavRoi {
    fn default() -> Self {
        MavRoi::MAV_ROI_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmdAck {
    MAV_CMD_ACK_OK = 0,
    MAV_CMD_ACK_ERR_FAIL = 1,
    MAV_CMD_ACK_ERR_ACCESS_DENIED = 2,
    MAV_CMD_ACK_ERR_NOT_SUPPORTED = 3,
    MAV_CMD_ACK_ERR_COORDINATE_FRAME_NOT_SUPPORTED = 4,
    MAV_CMD_ACK_ERR_COORDINATES_OUT_OF_RANGE = 5,
    MAV_CMD_ACK_ERR_X_LAT_OUT_OF_RANGE = 6,
    MAV_CMD_ACK_ERR_Y_LON_OUT_OF_RANGE = 7,
    MAV_CMD_ACK_ERR_Z_ALT_OUT_OF_RANGE = 8,
}
impl Default for MavCmdAck {
    fn default() -> Self {
        MavCmdAck::MAV_CMD_ACK_OK
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavParamType {
    MAV_PARAM_TYPE_UINT8 = 1,
    MAV_PARAM_TYPE_INT8 = 2,
    MAV_PARAM_TYPE_UINT16 = 3,
    MAV_PARAM_TYPE_INT16 = 4,
    MAV_PARAM_TYPE_UINT32 = 5,
    MAV_PARAM_TYPE_INT32 = 6,
    MAV_PARAM_TYPE_UINT64 = 7,
    MAV_PARAM_TYPE_INT64 = 8,
    MAV_PARAM_TYPE_REAL32 = 9,
    MAV_PARAM_TYPE_REAL64 = 10,
}
impl Default for MavParamType {
    fn default() -> Self {
        MavParamType::MAV_PARAM_TYPE_UINT8
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavParamExtType {
    MAV_PARAM_EXT_TYPE_UINT8 = 1,
    MAV_PARAM_EXT_TYPE_INT8 = 2,
    MAV_PARAM_EXT_TYPE_UINT16 = 3,
    MAV_PARAM_EXT_TYPE_INT16 = 4,
    MAV_PARAM_EXT_TYPE_UINT32 = 5,
    MAV_PARAM_EXT_TYPE_INT32 = 6,
    MAV_PARAM_EXT_TYPE_UINT64 = 7,
    MAV_PARAM_EXT_TYPE_INT64 = 8,
    MAV_PARAM_EXT_TYPE_REAL32 = 9,
    MAV_PARAM_EXT_TYPE_REAL64 = 10,
    MAV_PARAM_EXT_TYPE_CUSTOM = 11,
}
impl Default for MavParamExtType {
    fn default() -> Self {
        MavParamExtType::MAV_PARAM_EXT_TYPE_UINT8
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavResult {
    MAV_RESULT_ACCEPTED = 0,
    MAV_RESULT_TEMPORARILY_REJECTED = 1,
    MAV_RESULT_DENIED = 2,
    MAV_RESULT_UNSUPPORTED = 3,
    MAV_RESULT_FAILED = 4,
    MAV_RESULT_IN_PROGRESS = 5,
    MAV_RESULT_CANCELLED = 6,
}
impl Default for MavResult {
    fn default() -> Self {
        MavResult::MAV_RESULT_ACCEPTED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMissionResult {
    MAV_MISSION_ACCEPTED = 0,
    MAV_MISSION_ERROR = 1,
    MAV_MISSION_UNSUPPORTED_FRAME = 2,
    MAV_MISSION_UNSUPPORTED = 3,
    MAV_MISSION_NO_SPACE = 4,
    MAV_MISSION_INVALID = 5,
    MAV_MISSION_INVALID_PARAM1 = 6,
    MAV_MISSION_INVALID_PARAM2 = 7,
    MAV_MISSION_INVALID_PARAM3 = 8,
    MAV_MISSION_INVALID_PARAM4 = 9,
    MAV_MISSION_INVALID_PARAM5_X = 10,
    MAV_MISSION_INVALID_PARAM6_Y = 11,
    MAV_MISSION_INVALID_PARAM7 = 12,
    MAV_MISSION_INVALID_SEQUENCE = 13,
    MAV_MISSION_DENIED = 14,
    MAV_MISSION_OPERATION_CANCELLED = 15,
}
impl Default for MavMissionResult {
    fn default() -> Self {
        MavMissionResult::MAV_MISSION_ACCEPTED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavSeverity {
    MAV_SEVERITY_EMERGENCY = 0,
    MAV_SEVERITY_ALERT = 1,
    MAV_SEVERITY_CRITICAL = 2,
    MAV_SEVERITY_ERROR = 3,
    MAV_SEVERITY_WARNING = 4,
    MAV_SEVERITY_NOTICE = 5,
    MAV_SEVERITY_INFO = 6,
    MAV_SEVERITY_DEBUG = 7,
}
impl Default for MavSeverity {
    fn default() -> Self {
        MavSeverity::MAV_SEVERITY_EMERGENCY
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavPowerStatus : u16 { const MAV_POWER_STATUS_BRICK_VALID = 1 ; const MAV_POWER_STATUS_SERVO_VALID = 2 ; const MAV_POWER_STATUS_USB_CONNECTED = 4 ; const MAV_POWER_STATUS_PERIPH_OVERCURRENT = 8 ; const MAV_POWER_STATUS_PERIPH_HIPOWER_OVERCURRENT = 16 ; const MAV_POWER_STATUS_CHANGED = 32 ; } }
impl Default for MavPowerStatus {
    fn default() -> Self {
        MavPowerStatus::MAV_POWER_STATUS_BRICK_VALID
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum SerialControlDev {
    SERIAL_CONTROL_DEV_TELEM1 = 0,
    SERIAL_CONTROL_DEV_TELEM2 = 1,
    SERIAL_CONTROL_DEV_GPS1 = 2,
    SERIAL_CONTROL_DEV_GPS2 = 3,
    SERIAL_CONTROL_DEV_SHELL = 10,
    SERIAL_CONTROL_SERIAL0 = 100,
    SERIAL_CONTROL_SERIAL1 = 101,
    SERIAL_CONTROL_SERIAL2 = 102,
    SERIAL_CONTROL_SERIAL3 = 103,
    SERIAL_CONTROL_SERIAL4 = 104,
    SERIAL_CONTROL_SERIAL5 = 105,
    SERIAL_CONTROL_SERIAL6 = 106,
    SERIAL_CONTROL_SERIAL7 = 107,
    SERIAL_CONTROL_SERIAL8 = 108,
    SERIAL_CONTROL_SERIAL9 = 109,
}
impl Default for SerialControlDev {
    fn default() -> Self {
        SerialControlDev::SERIAL_CONTROL_DEV_TELEM1
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct SerialControlFlag : u8 { const SERIAL_CONTROL_FLAG_REPLY = 1 ; const SERIAL_CONTROL_FLAG_RESPOND = 2 ; const SERIAL_CONTROL_FLAG_EXCLUSIVE = 4 ; const SERIAL_CONTROL_FLAG_BLOCKING = 8 ; const SERIAL_CONTROL_FLAG_MULTI = 16 ; } }
impl Default for SerialControlFlag {
    fn default() -> Self {
        SerialControlFlag::SERIAL_CONTROL_FLAG_REPLY
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavDistanceSensor {
    MAV_DISTANCE_SENSOR_LASER = 0,
    MAV_DISTANCE_SENSOR_ULTRASOUND = 1,
    MAV_DISTANCE_SENSOR_INFRARED = 2,
    MAV_DISTANCE_SENSOR_RADAR = 3,
    MAV_DISTANCE_SENSOR_UNKNOWN = 4,
}
impl Default for MavDistanceSensor {
    fn default() -> Self {
        MavDistanceSensor::MAV_DISTANCE_SENSOR_LASER
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavSensorOrientation {
    MAV_SENSOR_ROTATION_NONE = 0,
    MAV_SENSOR_ROTATION_YAW_45 = 1,
    MAV_SENSOR_ROTATION_YAW_90 = 2,
    MAV_SENSOR_ROTATION_YAW_135 = 3,
    MAV_SENSOR_ROTATION_YAW_180 = 4,
    MAV_SENSOR_ROTATION_YAW_225 = 5,
    MAV_SENSOR_ROTATION_YAW_270 = 6,
    MAV_SENSOR_ROTATION_YAW_315 = 7,
    MAV_SENSOR_ROTATION_ROLL_180 = 8,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_45 = 9,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_90 = 10,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_135 = 11,
    MAV_SENSOR_ROTATION_PITCH_180 = 12,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_225 = 13,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_270 = 14,
    MAV_SENSOR_ROTATION_ROLL_180_YAW_315 = 15,
    MAV_SENSOR_ROTATION_ROLL_90 = 16,
    MAV_SENSOR_ROTATION_ROLL_90_YAW_45 = 17,
    MAV_SENSOR_ROTATION_ROLL_90_YAW_90 = 18,
    MAV_SENSOR_ROTATION_ROLL_90_YAW_135 = 19,
    MAV_SENSOR_ROTATION_ROLL_270 = 20,
    MAV_SENSOR_ROTATION_ROLL_270_YAW_45 = 21,
    MAV_SENSOR_ROTATION_ROLL_270_YAW_90 = 22,
    MAV_SENSOR_ROTATION_ROLL_270_YAW_135 = 23,
    MAV_SENSOR_ROTATION_PITCH_90 = 24,
    MAV_SENSOR_ROTATION_PITCH_270 = 25,
    MAV_SENSOR_ROTATION_PITCH_180_YAW_90 = 26,
    MAV_SENSOR_ROTATION_PITCH_180_YAW_270 = 27,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_90 = 28,
    MAV_SENSOR_ROTATION_ROLL_180_PITCH_90 = 29,
    MAV_SENSOR_ROTATION_ROLL_270_PITCH_90 = 30,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_180 = 31,
    MAV_SENSOR_ROTATION_ROLL_270_PITCH_180 = 32,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_270 = 33,
    MAV_SENSOR_ROTATION_ROLL_180_PITCH_270 = 34,
    MAV_SENSOR_ROTATION_ROLL_270_PITCH_270 = 35,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_180_YAW_90 = 36,
    MAV_SENSOR_ROTATION_ROLL_90_YAW_270 = 37,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_68_YAW_293 = 38,
    MAV_SENSOR_ROTATION_PITCH_315 = 39,
    MAV_SENSOR_ROTATION_ROLL_90_PITCH_315 = 40,
    MAV_SENSOR_ROTATION_ROLL_270_YAW_180 = 41,
    MAV_SENSOR_ROTATION_CUSTOM = 100,
}
impl Default for MavSensorOrientation {
    fn default() -> Self {
        MavSensorOrientation::MAV_SENSOR_ROTATION_NONE
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavProtocolCapability : u64 { const MAV_PROTOCOL_CAPABILITY_MISSION_FLOAT = 1 ; const MAV_PROTOCOL_CAPABILITY_PARAM_FLOAT = 2 ; const MAV_PROTOCOL_CAPABILITY_MISSION_INT = 4 ; const MAV_PROTOCOL_CAPABILITY_COMMAND_INT = 8 ; const MAV_PROTOCOL_CAPABILITY_PARAM_UNION = 16 ; const MAV_PROTOCOL_CAPABILITY_FTP = 32 ; const MAV_PROTOCOL_CAPABILITY_SET_ATTITUDE_TARGET = 64 ; const MAV_PROTOCOL_CAPABILITY_SET_POSITION_TARGET_LOCAL_NED = 128 ; const MAV_PROTOCOL_CAPABILITY_SET_POSITION_TARGET_GLOBAL_INT = 256 ; const MAV_PROTOCOL_CAPABILITY_TERRAIN = 512 ; const MAV_PROTOCOL_CAPABILITY_SET_ACTUATOR_TARGET = 1024 ; const MAV_PROTOCOL_CAPABILITY_FLIGHT_TERMINATION = 2048 ; const MAV_PROTOCOL_CAPABILITY_COMPASS_CALIBRATION = 4096 ; const MAV_PROTOCOL_CAPABILITY_MAVLINK2 = 8192 ; const MAV_PROTOCOL_CAPABILITY_MISSION_FENCE = 16384 ; const MAV_PROTOCOL_CAPABILITY_MISSION_RALLY = 32768 ; const MAV_PROTOCOL_CAPABILITY_FLIGHT_INFORMATION = 65536 ; } }
impl Default for MavProtocolCapability {
    fn default() -> Self {
        MavProtocolCapability::MAV_PROTOCOL_CAPABILITY_MISSION_FLOAT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMissionType {
    MAV_MISSION_TYPE_MISSION = 0,
    MAV_MISSION_TYPE_FENCE = 1,
    MAV_MISSION_TYPE_RALLY = 2,
    MAV_MISSION_TYPE_ALL = 255,
}
impl Default for MavMissionType {
    fn default() -> Self {
        MavMissionType::MAV_MISSION_TYPE_MISSION
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavEstimatorType {
    MAV_ESTIMATOR_TYPE_UNKNOWN = 0,
    MAV_ESTIMATOR_TYPE_NAIVE = 1,
    MAV_ESTIMATOR_TYPE_VISION = 2,
    MAV_ESTIMATOR_TYPE_VIO = 3,
    MAV_ESTIMATOR_TYPE_GPS = 4,
    MAV_ESTIMATOR_TYPE_GPS_INS = 5,
    MAV_ESTIMATOR_TYPE_MOCAP = 6,
    MAV_ESTIMATOR_TYPE_LIDAR = 7,
    MAV_ESTIMATOR_TYPE_AUTOPILOT = 8,
}
impl Default for MavEstimatorType {
    fn default() -> Self {
        MavEstimatorType::MAV_ESTIMATOR_TYPE_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavBatteryType {
    MAV_BATTERY_TYPE_UNKNOWN = 0,
    MAV_BATTERY_TYPE_LIPO = 1,
    MAV_BATTERY_TYPE_LIFE = 2,
    MAV_BATTERY_TYPE_LION = 3,
    MAV_BATTERY_TYPE_NIMH = 4,
}
impl Default for MavBatteryType {
    fn default() -> Self {
        MavBatteryType::MAV_BATTERY_TYPE_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavBatteryFunction {
    MAV_BATTERY_FUNCTION_UNKNOWN = 0,
    MAV_BATTERY_FUNCTION_ALL = 1,
    MAV_BATTERY_FUNCTION_PROPULSION = 2,
    MAV_BATTERY_FUNCTION_AVIONICS = 3,
    MAV_BATTERY_TYPE_PAYLOAD = 4,
}
impl Default for MavBatteryFunction {
    fn default() -> Self {
        MavBatteryFunction::MAV_BATTERY_FUNCTION_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavBatteryChargeState {
    MAV_BATTERY_CHARGE_STATE_UNDEFINED = 0,
    MAV_BATTERY_CHARGE_STATE_OK = 1,
    MAV_BATTERY_CHARGE_STATE_LOW = 2,
    MAV_BATTERY_CHARGE_STATE_CRITICAL = 3,
    MAV_BATTERY_CHARGE_STATE_EMERGENCY = 4,
    MAV_BATTERY_CHARGE_STATE_FAILED = 5,
    MAV_BATTERY_CHARGE_STATE_UNHEALTHY = 6,
    MAV_BATTERY_CHARGE_STATE_CHARGING = 7,
}
impl Default for MavBatteryChargeState {
    fn default() -> Self {
        MavBatteryChargeState::MAV_BATTERY_CHARGE_STATE_UNDEFINED
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavSmartBatteryFault : i32 { const MAV_SMART_BATTERY_FAULT_DEEP_DISCHARGE = 1 ; const MAV_SMART_BATTERY_FAULT_SPIKES = 2 ; const MAV_SMART_BATTERY_FAULT_SINGLE_CELL_FAIL = 4 ; const MAV_SMART_BATTERY_FAULT_OVER_CURRENT = 8 ; const MAV_SMART_BATTERY_FAULT_OVER_TEMPERATURE = 16 ; const MAV_SMART_BATTERY_FAULT_UNDER_TEMPERATURE = 32 ; } }
impl Default for MavSmartBatteryFault {
    fn default() -> Self {
        MavSmartBatteryFault::MAV_SMART_BATTERY_FAULT_DEEP_DISCHARGE
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct MavGeneratorStatusFlag : u64 { const MAV_GENERATOR_STATUS_FLAG_OFF = 1 ; const MAV_GENERATOR_STATUS_FLAG_READY = 2 ; const MAV_GENERATOR_STATUS_FLAG_GENERATING = 4 ; const MAV_GENERATOR_STATUS_FLAG_CHARGING = 8 ; const MAV_GENERATOR_STATUS_FLAG_REDUCED_POWER = 16 ; const MAV_GENERATOR_STATUS_FLAG_MAXPOWER = 32 ; const MAV_GENERATOR_STATUS_FLAG_OVERTEMP_WARNING = 64 ; const MAV_GENERATOR_STATUS_FLAG_OVERTEMP_FAULT = 128 ; const MAV_GENERATOR_STATUS_FLAG_ELECTRONICS_OVERTEMP_WARNING = 256 ; const MAV_GENERATOR_STATUS_FLAG_ELECTRONICS_OVERTEMP_FAULT = 512 ; const MAV_GENERATOR_STATUS_FLAG_ELECTRONICS_FAULT = 1024 ; const MAV_GENERATOR_STATUS_FLAG_POWERSOURCE_FAULT = 2048 ; const MAV_GENERATOR_STATUS_FLAG_COMMUNICATION_WARNING = 4096 ; const MAV_GENERATOR_STATUS_FLAG_COOLING_WARNING = 8192 ; const MAV_GENERATOR_STATUS_FLAG_POWER_RAIL_FAULT = 16384 ; const MAV_GENERATOR_STATUS_FLAG_OVERCURRENT_FAULT = 32768 ; const MAV_GENERATOR_STATUS_FLAG_BATTERY_OVERCHARGE_CURRENT_FAULT = 65536 ; const MAV_GENERATOR_STATUS_FLAG_OVERVOLTAGE_FAULT = 131072 ; const MAV_GENERATOR_STATUS_FLAG_BATTERY_UNDERVOLT_FAULT = 262144 ; const MAV_GENERATOR_STATUS_FLAG_START_INHIBITED = 524288 ; const MAV_GENERATOR_STATUS_FLAG_MAINTENANCE_REQUIRED = 1048576 ; } }
impl Default for MavGeneratorStatusFlag {
    fn default() -> Self {
        MavGeneratorStatusFlag::MAV_GENERATOR_STATUS_FLAG_OFF
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavVtolState {
    MAV_VTOL_STATE_UNDEFINED = 0,
    MAV_VTOL_STATE_TRANSITION_TO_FW = 1,
    MAV_VTOL_STATE_TRANSITION_TO_MC = 2,
    MAV_VTOL_STATE_MC = 3,
    MAV_VTOL_STATE_FW = 4,
}
impl Default for MavVtolState {
    fn default() -> Self {
        MavVtolState::MAV_VTOL_STATE_UNDEFINED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavLandedState {
    MAV_LANDED_STATE_UNDEFINED = 0,
    MAV_LANDED_STATE_ON_GROUND = 1,
    MAV_LANDED_STATE_IN_AIR = 2,
    MAV_LANDED_STATE_TAKEOFF = 3,
    MAV_LANDED_STATE_LANDING = 4,
}
impl Default for MavLandedState {
    fn default() -> Self {
        MavLandedState::MAV_LANDED_STATE_UNDEFINED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AdsbAltitudeType {
    ADSB_ALTITUDE_TYPE_PRESSURE_QNH = 0,
    ADSB_ALTITUDE_TYPE_GEOMETRIC = 1,
}
impl Default for AdsbAltitudeType {
    fn default() -> Self {
        AdsbAltitudeType::ADSB_ALTITUDE_TYPE_PRESSURE_QNH
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AdsbEmitterType {
    ADSB_EMITTER_TYPE_NO_INFO = 0,
    ADSB_EMITTER_TYPE_LIGHT = 1,
    ADSB_EMITTER_TYPE_SMALL = 2,
    ADSB_EMITTER_TYPE_LARGE = 3,
    ADSB_EMITTER_TYPE_HIGH_VORTEX_LARGE = 4,
    ADSB_EMITTER_TYPE_HEAVY = 5,
    ADSB_EMITTER_TYPE_HIGHLY_MANUV = 6,
    ADSB_EMITTER_TYPE_ROTOCRAFT = 7,
    ADSB_EMITTER_TYPE_UNASSIGNED = 8,
    ADSB_EMITTER_TYPE_GLIDER = 9,
    ADSB_EMITTER_TYPE_LIGHTER_AIR = 10,
    ADSB_EMITTER_TYPE_PARACHUTE = 11,
    ADSB_EMITTER_TYPE_ULTRA_LIGHT = 12,
    ADSB_EMITTER_TYPE_UNASSIGNED2 = 13,
    ADSB_EMITTER_TYPE_UAV = 14,
    ADSB_EMITTER_TYPE_SPACE = 15,
    ADSB_EMITTER_TYPE_UNASSGINED3 = 16,
    ADSB_EMITTER_TYPE_EMERGENCY_SURFACE = 17,
    ADSB_EMITTER_TYPE_SERVICE_SURFACE = 18,
    ADSB_EMITTER_TYPE_POINT_OBSTACLE = 19,
}
impl Default for AdsbEmitterType {
    fn default() -> Self {
        AdsbEmitterType::ADSB_EMITTER_TYPE_NO_INFO
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct AdsbFlags : u16 { const ADSB_FLAGS_VALID_COORDS = 1 ; const ADSB_FLAGS_VALID_ALTITUDE = 2 ; const ADSB_FLAGS_VALID_HEADING = 4 ; const ADSB_FLAGS_VALID_VELOCITY = 8 ; const ADSB_FLAGS_VALID_CALLSIGN = 16 ; const ADSB_FLAGS_VALID_SQUAWK = 32 ; const ADSB_FLAGS_SIMULATED = 64 ; const ADSB_FLAGS_VERTICAL_VELOCITY_VALID = 128 ; const ADSB_FLAGS_BARO_VALID = 256 ; const ADSB_FLAGS_SOURCE_UAT = 32768 ; } }
impl Default for AdsbFlags {
    fn default() -> Self {
        AdsbFlags::ADSB_FLAGS_VALID_COORDS
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavDoRepositionFlags {
    MAV_DO_REPOSITION_FLAGS_CHANGE_MODE = 1,
}
impl Default for MavDoRepositionFlags {
    fn default() -> Self {
        MavDoRepositionFlags::MAV_DO_REPOSITION_FLAGS_CHANGE_MODE
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct EstimatorStatusFlags : u16 { const ESTIMATOR_ATTITUDE = 1 ; const ESTIMATOR_VELOCITY_HORIZ = 2 ; const ESTIMATOR_VELOCITY_VERT = 4 ; const ESTIMATOR_POS_HORIZ_REL = 8 ; const ESTIMATOR_POS_HORIZ_ABS = 16 ; const ESTIMATOR_POS_VERT_ABS = 32 ; const ESTIMATOR_POS_VERT_AGL = 64 ; const ESTIMATOR_CONST_POS_MODE = 128 ; const ESTIMATOR_PRED_POS_HORIZ_REL = 256 ; const ESTIMATOR_PRED_POS_HORIZ_ABS = 512 ; const ESTIMATOR_GPS_GLITCH = 1024 ; const ESTIMATOR_ACCEL_ERROR = 2048 ; } }
impl Default for EstimatorStatusFlags {
    fn default() -> Self {
        EstimatorStatusFlags::ESTIMATOR_ATTITUDE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MotorTestOrder {
    MOTOR_TEST_ORDER_DEFAULT = 0,
    MOTOR_TEST_ORDER_SEQUENCE = 1,
    MOTOR_TEST_ORDER_BOARD = 2,
}
impl Default for MotorTestOrder {
    fn default() -> Self {
        MotorTestOrder::MOTOR_TEST_ORDER_DEFAULT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MotorTestThrottleType {
    MOTOR_TEST_THROTTLE_PERCENT = 0,
    MOTOR_TEST_THROTTLE_PWM = 1,
    MOTOR_TEST_THROTTLE_PILOT = 2,
    MOTOR_TEST_COMPASS_CAL = 3,
}
impl Default for MotorTestThrottleType {
    fn default() -> Self {
        MotorTestThrottleType::MOTOR_TEST_THROTTLE_PERCENT
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct GpsInputIgnoreFlags : u16 { const GPS_INPUT_IGNORE_FLAG_ALT = 1 ; const GPS_INPUT_IGNORE_FLAG_HDOP = 2 ; const GPS_INPUT_IGNORE_FLAG_VDOP = 4 ; const GPS_INPUT_IGNORE_FLAG_VEL_HORIZ = 8 ; const GPS_INPUT_IGNORE_FLAG_VEL_VERT = 16 ; const GPS_INPUT_IGNORE_FLAG_SPEED_ACCURACY = 32 ; const GPS_INPUT_IGNORE_FLAG_HORIZONTAL_ACCURACY = 64 ; const GPS_INPUT_IGNORE_FLAG_VERTICAL_ACCURACY = 128 ; } }
impl Default for GpsInputIgnoreFlags {
    fn default() -> Self {
        GpsInputIgnoreFlags::GPS_INPUT_IGNORE_FLAG_ALT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCollisionAction {
    MAV_COLLISION_ACTION_NONE = 0,
    MAV_COLLISION_ACTION_REPORT = 1,
    MAV_COLLISION_ACTION_ASCEND_OR_DESCEND = 2,
    MAV_COLLISION_ACTION_MOVE_HORIZONTALLY = 3,
    MAV_COLLISION_ACTION_MOVE_PERPENDICULAR = 4,
    MAV_COLLISION_ACTION_RTL = 5,
    MAV_COLLISION_ACTION_HOVER = 6,
}
impl Default for MavCollisionAction {
    fn default() -> Self {
        MavCollisionAction::MAV_COLLISION_ACTION_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCollisionThreatLevel {
    MAV_COLLISION_THREAT_LEVEL_NONE = 0,
    MAV_COLLISION_THREAT_LEVEL_LOW = 1,
    MAV_COLLISION_THREAT_LEVEL_HIGH = 2,
}
impl Default for MavCollisionThreatLevel {
    fn default() -> Self {
        MavCollisionThreatLevel::MAV_COLLISION_THREAT_LEVEL_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCollisionSrc {
    MAV_COLLISION_SRC_ADSB = 0,
    MAV_COLLISION_SRC_MAVLINK_GPS_GLOBAL_INT = 1,
}
impl Default for MavCollisionSrc {
    fn default() -> Self {
        MavCollisionSrc::MAV_COLLISION_SRC_ADSB
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GpsFixType {
    GPS_FIX_TYPE_NO_GPS = 0,
    GPS_FIX_TYPE_NO_FIX = 1,
    GPS_FIX_TYPE_2D_FIX = 2,
    GPS_FIX_TYPE_3D_FIX = 3,
    GPS_FIX_TYPE_DGPS = 4,
    GPS_FIX_TYPE_RTK_FLOAT = 5,
    GPS_FIX_TYPE_RTK_FIXED = 6,
    GPS_FIX_TYPE_STATIC = 7,
    GPS_FIX_TYPE_PPP = 8,
}
impl Default for GpsFixType {
    fn default() -> Self {
        GpsFixType::GPS_FIX_TYPE_NO_GPS
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum RtkBaselineCoordinateSystem {
    RTK_BASELINE_COORDINATE_SYSTEM_ECEF = 0,
    RTK_BASELINE_COORDINATE_SYSTEM_NED = 1,
}
impl Default for RtkBaselineCoordinateSystem {
    fn default() -> Self {
        RtkBaselineCoordinateSystem::RTK_BASELINE_COORDINATE_SYSTEM_ECEF
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum LandingTargetType {
    LANDING_TARGET_TYPE_LIGHT_BEACON = 0,
    LANDING_TARGET_TYPE_RADIO_BEACON = 1,
    LANDING_TARGET_TYPE_VISION_FIDUCIAL = 2,
    LANDING_TARGET_TYPE_VISION_OTHER = 3,
}
impl Default for LandingTargetType {
    fn default() -> Self {
        LandingTargetType::LANDING_TARGET_TYPE_LIGHT_BEACON
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum VtolTransitionHeading {
    VTOL_TRANSITION_HEADING_VEHICLE_DEFAULT = 0,
    VTOL_TRANSITION_HEADING_NEXT_WAYPOINT = 1,
    VTOL_TRANSITION_HEADING_TAKEOFF = 2,
    VTOL_TRANSITION_HEADING_SPECIFIED = 3,
    VTOL_TRANSITION_HEADING_ANY = 4,
}
impl Default for VtolTransitionHeading {
    fn default() -> Self {
        VtolTransitionHeading::VTOL_TRANSITION_HEADING_VEHICLE_DEFAULT
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct CameraCapFlags : u32 { const CAMERA_CAP_FLAGS_CAPTURE_VIDEO = 1 ; const CAMERA_CAP_FLAGS_CAPTURE_IMAGE = 2 ; const CAMERA_CAP_FLAGS_HAS_MODES = 4 ; const CAMERA_CAP_FLAGS_CAN_CAPTURE_IMAGE_IN_VIDEO_MODE = 8 ; const CAMERA_CAP_FLAGS_CAN_CAPTURE_VIDEO_IN_IMAGE_MODE = 16 ; const CAMERA_CAP_FLAGS_HAS_IMAGE_SURVEY_MODE = 32 ; const CAMERA_CAP_FLAGS_HAS_BASIC_ZOOM = 64 ; const CAMERA_CAP_FLAGS_HAS_BASIC_FOCUS = 128 ; const CAMERA_CAP_FLAGS_HAS_VIDEO_STREAM = 256 ; } }
impl Default for CameraCapFlags {
    fn default() -> Self {
        CameraCapFlags::CAMERA_CAP_FLAGS_CAPTURE_VIDEO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum VideoStreamStatusFlags {
    VIDEO_STREAM_STATUS_FLAGS_RUNNING = 1,
    VIDEO_STREAM_STATUS_FLAGS_THERMAL = 2,
}
impl Default for VideoStreamStatusFlags {
    fn default() -> Self {
        VideoStreamStatusFlags::VIDEO_STREAM_STATUS_FLAGS_RUNNING
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum VideoStreamType {
    VIDEO_STREAM_TYPE_RTSP = 0,
    VIDEO_STREAM_TYPE_RTPUDP = 1,
    VIDEO_STREAM_TYPE_TCP_MPEG = 2,
    VIDEO_STREAM_TYPE_MPEG_TS_H264 = 3,
}
impl Default for VideoStreamType {
    fn default() -> Self {
        VideoStreamType::VIDEO_STREAM_TYPE_RTSP
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CameraZoomType {
    ZOOM_TYPE_STEP = 0,
    ZOOM_TYPE_CONTINUOUS = 1,
    ZOOM_TYPE_RANGE = 2,
    ZOOM_TYPE_FOCAL_LENGTH = 3,
}
impl Default for CameraZoomType {
    fn default() -> Self {
        CameraZoomType::ZOOM_TYPE_STEP
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum SetFocusType {
    FOCUS_TYPE_STEP = 0,
    FOCUS_TYPE_CONTINUOUS = 1,
    FOCUS_TYPE_RANGE = 2,
    FOCUS_TYPE_METERS = 3,
}
impl Default for SetFocusType {
    fn default() -> Self {
        SetFocusType::FOCUS_TYPE_STEP
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum ParamAck {
    PARAM_ACK_ACCEPTED = 0,
    PARAM_ACK_VALUE_UNSUPPORTED = 1,
    PARAM_ACK_FAILED = 2,
    PARAM_ACK_IN_PROGRESS = 3,
}
impl Default for ParamAck {
    fn default() -> Self {
        ParamAck::PARAM_ACK_ACCEPTED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CameraMode {
    CAMERA_MODE_IMAGE = 0,
    CAMERA_MODE_VIDEO = 1,
    CAMERA_MODE_IMAGE_SURVEY = 2,
}
impl Default for CameraMode {
    fn default() -> Self {
        CameraMode::CAMERA_MODE_IMAGE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavArmAuthDeniedReason {
    MAV_ARM_AUTH_DENIED_REASON_GENERIC = 0,
    MAV_ARM_AUTH_DENIED_REASON_NONE = 1,
    MAV_ARM_AUTH_DENIED_REASON_INVALID_WAYPOINT = 2,
    MAV_ARM_AUTH_DENIED_REASON_TIMEOUT = 3,
    MAV_ARM_AUTH_DENIED_REASON_AIRSPACE_IN_USE = 4,
    MAV_ARM_AUTH_DENIED_REASON_BAD_WEATHER = 5,
}
impl Default for MavArmAuthDeniedReason {
    fn default() -> Self {
        MavArmAuthDeniedReason::MAV_ARM_AUTH_DENIED_REASON_GENERIC
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum RcType {
    RC_TYPE_SPEKTRUM_DSM2 = 0,
    RC_TYPE_SPEKTRUM_DSMX = 1,
}
impl Default for RcType {
    fn default() -> Self {
        RcType::RC_TYPE_SPEKTRUM_DSM2
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct PositionTargetTypemask : u16 { const POSITION_TARGET_TYPEMASK_X_IGNORE = 1 ; const POSITION_TARGET_TYPEMASK_Y_IGNORE = 2 ; const POSITION_TARGET_TYPEMASK_Z_IGNORE = 4 ; const POSITION_TARGET_TYPEMASK_VX_IGNORE = 8 ; const POSITION_TARGET_TYPEMASK_VY_IGNORE = 16 ; const POSITION_TARGET_TYPEMASK_VZ_IGNORE = 32 ; const POSITION_TARGET_TYPEMASK_AX_IGNORE = 64 ; const POSITION_TARGET_TYPEMASK_AY_IGNORE = 128 ; const POSITION_TARGET_TYPEMASK_AZ_IGNORE = 256 ; const POSITION_TARGET_TYPEMASK_FORCE_SET = 512 ; const POSITION_TARGET_TYPEMASK_YAW_IGNORE = 1024 ; const POSITION_TARGET_TYPEMASK_YAW_RATE_IGNORE = 2048 ; } }
impl Default for PositionTargetTypemask {
    fn default() -> Self {
        PositionTargetTypemask::POSITION_TARGET_TYPEMASK_X_IGNORE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum UtmFlightState {
    UTM_FLIGHT_STATE_UNKNOWN = 1,
    UTM_FLIGHT_STATE_GROUND = 2,
    UTM_FLIGHT_STATE_AIRBORNE = 3,
    UTM_FLIGHT_STATE_EMERGENCY = 16,
    UTM_FLIGHT_STATE_NOCTRL = 32,
}
impl Default for UtmFlightState {
    fn default() -> Self {
        UtmFlightState::UTM_FLIGHT_STATE_UNKNOWN
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct UtmDataAvailFlags : u8 { const UTM_DATA_AVAIL_FLAGS_TIME_VALID = 1 ; const UTM_DATA_AVAIL_FLAGS_UAS_ID_AVAILABLE = 2 ; const UTM_DATA_AVAIL_FLAGS_POSITION_AVAILABLE = 4 ; const UTM_DATA_AVAIL_FLAGS_ALTITUDE_AVAILABLE = 8 ; const UTM_DATA_AVAIL_FLAGS_RELATIVE_ALTITUDE_AVAILABLE = 16 ; const UTM_DATA_AVAIL_FLAGS_HORIZONTAL_VELO_AVAILABLE = 32 ; const UTM_DATA_AVAIL_FLAGS_VERTICAL_VELO_AVAILABLE = 64 ; const UTM_DATA_AVAIL_FLAGS_NEXT_WAYPOINT_AVAILABLE = 128 ; } }
impl Default for UtmDataAvailFlags {
    fn default() -> Self {
        UtmDataAvailFlags::UTM_DATA_AVAIL_FLAGS_TIME_VALID
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CellularNetworkRadioType {
    CELLULAR_NETWORK_RADIO_TYPE_NONE = 0,
    CELLULAR_NETWORK_RADIO_TYPE_GSM = 1,
    CELLULAR_NETWORK_RADIO_TYPE_CDMA = 2,
    CELLULAR_NETWORK_RADIO_TYPE_WCDMA = 3,
    CELLULAR_NETWORK_RADIO_TYPE_LTE = 4,
}
impl Default for CellularNetworkRadioType {
    fn default() -> Self {
        CellularNetworkRadioType::CELLULAR_NETWORK_RADIO_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CellularStatusFlag {
    CELLULAR_STATUS_FLAG_UNKNOWN = 0,
    CELLULAR_STATUS_FLAG_FAILED = 1,
    CELLULAR_STATUS_FLAG_INITIALIZING = 2,
    CELLULAR_STATUS_FLAG_LOCKED = 3,
    CELLULAR_STATUS_FLAG_DISABLED = 4,
    CELLULAR_STATUS_FLAG_DISABLING = 5,
    CELLULAR_STATUS_FLAG_ENABLING = 6,
    CELLULAR_STATUS_FLAG_ENABLED = 7,
    CELLULAR_STATUS_FLAG_SEARCHING = 8,
    CELLULAR_STATUS_FLAG_REGISTERED = 9,
    CELLULAR_STATUS_FLAG_DISCONNECTING = 10,
    CELLULAR_STATUS_FLAG_CONNECTING = 11,
    CELLULAR_STATUS_FLAG_CONNECTED = 12,
}
impl Default for CellularStatusFlag {
    fn default() -> Self {
        CellularStatusFlag::CELLULAR_STATUS_FLAG_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CellularNetworkFailedReason {
    CELLULAR_NETWORK_FAILED_REASON_NONE = 0,
    CELLULAR_NETWORK_FAILED_REASON_UNKNOWN = 1,
    CELLULAR_NETWORK_FAILED_REASON_SIM_MISSING = 2,
    CELLULAR_NETWORK_FAILED_REASON_SIM_ERROR = 3,
}
impl Default for CellularNetworkFailedReason {
    fn default() -> Self {
        CellularNetworkFailedReason::CELLULAR_NETWORK_FAILED_REASON_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PrecisionLandMode {
    PRECISION_LAND_MODE_DISABLED = 0,
    PRECISION_LAND_MODE_OPPORTUNISTIC = 1,
    PRECISION_LAND_MODE_REQUIRED = 2,
}
impl Default for PrecisionLandMode {
    fn default() -> Self {
        PrecisionLandMode::PRECISION_LAND_MODE_DISABLED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum ParachuteAction {
    PARACHUTE_DISABLE = 0,
    PARACHUTE_ENABLE = 1,
    PARACHUTE_RELEASE = 2,
}
impl Default for ParachuteAction {
    fn default() -> Self {
        ParachuteAction::PARACHUTE_DISABLE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavTunnelPayloadType {
    MAV_TUNNEL_PAYLOAD_TYPE_UNKNOWN = 0,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED0 = 200,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED1 = 201,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED2 = 202,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED3 = 203,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED4 = 204,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED5 = 205,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED6 = 206,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED7 = 207,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED8 = 208,
    MAV_TUNNEL_PAYLOAD_TYPE_STORM32_RESERVED9 = 209,
}
impl Default for MavTunnelPayloadType {
    fn default() -> Self {
        MavTunnelPayloadType::MAV_TUNNEL_PAYLOAD_TYPE_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidIdType {
    MAV_ODID_ID_TYPE_NONE = 0,
    MAV_ODID_ID_TYPE_SERIAL_NUMBER = 1,
    MAV_ODID_ID_TYPE_CAA_REGISTRATION_ID = 2,
    MAV_ODID_ID_TYPE_UTM_ASSIGNED_UUID = 3,
}
impl Default for MavOdidIdType {
    fn default() -> Self {
        MavOdidIdType::MAV_ODID_ID_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidUaType {
    MAV_ODID_UA_TYPE_NONE = 0,
    MAV_ODID_UA_TYPE_AEROPLANE = 1,
    MAV_ODID_UA_TYPE_HELICOPTER_OR_MULTIROTOR = 2,
    MAV_ODID_UA_TYPE_GYROPLANE = 3,
    MAV_ODID_UA_TYPE_HYBRID_LIFT = 4,
    MAV_ODID_UA_TYPE_ORNITHOPTER = 5,
    MAV_ODID_UA_TYPE_GLIDER = 6,
    MAV_ODID_UA_TYPE_KITE = 7,
    MAV_ODID_UA_TYPE_FREE_BALLOON = 8,
    MAV_ODID_UA_TYPE_CAPTIVE_BALLOON = 9,
    MAV_ODID_UA_TYPE_AIRSHIP = 10,
    MAV_ODID_UA_TYPE_FREE_FALL_PARACHUTE = 11,
    MAV_ODID_UA_TYPE_ROCKET = 12,
    MAV_ODID_UA_TYPE_TETHERED_POWERED_AIRCRAFT = 13,
    MAV_ODID_UA_TYPE_GROUND_OBSTACLE = 14,
    MAV_ODID_UA_TYPE_OTHER = 15,
}
impl Default for MavOdidUaType {
    fn default() -> Self {
        MavOdidUaType::MAV_ODID_UA_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidStatus {
    MAV_ODID_STATUS_UNDECLARED = 0,
    MAV_ODID_STATUS_GROUND = 1,
    MAV_ODID_STATUS_AIRBORNE = 2,
    MAV_ODID_STATUS_EMERGENCY = 3,
}
impl Default for MavOdidStatus {
    fn default() -> Self {
        MavOdidStatus::MAV_ODID_STATUS_UNDECLARED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidHeightRef {
    MAV_ODID_HEIGHT_REF_OVER_TAKEOFF = 0,
    MAV_ODID_HEIGHT_REF_OVER_GROUND = 1,
}
impl Default for MavOdidHeightRef {
    fn default() -> Self {
        MavOdidHeightRef::MAV_ODID_HEIGHT_REF_OVER_TAKEOFF
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidHorAcc {
    MAV_ODID_HOR_ACC_UNKNOWN = 0,
    MAV_ODID_HOR_ACC_10NM = 1,
    MAV_ODID_HOR_ACC_4NM = 2,
    MAV_ODID_HOR_ACC_2NM = 3,
    MAV_ODID_HOR_ACC_1NM = 4,
    MAV_ODID_HOR_ACC_0_5NM = 5,
    MAV_ODID_HOR_ACC_0_3NM = 6,
    MAV_ODID_HOR_ACC_0_1NM = 7,
    MAV_ODID_HOR_ACC_0_05NM = 8,
    MAV_ODID_HOR_ACC_30_METER = 9,
    MAV_ODID_HOR_ACC_10_METER = 10,
    MAV_ODID_HOR_ACC_3_METER = 11,
    MAV_ODID_HOR_ACC_1_METER = 12,
}
impl Default for MavOdidHorAcc {
    fn default() -> Self {
        MavOdidHorAcc::MAV_ODID_HOR_ACC_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidVerAcc {
    MAV_ODID_VER_ACC_UNKNOWN = 0,
    MAV_ODID_VER_ACC_150_METER = 1,
    MAV_ODID_VER_ACC_45_METER = 2,
    MAV_ODID_VER_ACC_25_METER = 3,
    MAV_ODID_VER_ACC_10_METER = 4,
    MAV_ODID_VER_ACC_3_METER = 5,
    MAV_ODID_VER_ACC_1_METER = 6,
}
impl Default for MavOdidVerAcc {
    fn default() -> Self {
        MavOdidVerAcc::MAV_ODID_VER_ACC_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidSpeedAcc {
    MAV_ODID_SPEED_ACC_UNKNOWN = 0,
    MAV_ODID_SPEED_ACC_10_METERS_PER_SECOND = 1,
    MAV_ODID_SPEED_ACC_3_METERS_PER_SECOND = 2,
    MAV_ODID_SPEED_ACC_1_METERS_PER_SECOND = 3,
    MAV_ODID_SPEED_ACC_0_3_METERS_PER_SECOND = 4,
}
impl Default for MavOdidSpeedAcc {
    fn default() -> Self {
        MavOdidSpeedAcc::MAV_ODID_SPEED_ACC_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidTimeAcc {
    MAV_ODID_TIME_ACC_UNKNOWN = 0,
    MAV_ODID_TIME_ACC_0_1_SECOND = 1,
    MAV_ODID_TIME_ACC_0_2_SECOND = 2,
    MAV_ODID_TIME_ACC_0_3_SECOND = 3,
    MAV_ODID_TIME_ACC_0_4_SECOND = 4,
    MAV_ODID_TIME_ACC_0_5_SECOND = 5,
    MAV_ODID_TIME_ACC_0_6_SECOND = 6,
    MAV_ODID_TIME_ACC_0_7_SECOND = 7,
    MAV_ODID_TIME_ACC_0_8_SECOND = 8,
    MAV_ODID_TIME_ACC_0_9_SECOND = 9,
    MAV_ODID_TIME_ACC_1_0_SECOND = 10,
    MAV_ODID_TIME_ACC_1_1_SECOND = 11,
    MAV_ODID_TIME_ACC_1_2_SECOND = 12,
    MAV_ODID_TIME_ACC_1_3_SECOND = 13,
    MAV_ODID_TIME_ACC_1_4_SECOND = 14,
    MAV_ODID_TIME_ACC_1_5_SECOND = 15,
}
impl Default for MavOdidTimeAcc {
    fn default() -> Self {
        MavOdidTimeAcc::MAV_ODID_TIME_ACC_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidAuthType {
    MAV_ODID_AUTH_TYPE_NONE = 0,
    MAV_ODID_AUTH_TYPE_UAS_ID_SIGNATURE = 1,
    MAV_ODID_AUTH_TYPE_OPERATOR_ID_SIGNATURE = 2,
    MAV_ODID_AUTH_TYPE_MESSAGE_SET_SIGNATURE = 3,
    MAV_ODID_AUTH_TYPE_NETWORK_REMOTE_ID = 4,
}
impl Default for MavOdidAuthType {
    fn default() -> Self {
        MavOdidAuthType::MAV_ODID_AUTH_TYPE_NONE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidDescType {
    MAV_ODID_DESC_TYPE_TEXT = 0,
}
impl Default for MavOdidDescType {
    fn default() -> Self {
        MavOdidDescType::MAV_ODID_DESC_TYPE_TEXT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidOperatorLocationType {
    MAV_ODID_OPERATOR_LOCATION_TYPE_TAKEOFF = 0,
    MAV_ODID_OPERATOR_LOCATION_TYPE_LIVE_GNSS = 1,
    MAV_ODID_OPERATOR_LOCATION_TYPE_FIXED = 2,
}
impl Default for MavOdidOperatorLocationType {
    fn default() -> Self {
        MavOdidOperatorLocationType::MAV_ODID_OPERATOR_LOCATION_TYPE_TAKEOFF
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidClassificationType {
    MAV_ODID_CLASSIFICATION_TYPE_UNDECLARED = 0,
    MAV_ODID_CLASSIFICATION_TYPE_EU = 1,
}
impl Default for MavOdidClassificationType {
    fn default() -> Self {
        MavOdidClassificationType::MAV_ODID_CLASSIFICATION_TYPE_UNDECLARED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidCategoryEu {
    MAV_ODID_CATEGORY_EU_UNDECLARED = 0,
    MAV_ODID_CATEGORY_EU_OPEN = 1,
    MAV_ODID_CATEGORY_EU_SPECIFIC = 2,
    MAV_ODID_CATEGORY_EU_CERTIFIED = 3,
}
impl Default for MavOdidCategoryEu {
    fn default() -> Self {
        MavOdidCategoryEu::MAV_ODID_CATEGORY_EU_UNDECLARED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidClassEu {
    MAV_ODID_CLASS_EU_UNDECLARED = 0,
    MAV_ODID_CLASS_EU_CLASS_0 = 1,
    MAV_ODID_CLASS_EU_CLASS_1 = 2,
    MAV_ODID_CLASS_EU_CLASS_2 = 3,
    MAV_ODID_CLASS_EU_CLASS_3 = 4,
    MAV_ODID_CLASS_EU_CLASS_4 = 5,
    MAV_ODID_CLASS_EU_CLASS_5 = 6,
    MAV_ODID_CLASS_EU_CLASS_6 = 7,
}
impl Default for MavOdidClassEu {
    fn default() -> Self {
        MavOdidClassEu::MAV_ODID_CLASS_EU_UNDECLARED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavOdidOperatorIdType {
    MAV_ODID_OPERATOR_ID_TYPE_CAA = 0,
}
impl Default for MavOdidOperatorIdType {
    fn default() -> Self {
        MavOdidOperatorIdType::MAV_ODID_OPERATOR_ID_TYPE_CAA
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct TuneFormat : u32 { const TUNE_FORMAT_QBASIC1_1 = 1 ; const TUNE_FORMAT_MML_MODERN = 2 ; } }
impl Default for TuneFormat {
    fn default() -> Self {
        TuneFormat::TUNE_FORMAT_QBASIC1_1
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum ComponentCapFlags {
    COMPONENT_CAP_FLAGS_PARAM = 1,
    COMPONENT_CAP_FLAGS_PARAM_EXT = 2,
}
impl Default for ComponentCapFlags {
    fn default() -> Self {
        ComponentCapFlags::COMPONENT_CAP_FLAGS_PARAM
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AisType {
    AIS_TYPE_UNKNOWN = 0,
    AIS_TYPE_RESERVED_1 = 1,
    AIS_TYPE_RESERVED_2 = 2,
    AIS_TYPE_RESERVED_3 = 3,
    AIS_TYPE_RESERVED_4 = 4,
    AIS_TYPE_RESERVED_5 = 5,
    AIS_TYPE_RESERVED_6 = 6,
    AIS_TYPE_RESERVED_7 = 7,
    AIS_TYPE_RESERVED_8 = 8,
    AIS_TYPE_RESERVED_9 = 9,
    AIS_TYPE_RESERVED_10 = 10,
    AIS_TYPE_RESERVED_11 = 11,
    AIS_TYPE_RESERVED_12 = 12,
    AIS_TYPE_RESERVED_13 = 13,
    AIS_TYPE_RESERVED_14 = 14,
    AIS_TYPE_RESERVED_15 = 15,
    AIS_TYPE_RESERVED_16 = 16,
    AIS_TYPE_RESERVED_17 = 17,
    AIS_TYPE_RESERVED_18 = 18,
    AIS_TYPE_RESERVED_19 = 19,
    AIS_TYPE_WIG = 20,
    AIS_TYPE_WIG_HAZARDOUS_A = 21,
    AIS_TYPE_WIG_HAZARDOUS_B = 22,
    AIS_TYPE_WIG_HAZARDOUS_C = 23,
    AIS_TYPE_WIG_HAZARDOUS_D = 24,
    AIS_TYPE_WIG_RESERVED_1 = 25,
    AIS_TYPE_WIG_RESERVED_2 = 26,
    AIS_TYPE_WIG_RESERVED_3 = 27,
    AIS_TYPE_WIG_RESERVED_4 = 28,
    AIS_TYPE_WIG_RESERVED_5 = 29,
    AIS_TYPE_FISHING = 30,
    AIS_TYPE_TOWING = 31,
    AIS_TYPE_TOWING_LARGE = 32,
    AIS_TYPE_DREDGING = 33,
    AIS_TYPE_DIVING = 34,
    AIS_TYPE_MILITARY = 35,
    AIS_TYPE_SAILING = 36,
    AIS_TYPE_PLEASURE = 37,
    AIS_TYPE_RESERVED_20 = 38,
    AIS_TYPE_RESERVED_21 = 39,
    AIS_TYPE_HSC = 40,
    AIS_TYPE_HSC_HAZARDOUS_A = 41,
    AIS_TYPE_HSC_HAZARDOUS_B = 42,
    AIS_TYPE_HSC_HAZARDOUS_C = 43,
    AIS_TYPE_HSC_HAZARDOUS_D = 44,
    AIS_TYPE_HSC_RESERVED_1 = 45,
    AIS_TYPE_HSC_RESERVED_2 = 46,
    AIS_TYPE_HSC_RESERVED_3 = 47,
    AIS_TYPE_HSC_RESERVED_4 = 48,
    AIS_TYPE_HSC_UNKNOWN = 49,
    AIS_TYPE_PILOT = 50,
    AIS_TYPE_SAR = 51,
    AIS_TYPE_TUG = 52,
    AIS_TYPE_PORT_TENDER = 53,
    AIS_TYPE_ANTI_POLLUTION = 54,
    AIS_TYPE_LAW_ENFORCEMENT = 55,
    AIS_TYPE_SPARE_LOCAL_1 = 56,
    AIS_TYPE_SPARE_LOCAL_2 = 57,
    AIS_TYPE_MEDICAL_TRANSPORT = 58,
    AIS_TYPE_NONECOMBATANT = 59,
    AIS_TYPE_PASSENGER = 60,
    AIS_TYPE_PASSENGER_HAZARDOUS_A = 61,
    AIS_TYPE_PASSENGER_HAZARDOUS_B = 62,
    AIS_TYPE_AIS_TYPE_PASSENGER_HAZARDOUS_C = 63,
    AIS_TYPE_PASSENGER_HAZARDOUS_D = 64,
    AIS_TYPE_PASSENGER_RESERVED_1 = 65,
    AIS_TYPE_PASSENGER_RESERVED_2 = 66,
    AIS_TYPE_PASSENGER_RESERVED_3 = 67,
    AIS_TYPE_AIS_TYPE_PASSENGER_RESERVED_4 = 68,
    AIS_TYPE_PASSENGER_UNKNOWN = 69,
    AIS_TYPE_CARGO = 70,
    AIS_TYPE_CARGO_HAZARDOUS_A = 71,
    AIS_TYPE_CARGO_HAZARDOUS_B = 72,
    AIS_TYPE_CARGO_HAZARDOUS_C = 73,
    AIS_TYPE_CARGO_HAZARDOUS_D = 74,
    AIS_TYPE_CARGO_RESERVED_1 = 75,
    AIS_TYPE_CARGO_RESERVED_2 = 76,
    AIS_TYPE_CARGO_RESERVED_3 = 77,
    AIS_TYPE_CARGO_RESERVED_4 = 78,
    AIS_TYPE_CARGO_UNKNOWN = 79,
    AIS_TYPE_TANKER = 80,
    AIS_TYPE_TANKER_HAZARDOUS_A = 81,
    AIS_TYPE_TANKER_HAZARDOUS_B = 82,
    AIS_TYPE_TANKER_HAZARDOUS_C = 83,
    AIS_TYPE_TANKER_HAZARDOUS_D = 84,
    AIS_TYPE_TANKER_RESERVED_1 = 85,
    AIS_TYPE_TANKER_RESERVED_2 = 86,
    AIS_TYPE_TANKER_RESERVED_3 = 87,
    AIS_TYPE_TANKER_RESERVED_4 = 88,
    AIS_TYPE_TANKER_UNKNOWN = 89,
    AIS_TYPE_OTHER = 90,
    AIS_TYPE_OTHER_HAZARDOUS_A = 91,
    AIS_TYPE_OTHER_HAZARDOUS_B = 92,
    AIS_TYPE_OTHER_HAZARDOUS_C = 93,
    AIS_TYPE_OTHER_HAZARDOUS_D = 94,
    AIS_TYPE_OTHER_RESERVED_1 = 95,
    AIS_TYPE_OTHER_RESERVED_2 = 96,
    AIS_TYPE_OTHER_RESERVED_3 = 97,
    AIS_TYPE_OTHER_RESERVED_4 = 98,
    AIS_TYPE_OTHER_UNKNOWN = 99,
}
impl Default for AisType {
    fn default() -> Self {
        AisType::AIS_TYPE_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum AisNavStatus {
    UNDER_WAY = 0,
    AIS_NAV_ANCHORED = 1,
    AIS_NAV_UN_COMMANDED = 2,
    AIS_NAV_RESTRICTED_MANOEUVERABILITY = 3,
    AIS_NAV_DRAUGHT_CONSTRAINED = 4,
    AIS_NAV_MOORED = 5,
    AIS_NAV_AGROUND = 6,
    AIS_NAV_FISHING = 7,
    AIS_NAV_SAILING = 8,
    AIS_NAV_RESERVED_HSC = 9,
    AIS_NAV_RESERVED_WIG = 10,
    AIS_NAV_RESERVED_1 = 11,
    AIS_NAV_RESERVED_2 = 12,
    AIS_NAV_RESERVED_3 = 13,
    AIS_NAV_AIS_SART = 14,
    AIS_NAV_UNKNOWN = 15,
}
impl Default for AisNavStatus {
    fn default() -> Self {
        AisNavStatus::UNDER_WAY
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct AisFlags : u16 { const AIS_FLAGS_POSITION_ACCURACY = 1 ; const AIS_FLAGS_VALID_COG = 2 ; const AIS_FLAGS_VALID_VELOCITY = 4 ; const AIS_FLAGS_HIGH_VELOCITY = 8 ; const AIS_FLAGS_VALID_TURN_RATE = 16 ; const AIS_FLAGS_TURN_RATE_SIGN_ONLY = 32 ; const AIS_FLAGS_VALID_DIMENSIONS = 64 ; const AIS_FLAGS_LARGE_BOW_DIMENSION = 128 ; const AIS_FLAGS_LARGE_STERN_DIMENSION = 256 ; const AIS_FLAGS_LARGE_PORT_DIMENSION = 512 ; const AIS_FLAGS_LARGE_STARBOARD_DIMENSION = 1024 ; const AIS_FLAGS_VALID_CALLSIGN = 2048 ; const AIS_FLAGS_VALID_NAME = 4096 ; } }
impl Default for AisFlags {
    fn default() -> Self {
        AisFlags::AIS_FLAGS_POSITION_ACCURACY
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FailureUnit {
    FAILURE_UNIT_SENSOR_GYRO = 0,
    FAILURE_UNIT_SENSOR_ACCEL = 1,
    FAILURE_UNIT_SENSOR_MAG = 2,
    FAILURE_UNIT_SENSOR_BARO = 3,
    FAILURE_UNIT_SENSOR_GPS = 4,
    FAILURE_UNIT_SENSOR_OPTICAL_FLOW = 5,
    FAILURE_UNIT_SENSOR_VIO = 6,
    FAILURE_UNIT_SENSOR_DISTANCE_SENSOR = 7,
    FAILURE_UNIT_SENSOR_AIRSPEED = 8,
    FAILURE_UNIT_SYSTEM_BATTERY = 100,
    FAILURE_UNIT_SYSTEM_MOTOR = 101,
    FAILURE_UNIT_SYSTEM_SERVO = 102,
    FAILURE_UNIT_SYSTEM_AVOIDANCE = 103,
    FAILURE_UNIT_SYSTEM_RC_SIGNAL = 104,
    FAILURE_UNIT_SYSTEM_MAVLINK_SIGNAL = 105,
}
impl Default for FailureUnit {
    fn default() -> Self {
        FailureUnit::FAILURE_UNIT_SENSOR_GYRO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum FailureType {
    FAILURE_TYPE_OK = 0,
    FAILURE_TYPE_OFF = 1,
    FAILURE_TYPE_STUCK = 2,
    FAILURE_TYPE_GARBAGE = 3,
    FAILURE_TYPE_WRONG = 4,
    FAILURE_TYPE_SLOW = 5,
    FAILURE_TYPE_DELAYED = 6,
    FAILURE_TYPE_INTERMITTENT = 7,
}
impl Default for FailureType {
    fn default() -> Self {
        FailureType::FAILURE_TYPE_OK
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
pub struct SYS_STATUS_DATA {
    pub onboard_control_sensors_present: MavSysStatusSensor,
    pub onboard_control_sensors_enabled: MavSysStatusSensor,
    pub onboard_control_sensors_health: MavSysStatusSensor,
    pub load: u16,
    pub voltage_battery: u16,
    pub current_battery: i16,
    pub drop_rate_comm: u16,
    pub errors_comm: u16,
    pub errors_count1: u16,
    pub errors_count2: u16,
    pub errors_count3: u16,
    pub errors_count4: u16,
    pub battery_remaining: i8,
}
impl SYS_STATUS_DATA {
    pub const ENCODED_LEN: usize = 31usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SYS_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SYS_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.onboard_control_sensors_present = MavSysStatusSensor::from_bits(
            tmp & MavSysStatusSensor::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavSysStatusSensor".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u32_le();
        _struct.onboard_control_sensors_enabled = MavSysStatusSensor::from_bits(
            tmp & MavSysStatusSensor::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavSysStatusSensor".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u32_le();
        _struct.onboard_control_sensors_health = MavSysStatusSensor::from_bits(
            tmp & MavSysStatusSensor::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavSysStatusSensor".to_string(),
            value: tmp as u32,
        })?;
        _struct.load = buf.get_u16_le();
        _struct.voltage_battery = buf.get_u16_le();
        _struct.current_battery = buf.get_i16_le();
        _struct.drop_rate_comm = buf.get_u16_le();
        _struct.errors_comm = buf.get_u16_le();
        _struct.errors_count1 = buf.get_u16_le();
        _struct.errors_count2 = buf.get_u16_le();
        _struct.errors_count3 = buf.get_u16_le();
        _struct.errors_count4 = buf.get_u16_le();
        _struct.battery_remaining = buf.get_i8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.onboard_control_sensors_present.bits());
        _tmp.put_u32_le(self.onboard_control_sensors_enabled.bits());
        _tmp.put_u32_le(self.onboard_control_sensors_health.bits());
        _tmp.put_u16_le(self.load);
        _tmp.put_u16_le(self.voltage_battery);
        _tmp.put_i16_le(self.current_battery);
        _tmp.put_u16_le(self.drop_rate_comm);
        _tmp.put_u16_le(self.errors_comm);
        _tmp.put_u16_le(self.errors_count1);
        _tmp.put_u16_le(self.errors_count2);
        _tmp.put_u16_le(self.errors_count3);
        _tmp.put_u16_le(self.errors_count4);
        _tmp.put_i8(self.battery_remaining);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SYSTEM_TIME_DATA {
    pub time_unix_usec: u64,
    pub time_boot_ms: u32,
}
impl SYSTEM_TIME_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SYSTEM_TIME_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SYSTEM_TIME_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_unix_usec = buf.get_u64_le();
        _struct.time_boot_ms = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_unix_usec);
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PING_DATA {
    pub time_usec: u64,
    pub seq: u32,
    pub target_system: u8,
    pub target_component: u8,
}
impl PING_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PING_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PING_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.seq = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.seq);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CHANGE_OPERATOR_CONTROL_DATA {
    pub target_system: u8,
    pub control_request: u8,
    pub version: u8,
    pub passkey: [char; 25],
}
impl CHANGE_OPERATOR_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CHANGE_OPERATOR_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CHANGE_OPERATOR_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.control_request = buf.get_u8();
        _struct.version = buf.get_u8();
        for idx in 0..25usize {
            let val = buf.get_u8() as char;
            _struct.passkey[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.control_request);
        _tmp.put_u8(self.version);
        for val in &self.passkey {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CHANGE_OPERATOR_CONTROL_ACK_DATA {
    pub gcs_system_id: u8,
    pub control_request: u8,
    pub ack: u8,
}
impl CHANGE_OPERATOR_CONTROL_ACK_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CHANGE_OPERATOR_CONTROL_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CHANGE_OPERATOR_CONTROL_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.gcs_system_id = buf.get_u8();
        _struct.control_request = buf.get_u8();
        _struct.ack = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.gcs_system_id);
        _tmp.put_u8(self.control_request);
        _tmp.put_u8(self.ack);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AUTH_KEY_DATA {
    pub key: [char; 32],
}
impl AUTH_KEY_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AUTH_KEY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AUTH_KEY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.key[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.key {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LINK_NODE_STATUS_DATA {
    pub timestamp: u64,
    pub tx_rate: u32,
    pub rx_rate: u32,
    pub messages_sent: u32,
    pub messages_received: u32,
    pub messages_lost: u32,
    pub rx_parse_err: u16,
    pub tx_overflows: u16,
    pub rx_overflows: u16,
    pub tx_buf: u8,
    pub rx_buf: u8,
}
impl LINK_NODE_STATUS_DATA {
    pub const ENCODED_LEN: usize = 36usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LINK_NODE_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LINK_NODE_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.tx_rate = buf.get_u32_le();
        _struct.rx_rate = buf.get_u32_le();
        _struct.messages_sent = buf.get_u32_le();
        _struct.messages_received = buf.get_u32_le();
        _struct.messages_lost = buf.get_u32_le();
        _struct.rx_parse_err = buf.get_u16_le();
        _struct.tx_overflows = buf.get_u16_le();
        _struct.rx_overflows = buf.get_u16_le();
        _struct.tx_buf = buf.get_u8();
        _struct.rx_buf = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u32_le(self.tx_rate);
        _tmp.put_u32_le(self.rx_rate);
        _tmp.put_u32_le(self.messages_sent);
        _tmp.put_u32_le(self.messages_received);
        _tmp.put_u32_le(self.messages_lost);
        _tmp.put_u16_le(self.rx_parse_err);
        _tmp.put_u16_le(self.tx_overflows);
        _tmp.put_u16_le(self.rx_overflows);
        _tmp.put_u8(self.tx_buf);
        _tmp.put_u8(self.rx_buf);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_MODE_DATA {
    pub custom_mode: u32,
    pub target_system: u8,
    pub base_mode: MavMode,
}
impl SET_MODE_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_MODE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_MODE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.custom_mode = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.base_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavMode".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.base_mode as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_REQUEST_READ_DATA {
    pub param_index: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: [char; 16],
}
impl PARAM_REQUEST_READ_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_REQUEST_READ_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_REQUEST_READ_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.param_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl PARAM_REQUEST_LIST_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_REQUEST_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_REQUEST_LIST_DATA::ENCODED_LEN];
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
pub struct PARAM_VALUE_DATA {
    pub param_value: f32,
    pub param_count: u16,
    pub param_index: u16,
    pub param_id: [char; 16],
    pub param_type: MavParamType,
}
impl PARAM_VALUE_DATA {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_VALUE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_VALUE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_value = buf.get_f32_le();
        _struct.param_count = buf.get_u16_le();
        _struct.param_index = buf.get_u16_le();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavParamType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value);
        _tmp.put_u16_le(self.param_count);
        _tmp.put_u16_le(self.param_index);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_SET_DATA {
    pub param_value: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: [char; 16],
    pub param_type: MavParamType,
}
impl PARAM_SET_DATA {
    pub const ENCODED_LEN: usize = 23usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_SET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_SET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_value = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavParamType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_RAW_INT_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub cog: u16,
    pub fix_type: GpsFixType,
    pub satellites_visible: u8,
}
impl GPS_RAW_INT_DATA {
    pub const ENCODED_LEN: usize = 30usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_RAW_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_RAW_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.eph = buf.get_u16_le();
        _struct.epv = buf.get_u16_le();
        _struct.vel = buf.get_u16_le();
        _struct.cog = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GpsFixType".to_string(),
            value: tmp as u32,
        })?;
        _struct.satellites_visible = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_u16_le(self.eph);
        _tmp.put_u16_le(self.epv);
        _tmp.put_u16_le(self.vel);
        _tmp.put_u16_le(self.cog);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_STATUS_DATA {
    pub satellites_visible: u8,
    pub satellite_prn: [u8; 20],
    pub satellite_used: [u8; 20],
    pub satellite_elevation: [u8; 20],
    pub satellite_azimuth: [u8; 20],
    pub satellite_snr: [u8; 20],
}
impl GPS_STATUS_DATA {
    pub const ENCODED_LEN: usize = 101usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.satellites_visible = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.satellite_prn[idx] = val;
        }
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.satellite_used[idx] = val;
        }
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.satellite_elevation[idx] = val;
        }
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.satellite_azimuth[idx] = val;
        }
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.satellite_snr[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.satellites_visible);
        for val in &self.satellite_prn {
            _tmp.put_u8(*val);
        }
        for val in &self.satellite_used {
            _tmp.put_u8(*val);
        }
        for val in &self.satellite_elevation {
            _tmp.put_u8(*val);
        }
        for val in &self.satellite_azimuth {
            _tmp.put_u8(*val);
        }
        for val in &self.satellite_snr {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_IMU_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}
impl SCALED_IMU_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_IMU_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_IMU_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        _struct.xgyro = buf.get_i16_le();
        _struct.ygyro = buf.get_i16_le();
        _struct.zgyro = buf.get_i16_le();
        _struct.xmag = buf.get_i16_le();
        _struct.ymag = buf.get_i16_le();
        _struct.zmag = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp.put_i16_le(self.xgyro);
        _tmp.put_i16_le(self.ygyro);
        _tmp.put_i16_le(self.zgyro);
        _tmp.put_i16_le(self.xmag);
        _tmp.put_i16_le(self.ymag);
        _tmp.put_i16_le(self.zmag);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RAW_IMU_DATA {
    pub time_usec: u64,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}
impl RAW_IMU_DATA {
    pub const ENCODED_LEN: usize = 26usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RAW_IMU_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RAW_IMU_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        _struct.xgyro = buf.get_i16_le();
        _struct.ygyro = buf.get_i16_le();
        _struct.zgyro = buf.get_i16_le();
        _struct.xmag = buf.get_i16_le();
        _struct.ymag = buf.get_i16_le();
        _struct.zmag = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp.put_i16_le(self.xgyro);
        _tmp.put_i16_le(self.ygyro);
        _tmp.put_i16_le(self.zgyro);
        _tmp.put_i16_le(self.xmag);
        _tmp.put_i16_le(self.ymag);
        _tmp.put_i16_le(self.zmag);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RAW_PRESSURE_DATA {
    pub time_usec: u64,
    pub press_abs: i16,
    pub press_diff1: i16,
    pub press_diff2: i16,
    pub temperature: i16,
}
impl RAW_PRESSURE_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RAW_PRESSURE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RAW_PRESSURE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.press_abs = buf.get_i16_le();
        _struct.press_diff1 = buf.get_i16_le();
        _struct.press_diff2 = buf.get_i16_le();
        _struct.temperature = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i16_le(self.press_abs);
        _tmp.put_i16_le(self.press_diff1);
        _tmp.put_i16_le(self.press_diff2);
        _tmp.put_i16_le(self.temperature);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_PRESSURE_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}
impl SCALED_PRESSURE_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_PRESSURE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_PRESSURE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.press_abs = buf.get_f32_le();
        _struct.press_diff = buf.get_f32_le();
        _struct.temperature = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.press_abs);
        _tmp.put_f32_le(self.press_diff);
        _tmp.put_i16_le(self.temperature);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATTITUDE_DATA {
    pub time_boot_ms: u32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
}
impl ATTITUDE_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ATTITUDE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ATTITUDE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATTITUDE_QUATERNION_DATA {
    pub time_boot_ms: u32,
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
}
impl ATTITUDE_QUATERNION_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ATTITUDE_QUATERNION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ATTITUDE_QUATERNION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.q1 = buf.get_f32_le();
        _struct.q2 = buf.get_f32_le();
        _struct.q3 = buf.get_f32_le();
        _struct.q4 = buf.get_f32_le();
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.q1);
        _tmp.put_f32_le(self.q2);
        _tmp.put_f32_le(self.q3);
        _tmp.put_f32_le(self.q4);
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOCAL_POSITION_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}
impl LOCAL_POSITION_NED_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOCAL_POSITION_NED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOCAL_POSITION_NED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GLOBAL_POSITION_INT_DATA {
    pub time_boot_ms: u32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub hdg: u16,
}
impl GLOBAL_POSITION_INT_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GLOBAL_POSITION_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GLOBAL_POSITION_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.relative_alt = buf.get_i32_le();
        _struct.vx = buf.get_i16_le();
        _struct.vy = buf.get_i16_le();
        _struct.vz = buf.get_i16_le();
        _struct.hdg = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i32_le(self.relative_alt);
        _tmp.put_i16_le(self.vx);
        _tmp.put_i16_le(self.vy);
        _tmp.put_i16_le(self.vz);
        _tmp.put_u16_le(self.hdg);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RC_CHANNELS_SCALED_DATA {
    pub time_boot_ms: u32,
    pub chan1_scaled: i16,
    pub chan2_scaled: i16,
    pub chan3_scaled: i16,
    pub chan4_scaled: i16,
    pub chan5_scaled: i16,
    pub chan6_scaled: i16,
    pub chan7_scaled: i16,
    pub chan8_scaled: i16,
    pub port: u8,
    pub rssi: u8,
}
impl RC_CHANNELS_SCALED_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RC_CHANNELS_SCALED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RC_CHANNELS_SCALED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.chan1_scaled = buf.get_i16_le();
        _struct.chan2_scaled = buf.get_i16_le();
        _struct.chan3_scaled = buf.get_i16_le();
        _struct.chan4_scaled = buf.get_i16_le();
        _struct.chan5_scaled = buf.get_i16_le();
        _struct.chan6_scaled = buf.get_i16_le();
        _struct.chan7_scaled = buf.get_i16_le();
        _struct.chan8_scaled = buf.get_i16_le();
        _struct.port = buf.get_u8();
        _struct.rssi = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i16_le(self.chan1_scaled);
        _tmp.put_i16_le(self.chan2_scaled);
        _tmp.put_i16_le(self.chan3_scaled);
        _tmp.put_i16_le(self.chan4_scaled);
        _tmp.put_i16_le(self.chan5_scaled);
        _tmp.put_i16_le(self.chan6_scaled);
        _tmp.put_i16_le(self.chan7_scaled);
        _tmp.put_i16_le(self.chan8_scaled);
        _tmp.put_u8(self.port);
        _tmp.put_u8(self.rssi);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RC_CHANNELS_RAW_DATA {
    pub time_boot_ms: u32,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub port: u8,
    pub rssi: u8,
}
impl RC_CHANNELS_RAW_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RC_CHANNELS_RAW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RC_CHANNELS_RAW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.chan1_raw = buf.get_u16_le();
        _struct.chan2_raw = buf.get_u16_le();
        _struct.chan3_raw = buf.get_u16_le();
        _struct.chan4_raw = buf.get_u16_le();
        _struct.chan5_raw = buf.get_u16_le();
        _struct.chan6_raw = buf.get_u16_le();
        _struct.chan7_raw = buf.get_u16_le();
        _struct.chan8_raw = buf.get_u16_le();
        _struct.port = buf.get_u8();
        _struct.rssi = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u16_le(self.chan1_raw);
        _tmp.put_u16_le(self.chan2_raw);
        _tmp.put_u16_le(self.chan3_raw);
        _tmp.put_u16_le(self.chan4_raw);
        _tmp.put_u16_le(self.chan5_raw);
        _tmp.put_u16_le(self.chan6_raw);
        _tmp.put_u16_le(self.chan7_raw);
        _tmp.put_u16_le(self.chan8_raw);
        _tmp.put_u8(self.port);
        _tmp.put_u8(self.rssi);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERVO_OUTPUT_RAW_DATA {
    pub time_usec: u32,
    pub servo1_raw: u16,
    pub servo2_raw: u16,
    pub servo3_raw: u16,
    pub servo4_raw: u16,
    pub servo5_raw: u16,
    pub servo6_raw: u16,
    pub servo7_raw: u16,
    pub servo8_raw: u16,
    pub port: u8,
}
impl SERVO_OUTPUT_RAW_DATA {
    pub const ENCODED_LEN: usize = 21usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERVO_OUTPUT_RAW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERVO_OUTPUT_RAW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u32_le();
        _struct.servo1_raw = buf.get_u16_le();
        _struct.servo2_raw = buf.get_u16_le();
        _struct.servo3_raw = buf.get_u16_le();
        _struct.servo4_raw = buf.get_u16_le();
        _struct.servo5_raw = buf.get_u16_le();
        _struct.servo6_raw = buf.get_u16_le();
        _struct.servo7_raw = buf.get_u16_le();
        _struct.servo8_raw = buf.get_u16_le();
        _struct.port = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_usec);
        _tmp.put_u16_le(self.servo1_raw);
        _tmp.put_u16_le(self.servo2_raw);
        _tmp.put_u16_le(self.servo3_raw);
        _tmp.put_u16_le(self.servo4_raw);
        _tmp.put_u16_le(self.servo5_raw);
        _tmp.put_u16_le(self.servo6_raw);
        _tmp.put_u16_le(self.servo7_raw);
        _tmp.put_u16_le(self.servo8_raw);
        _tmp.put_u8(self.port);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_REQUEST_PARTIAL_LIST_DATA {
    pub start_index: i16,
    pub end_index: i16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_REQUEST_PARTIAL_LIST_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_REQUEST_PARTIAL_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_REQUEST_PARTIAL_LIST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.start_index = buf.get_i16_le();
        _struct.end_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index);
        _tmp.put_i16_le(self.end_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_WRITE_PARTIAL_LIST_DATA {
    pub start_index: i16,
    pub end_index: i16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_WRITE_PARTIAL_LIST_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_WRITE_PARTIAL_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_WRITE_PARTIAL_LIST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.start_index = buf.get_i16_le();
        _struct.end_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index);
        _tmp.put_i16_le(self.end_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_ITEM_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub seq: u16,
    pub command: MavCmd,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: MavFrame,
    pub current: u8,
    pub autocontinue: u8,
}
impl MISSION_ITEM_DATA {
    pub const ENCODED_LEN: usize = 37usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_ITEM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_ITEM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param1 = buf.get_f32_le();
        _struct.param2 = buf.get_f32_le();
        _struct.param3 = buf.get_f32_le();
        _struct.param4 = buf.get_f32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.seq = buf.get_u16_le();
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
        _tmp.put_f32_le(self.param1);
        _tmp.put_f32_le(self.param2);
        _tmp.put_f32_le(self.param3);
        _tmp.put_f32_le(self.param4);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_u16_le(self.seq);
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
pub struct MISSION_REQUEST_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_REQUEST_DATA::ENCODED_LEN];
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
pub struct MISSION_SET_CURRENT_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_SET_CURRENT_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_SET_CURRENT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_SET_CURRENT_DATA::ENCODED_LEN];
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
pub struct MISSION_CURRENT_DATA {
    pub seq: u16,
}
impl MISSION_CURRENT_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_CURRENT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_CURRENT_DATA::ENCODED_LEN];
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
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_REQUEST_LIST_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_REQUEST_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_REQUEST_LIST_DATA::ENCODED_LEN];
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
pub struct MISSION_COUNT_DATA {
    pub count: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_COUNT_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_COUNT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_COUNT_DATA::ENCODED_LEN];
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
pub struct MISSION_CLEAR_ALL_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_CLEAR_ALL_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_CLEAR_ALL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_CLEAR_ALL_DATA::ENCODED_LEN];
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
pub struct MISSION_ITEM_REACHED_DATA {
    pub seq: u16,
}
impl MISSION_ITEM_REACHED_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_ITEM_REACHED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_ITEM_REACHED_DATA::ENCODED_LEN];
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
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_ACK_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub mavtype: MavMissionResult,
}
impl MISSION_ACK_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavMissionResult".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.mavtype as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_GPS_GLOBAL_ORIGIN_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub target_system: u8,
}
impl SET_GPS_GLOBAL_ORIGIN_DATA {
    pub const ENCODED_LEN: usize = 13usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_GPS_GLOBAL_ORIGIN_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_GPS_GLOBAL_ORIGIN_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.altitude = buf.get_i32_le();
        _struct.target_system = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_i32_le(self.altitude);
        _tmp.put_u8(self.target_system);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_GLOBAL_ORIGIN_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
}
impl GPS_GLOBAL_ORIGIN_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_GLOBAL_ORIGIN_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_GLOBAL_ORIGIN_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.altitude = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_i32_le(self.altitude);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_MAP_RC_DATA {
    pub param_value0: f32,
    pub scale: f32,
    pub param_value_min: f32,
    pub param_value_max: f32,
    pub param_index: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: [char; 16],
    pub parameter_rc_channel_index: u8,
}
impl PARAM_MAP_RC_DATA {
    pub const ENCODED_LEN: usize = 37usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_MAP_RC_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_MAP_RC_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_value0 = buf.get_f32_le();
        _struct.scale = buf.get_f32_le();
        _struct.param_value_min = buf.get_f32_le();
        _struct.param_value_max = buf.get_f32_le();
        _struct.param_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        _struct.parameter_rc_channel_index = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.param_value0);
        _tmp.put_f32_le(self.scale);
        _tmp.put_f32_le(self.param_value_min);
        _tmp.put_f32_le(self.param_value_max);
        _tmp.put_i16_le(self.param_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.parameter_rc_channel_index);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_REQUEST_INT_DATA {
    pub seq: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl MISSION_REQUEST_INT_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_REQUEST_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_REQUEST_INT_DATA::ENCODED_LEN];
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
pub struct MISSION_CHANGED_DATA {
    pub start_index: i16,
    pub end_index: i16,
    pub origin_sysid: u8,
    pub origin_compid: MavComponent,
    pub mission_type: MavMissionType,
}
impl MISSION_CHANGED_DATA {
    pub const ENCODED_LEN: usize = 7usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_CHANGED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_CHANGED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.start_index = buf.get_i16_le();
        _struct.end_index = buf.get_i16_le();
        _struct.origin_sysid = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.origin_compid = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavComponent".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.mission_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavMissionType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.start_index);
        _tmp.put_i16_le(self.end_index);
        _tmp.put_u8(self.origin_sysid);
        _tmp.put_u8(self.origin_compid as u8);
        _tmp.put_u8(self.mission_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SAFETY_SET_ALLOWED_AREA_DATA {
    pub p1x: f32,
    pub p1y: f32,
    pub p1z: f32,
    pub p2x: f32,
    pub p2y: f32,
    pub p2z: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: MavFrame,
}
impl SAFETY_SET_ALLOWED_AREA_DATA {
    pub const ENCODED_LEN: usize = 27usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SAFETY_SET_ALLOWED_AREA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SAFETY_SET_ALLOWED_AREA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.p1x = buf.get_f32_le();
        _struct.p1y = buf.get_f32_le();
        _struct.p1z = buf.get_f32_le();
        _struct.p2x = buf.get_f32_le();
        _struct.p2y = buf.get_f32_le();
        _struct.p2z = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.p1x);
        _tmp.put_f32_le(self.p1y);
        _tmp.put_f32_le(self.p1z);
        _tmp.put_f32_le(self.p2x);
        _tmp.put_f32_le(self.p2y);
        _tmp.put_f32_le(self.p2z);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SAFETY_ALLOWED_AREA_DATA {
    pub p1x: f32,
    pub p1y: f32,
    pub p1z: f32,
    pub p2x: f32,
    pub p2y: f32,
    pub p2z: f32,
    pub frame: MavFrame,
}
impl SAFETY_ALLOWED_AREA_DATA {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SAFETY_ALLOWED_AREA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SAFETY_ALLOWED_AREA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.p1x = buf.get_f32_le();
        _struct.p1y = buf.get_f32_le();
        _struct.p1z = buf.get_f32_le();
        _struct.p2x = buf.get_f32_le();
        _struct.p2y = buf.get_f32_le();
        _struct.p2z = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.p1x);
        _tmp.put_f32_le(self.p1y);
        _tmp.put_f32_le(self.p1z);
        _tmp.put_f32_le(self.p2x);
        _tmp.put_f32_le(self.p2y);
        _tmp.put_f32_le(self.p2z);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATTITUDE_QUATERNION_COV_DATA {
    pub time_usec: u64,
    pub q: [f32; 4],
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub covariance: [f32; 9],
}
impl ATTITUDE_QUATERNION_COV_DATA {
    pub const ENCODED_LEN: usize = 72usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ATTITUDE_QUATERNION_COV_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ATTITUDE_QUATERNION_COV_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        for idx in 0..9usize {
            let val = buf.get_f32_le();
            _struct.covariance[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        for val in &self.covariance {
            _tmp.put_f32_le(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NAV_CONTROLLER_OUTPUT_DATA {
    pub nav_roll: f32,
    pub nav_pitch: f32,
    pub alt_error: f32,
    pub aspd_error: f32,
    pub xtrack_error: f32,
    pub nav_bearing: i16,
    pub target_bearing: i16,
    pub wp_dist: u16,
}
impl NAV_CONTROLLER_OUTPUT_DATA {
    pub const ENCODED_LEN: usize = 26usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < NAV_CONTROLLER_OUTPUT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; NAV_CONTROLLER_OUTPUT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.nav_roll = buf.get_f32_le();
        _struct.nav_pitch = buf.get_f32_le();
        _struct.alt_error = buf.get_f32_le();
        _struct.aspd_error = buf.get_f32_le();
        _struct.xtrack_error = buf.get_f32_le();
        _struct.nav_bearing = buf.get_i16_le();
        _struct.target_bearing = buf.get_i16_le();
        _struct.wp_dist = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.nav_roll);
        _tmp.put_f32_le(self.nav_pitch);
        _tmp.put_f32_le(self.alt_error);
        _tmp.put_f32_le(self.aspd_error);
        _tmp.put_f32_le(self.xtrack_error);
        _tmp.put_i16_le(self.nav_bearing);
        _tmp.put_i16_le(self.target_bearing);
        _tmp.put_u16_le(self.wp_dist);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GLOBAL_POSITION_INT_COV_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub covariance: Vec<f32>, /* 36 elements */
    pub estimator_type: MavEstimatorType,
}
impl GLOBAL_POSITION_INT_COV_DATA {
    pub const ENCODED_LEN: usize = 181usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GLOBAL_POSITION_INT_COV_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GLOBAL_POSITION_INT_COV_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.relative_alt = buf.get_i32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        for _ in 0..36usize {
            let val = buf.get_f32_le();
            _struct.covariance.push(val);
        }
        let tmp = buf.get_u8();
        _struct.estimator_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavEstimatorType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i32_le(self.relative_alt);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        for val in &self.covariance {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.estimator_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOCAL_POSITION_NED_COV_DATA {
    pub time_usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub ax: f32,
    pub ay: f32,
    pub az: f32,
    pub covariance: Vec<f32>, /* 45 elements */
    pub estimator_type: MavEstimatorType,
}
impl LOCAL_POSITION_NED_COV_DATA {
    pub const ENCODED_LEN: usize = 225usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOCAL_POSITION_NED_COV_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOCAL_POSITION_NED_COV_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.ax = buf.get_f32_le();
        _struct.ay = buf.get_f32_le();
        _struct.az = buf.get_f32_le();
        for _ in 0..45usize {
            let val = buf.get_f32_le();
            _struct.covariance.push(val);
        }
        let tmp = buf.get_u8();
        _struct.estimator_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavEstimatorType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.ax);
        _tmp.put_f32_le(self.ay);
        _tmp.put_f32_le(self.az);
        for val in &self.covariance {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.estimator_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RC_CHANNELS_DATA {
    pub time_boot_ms: u32,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub chan9_raw: u16,
    pub chan10_raw: u16,
    pub chan11_raw: u16,
    pub chan12_raw: u16,
    pub chan13_raw: u16,
    pub chan14_raw: u16,
    pub chan15_raw: u16,
    pub chan16_raw: u16,
    pub chan17_raw: u16,
    pub chan18_raw: u16,
    pub chancount: u8,
    pub rssi: u8,
}
impl RC_CHANNELS_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RC_CHANNELS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RC_CHANNELS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.chan1_raw = buf.get_u16_le();
        _struct.chan2_raw = buf.get_u16_le();
        _struct.chan3_raw = buf.get_u16_le();
        _struct.chan4_raw = buf.get_u16_le();
        _struct.chan5_raw = buf.get_u16_le();
        _struct.chan6_raw = buf.get_u16_le();
        _struct.chan7_raw = buf.get_u16_le();
        _struct.chan8_raw = buf.get_u16_le();
        _struct.chan9_raw = buf.get_u16_le();
        _struct.chan10_raw = buf.get_u16_le();
        _struct.chan11_raw = buf.get_u16_le();
        _struct.chan12_raw = buf.get_u16_le();
        _struct.chan13_raw = buf.get_u16_le();
        _struct.chan14_raw = buf.get_u16_le();
        _struct.chan15_raw = buf.get_u16_le();
        _struct.chan16_raw = buf.get_u16_le();
        _struct.chan17_raw = buf.get_u16_le();
        _struct.chan18_raw = buf.get_u16_le();
        _struct.chancount = buf.get_u8();
        _struct.rssi = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u16_le(self.chan1_raw);
        _tmp.put_u16_le(self.chan2_raw);
        _tmp.put_u16_le(self.chan3_raw);
        _tmp.put_u16_le(self.chan4_raw);
        _tmp.put_u16_le(self.chan5_raw);
        _tmp.put_u16_le(self.chan6_raw);
        _tmp.put_u16_le(self.chan7_raw);
        _tmp.put_u16_le(self.chan8_raw);
        _tmp.put_u16_le(self.chan9_raw);
        _tmp.put_u16_le(self.chan10_raw);
        _tmp.put_u16_le(self.chan11_raw);
        _tmp.put_u16_le(self.chan12_raw);
        _tmp.put_u16_le(self.chan13_raw);
        _tmp.put_u16_le(self.chan14_raw);
        _tmp.put_u16_le(self.chan15_raw);
        _tmp.put_u16_le(self.chan16_raw);
        _tmp.put_u16_le(self.chan17_raw);
        _tmp.put_u16_le(self.chan18_raw);
        _tmp.put_u8(self.chancount);
        _tmp.put_u8(self.rssi);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct REQUEST_DATA_STREAM_DATA {
    pub req_message_rate: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub req_stream_id: u8,
    pub start_stop: u8,
}
impl REQUEST_DATA_STREAM_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < REQUEST_DATA_STREAM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; REQUEST_DATA_STREAM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.req_message_rate = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.req_stream_id = buf.get_u8();
        _struct.start_stop = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.req_message_rate);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.req_stream_id);
        _tmp.put_u8(self.start_stop);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA_STREAM_DATA {
    pub message_rate: u16,
    pub stream_id: u8,
    pub on_off: u8,
}
impl DATA_STREAM_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA_STREAM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA_STREAM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.message_rate = buf.get_u16_le();
        _struct.stream_id = buf.get_u8();
        _struct.on_off = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.message_rate);
        _tmp.put_u8(self.stream_id);
        _tmp.put_u8(self.on_off);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MANUAL_CONTROL_DATA {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub r: i16,
    pub buttons: u16,
    pub target: u8,
}
impl MANUAL_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 11usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MANUAL_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MANUAL_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.x = buf.get_i16_le();
        _struct.y = buf.get_i16_le();
        _struct.z = buf.get_i16_le();
        _struct.r = buf.get_i16_le();
        _struct.buttons = buf.get_u16_le();
        _struct.target = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.x);
        _tmp.put_i16_le(self.y);
        _tmp.put_i16_le(self.z);
        _tmp.put_i16_le(self.r);
        _tmp.put_u16_le(self.buttons);
        _tmp.put_u8(self.target);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RC_CHANNELS_OVERRIDE_DATA {
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl RC_CHANNELS_OVERRIDE_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RC_CHANNELS_OVERRIDE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RC_CHANNELS_OVERRIDE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.chan1_raw = buf.get_u16_le();
        _struct.chan2_raw = buf.get_u16_le();
        _struct.chan3_raw = buf.get_u16_le();
        _struct.chan4_raw = buf.get_u16_le();
        _struct.chan5_raw = buf.get_u16_le();
        _struct.chan6_raw = buf.get_u16_le();
        _struct.chan7_raw = buf.get_u16_le();
        _struct.chan8_raw = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.chan1_raw);
        _tmp.put_u16_le(self.chan2_raw);
        _tmp.put_u16_le(self.chan3_raw);
        _tmp.put_u16_le(self.chan4_raw);
        _tmp.put_u16_le(self.chan5_raw);
        _tmp.put_u16_le(self.chan6_raw);
        _tmp.put_u16_le(self.chan7_raw);
        _tmp.put_u16_le(self.chan8_raw);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MISSION_ITEM_INT_DATA {
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub seq: u16,
    pub command: MavCmd,
    pub target_system: u8,
    pub target_component: u8,
    pub frame: MavFrame,
    pub current: u8,
    pub autocontinue: u8,
}
impl MISSION_ITEM_INT_DATA {
    pub const ENCODED_LEN: usize = 37usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MISSION_ITEM_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MISSION_ITEM_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param1 = buf.get_f32_le();
        _struct.param2 = buf.get_f32_le();
        _struct.param3 = buf.get_f32_le();
        _struct.param4 = buf.get_f32_le();
        _struct.x = buf.get_i32_le();
        _struct.y = buf.get_i32_le();
        _struct.z = buf.get_f32_le();
        _struct.seq = buf.get_u16_le();
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
        _tmp.put_f32_le(self.param1);
        _tmp.put_f32_le(self.param2);
        _tmp.put_f32_le(self.param3);
        _tmp.put_f32_le(self.param4);
        _tmp.put_i32_le(self.x);
        _tmp.put_i32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_u16_le(self.seq);
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
pub struct VFR_HUD_DATA {
    pub airspeed: f32,
    pub groundspeed: f32,
    pub alt: f32,
    pub climb: f32,
    pub heading: i16,
    pub throttle: u16,
}
impl VFR_HUD_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VFR_HUD_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VFR_HUD_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.airspeed = buf.get_f32_le();
        _struct.groundspeed = buf.get_f32_le();
        _struct.alt = buf.get_f32_le();
        _struct.climb = buf.get_f32_le();
        _struct.heading = buf.get_i16_le();
        _struct.throttle = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.airspeed);
        _tmp.put_f32_le(self.groundspeed);
        _tmp.put_f32_le(self.alt);
        _tmp.put_f32_le(self.climb);
        _tmp.put_i16_le(self.heading);
        _tmp.put_u16_le(self.throttle);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMMAND_INT_DATA {
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
impl COMMAND_INT_DATA {
    pub const ENCODED_LEN: usize = 35usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
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
pub struct COMMAND_LONG_DATA {
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
impl COMMAND_LONG_DATA {
    pub const ENCODED_LEN: usize = 33usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_LONG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_LONG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
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
pub struct COMMAND_ACK_DATA {
    pub command: MavCmd,
    pub result: MavResult,
}
impl COMMAND_ACK_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u16_le();
        _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCmd".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.result = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavResult".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.result as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMMAND_CANCEL_DATA {
    pub command: MavCmd,
    pub target_system: u8,
    pub target_component: u8,
}
impl COMMAND_CANCEL_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMMAND_CANCEL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMMAND_CANCEL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u16_le();
        _struct.command = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCmd".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.command as u16);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MANUAL_SETPOINT_DATA {
    pub time_boot_ms: u32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub thrust: f32,
    pub mode_switch: u8,
    pub manual_override_switch: u8,
}
impl MANUAL_SETPOINT_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MANUAL_SETPOINT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MANUAL_SETPOINT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.thrust = buf.get_f32_le();
        _struct.mode_switch = buf.get_u8();
        _struct.manual_override_switch = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.thrust);
        _tmp.put_u8(self.mode_switch);
        _tmp.put_u8(self.manual_override_switch);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_ATTITUDE_TARGET_DATA {
    pub time_boot_ms: u32,
    pub q: [f32; 4],
    pub body_roll_rate: f32,
    pub body_pitch_rate: f32,
    pub body_yaw_rate: f32,
    pub thrust: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub type_mask: u8,
}
impl SET_ATTITUDE_TARGET_DATA {
    pub const ENCODED_LEN: usize = 39usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_ATTITUDE_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_ATTITUDE_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.body_roll_rate = buf.get_f32_le();
        _struct.body_pitch_rate = buf.get_f32_le();
        _struct.body_yaw_rate = buf.get_f32_le();
        _struct.thrust = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.type_mask = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.body_roll_rate);
        _tmp.put_f32_le(self.body_pitch_rate);
        _tmp.put_f32_le(self.body_yaw_rate);
        _tmp.put_f32_le(self.thrust);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.type_mask);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATTITUDE_TARGET_DATA {
    pub time_boot_ms: u32,
    pub q: [f32; 4],
    pub body_roll_rate: f32,
    pub body_pitch_rate: f32,
    pub body_yaw_rate: f32,
    pub thrust: f32,
    pub type_mask: u8,
}
impl ATTITUDE_TARGET_DATA {
    pub const ENCODED_LEN: usize = 37usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ATTITUDE_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ATTITUDE_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.body_roll_rate = buf.get_f32_le();
        _struct.body_pitch_rate = buf.get_f32_le();
        _struct.body_yaw_rate = buf.get_f32_le();
        _struct.thrust = buf.get_f32_le();
        _struct.type_mask = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.body_roll_rate);
        _tmp.put_f32_le(self.body_pitch_rate);
        _tmp.put_f32_le(self.body_yaw_rate);
        _tmp.put_f32_le(self.thrust);
        _tmp.put_u8(self.type_mask);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_POSITION_TARGET_LOCAL_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: PositionTargetTypemask,
    pub target_system: u8,
    pub target_component: u8,
    pub coordinate_frame: MavFrame,
}
impl SET_POSITION_TARGET_LOCAL_NED_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_POSITION_TARGET_LOCAL_NED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_POSITION_TARGET_LOCAL_NED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.afx = buf.get_f32_le();
        _struct.afy = buf.get_f32_le();
        _struct.afz = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.yaw_rate = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.type_mask = PositionTargetTypemask::from_bits(
            tmp & PositionTargetTypemask::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "PositionTargetTypemask".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.coordinate_frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.afx);
        _tmp.put_f32_le(self.afy);
        _tmp.put_f32_le(self.afz);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.yaw_rate);
        _tmp.put_u16_le(self.type_mask.bits());
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct POSITION_TARGET_LOCAL_NED_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: PositionTargetTypemask,
    pub coordinate_frame: MavFrame,
}
impl POSITION_TARGET_LOCAL_NED_DATA {
    pub const ENCODED_LEN: usize = 51usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < POSITION_TARGET_LOCAL_NED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; POSITION_TARGET_LOCAL_NED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.afx = buf.get_f32_le();
        _struct.afy = buf.get_f32_le();
        _struct.afz = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.yaw_rate = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.type_mask = PositionTargetTypemask::from_bits(
            tmp & PositionTargetTypemask::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "PositionTargetTypemask".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.coordinate_frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.afx);
        _tmp.put_f32_le(self.afy);
        _tmp.put_f32_le(self.afz);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.yaw_rate);
        _tmp.put_u16_le(self.type_mask.bits());
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_POSITION_TARGET_GLOBAL_INT_DATA {
    pub time_boot_ms: u32,
    pub lat_int: i32,
    pub lon_int: i32,
    pub alt: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: PositionTargetTypemask,
    pub target_system: u8,
    pub target_component: u8,
    pub coordinate_frame: MavFrame,
}
impl SET_POSITION_TARGET_GLOBAL_INT_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_POSITION_TARGET_GLOBAL_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_POSITION_TARGET_GLOBAL_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.lat_int = buf.get_i32_le();
        _struct.lon_int = buf.get_i32_le();
        _struct.alt = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.afx = buf.get_f32_le();
        _struct.afy = buf.get_f32_le();
        _struct.afz = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.yaw_rate = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.type_mask = PositionTargetTypemask::from_bits(
            tmp & PositionTargetTypemask::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "PositionTargetTypemask".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.coordinate_frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.lat_int);
        _tmp.put_i32_le(self.lon_int);
        _tmp.put_f32_le(self.alt);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.afx);
        _tmp.put_f32_le(self.afy);
        _tmp.put_f32_le(self.afz);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.yaw_rate);
        _tmp.put_u16_le(self.type_mask.bits());
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct POSITION_TARGET_GLOBAL_INT_DATA {
    pub time_boot_ms: u32,
    pub lat_int: i32,
    pub lon_int: i32,
    pub alt: f32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub afx: f32,
    pub afy: f32,
    pub afz: f32,
    pub yaw: f32,
    pub yaw_rate: f32,
    pub type_mask: PositionTargetTypemask,
    pub coordinate_frame: MavFrame,
}
impl POSITION_TARGET_GLOBAL_INT_DATA {
    pub const ENCODED_LEN: usize = 51usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < POSITION_TARGET_GLOBAL_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; POSITION_TARGET_GLOBAL_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.lat_int = buf.get_i32_le();
        _struct.lon_int = buf.get_i32_le();
        _struct.alt = buf.get_f32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.afx = buf.get_f32_le();
        _struct.afy = buf.get_f32_le();
        _struct.afz = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.yaw_rate = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.type_mask = PositionTargetTypemask::from_bits(
            tmp & PositionTargetTypemask::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "PositionTargetTypemask".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.coordinate_frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.lat_int);
        _tmp.put_i32_le(self.lon_int);
        _tmp.put_f32_le(self.alt);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.afx);
        _tmp.put_f32_le(self.afy);
        _tmp.put_f32_le(self.afz);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.yaw_rate);
        _tmp.put_u16_le(self.type_mask.bits());
        _tmp.put_u8(self.coordinate_frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
    pub time_boot_ms: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_STATE_DATA {
    pub time_usec: u64,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
}
impl HIL_STATE_DATA {
    pub const ENCODED_LEN: usize = 56usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_STATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_STATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.vx = buf.get_i16_le();
        _struct.vy = buf.get_i16_le();
        _struct.vz = buf.get_i16_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i16_le(self.vx);
        _tmp.put_i16_le(self.vy);
        _tmp.put_i16_le(self.vz);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_CONTROLS_DATA {
    pub time_usec: u64,
    pub roll_ailerons: f32,
    pub pitch_elevator: f32,
    pub yaw_rudder: f32,
    pub throttle: f32,
    pub aux1: f32,
    pub aux2: f32,
    pub aux3: f32,
    pub aux4: f32,
    pub mode: MavMode,
    pub nav_mode: u8,
}
impl HIL_CONTROLS_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_CONTROLS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_CONTROLS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.roll_ailerons = buf.get_f32_le();
        _struct.pitch_elevator = buf.get_f32_le();
        _struct.yaw_rudder = buf.get_f32_le();
        _struct.throttle = buf.get_f32_le();
        _struct.aux1 = buf.get_f32_le();
        _struct.aux2 = buf.get_f32_le();
        _struct.aux3 = buf.get_f32_le();
        _struct.aux4 = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavMode".to_string(),
            value: tmp as u32,
        })?;
        _struct.nav_mode = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.roll_ailerons);
        _tmp.put_f32_le(self.pitch_elevator);
        _tmp.put_f32_le(self.yaw_rudder);
        _tmp.put_f32_le(self.throttle);
        _tmp.put_f32_le(self.aux1);
        _tmp.put_f32_le(self.aux2);
        _tmp.put_f32_le(self.aux3);
        _tmp.put_f32_le(self.aux4);
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.nav_mode);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_RC_INPUTS_RAW_DATA {
    pub time_usec: u64,
    pub chan1_raw: u16,
    pub chan2_raw: u16,
    pub chan3_raw: u16,
    pub chan4_raw: u16,
    pub chan5_raw: u16,
    pub chan6_raw: u16,
    pub chan7_raw: u16,
    pub chan8_raw: u16,
    pub chan9_raw: u16,
    pub chan10_raw: u16,
    pub chan11_raw: u16,
    pub chan12_raw: u16,
    pub rssi: u8,
}
impl HIL_RC_INPUTS_RAW_DATA {
    pub const ENCODED_LEN: usize = 33usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_RC_INPUTS_RAW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_RC_INPUTS_RAW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.chan1_raw = buf.get_u16_le();
        _struct.chan2_raw = buf.get_u16_le();
        _struct.chan3_raw = buf.get_u16_le();
        _struct.chan4_raw = buf.get_u16_le();
        _struct.chan5_raw = buf.get_u16_le();
        _struct.chan6_raw = buf.get_u16_le();
        _struct.chan7_raw = buf.get_u16_le();
        _struct.chan8_raw = buf.get_u16_le();
        _struct.chan9_raw = buf.get_u16_le();
        _struct.chan10_raw = buf.get_u16_le();
        _struct.chan11_raw = buf.get_u16_le();
        _struct.chan12_raw = buf.get_u16_le();
        _struct.rssi = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u16_le(self.chan1_raw);
        _tmp.put_u16_le(self.chan2_raw);
        _tmp.put_u16_le(self.chan3_raw);
        _tmp.put_u16_le(self.chan4_raw);
        _tmp.put_u16_le(self.chan5_raw);
        _tmp.put_u16_le(self.chan6_raw);
        _tmp.put_u16_le(self.chan7_raw);
        _tmp.put_u16_le(self.chan8_raw);
        _tmp.put_u16_le(self.chan9_raw);
        _tmp.put_u16_le(self.chan10_raw);
        _tmp.put_u16_le(self.chan11_raw);
        _tmp.put_u16_le(self.chan12_raw);
        _tmp.put_u8(self.rssi);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_ACTUATOR_CONTROLS_DATA {
    pub time_usec: u64,
    pub flags: u64,
    pub controls: [f32; 16],
    pub mode: MavModeFlag,
}
impl HIL_ACTUATOR_CONTROLS_DATA {
    pub const ENCODED_LEN: usize = 81usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_ACTUATOR_CONTROLS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_ACTUATOR_CONTROLS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.flags = buf.get_u64_le();
        for idx in 0..16usize {
            let val = buf.get_f32_le();
            _struct.controls[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.mode = MavModeFlag::from_bits(tmp & MavModeFlag::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u64_le(self.flags);
        for val in &self.controls {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.mode.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPTICAL_FLOW_DATA {
    pub time_usec: u64,
    pub flow_comp_m_x: f32,
    pub flow_comp_m_y: f32,
    pub ground_distance: f32,
    pub flow_x: i16,
    pub flow_y: i16,
    pub sensor_id: u8,
    pub quality: u8,
}
impl OPTICAL_FLOW_DATA {
    pub const ENCODED_LEN: usize = 26usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPTICAL_FLOW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPTICAL_FLOW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.flow_comp_m_x = buf.get_f32_le();
        _struct.flow_comp_m_y = buf.get_f32_le();
        _struct.ground_distance = buf.get_f32_le();
        _struct.flow_x = buf.get_i16_le();
        _struct.flow_y = buf.get_i16_le();
        _struct.sensor_id = buf.get_u8();
        _struct.quality = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.flow_comp_m_x);
        _tmp.put_f32_le(self.flow_comp_m_y);
        _tmp.put_f32_le(self.ground_distance);
        _tmp.put_i16_le(self.flow_x);
        _tmp.put_i16_le(self.flow_y);
        _tmp.put_u8(self.sensor_id);
        _tmp.put_u8(self.quality);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GLOBAL_VISION_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl GLOBAL_VISION_POSITION_ESTIMATE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GLOBAL_VISION_POSITION_ESTIMATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GLOBAL_VISION_POSITION_ESTIMATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VISION_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl VISION_POSITION_ESTIMATE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VISION_POSITION_ESTIMATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VISION_POSITION_ESTIMATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VISION_SPEED_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl VISION_SPEED_ESTIMATE_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VISION_SPEED_ESTIMATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VISION_SPEED_ESTIMATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VICON_POSITION_ESTIMATE_DATA {
    pub usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl VICON_POSITION_ESTIMATE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VICON_POSITION_ESTIMATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VICON_POSITION_ESTIMATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIGHRES_IMU_DATA {
    pub time_usec: u64,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub xmag: f32,
    pub ymag: f32,
    pub zmag: f32,
    pub abs_pressure: f32,
    pub diff_pressure: f32,
    pub pressure_alt: f32,
    pub temperature: f32,
    pub fields_updated: u16,
}
impl HIGHRES_IMU_DATA {
    pub const ENCODED_LEN: usize = 62usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIGHRES_IMU_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIGHRES_IMU_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.xacc = buf.get_f32_le();
        _struct.yacc = buf.get_f32_le();
        _struct.zacc = buf.get_f32_le();
        _struct.xgyro = buf.get_f32_le();
        _struct.ygyro = buf.get_f32_le();
        _struct.zgyro = buf.get_f32_le();
        _struct.xmag = buf.get_f32_le();
        _struct.ymag = buf.get_f32_le();
        _struct.zmag = buf.get_f32_le();
        _struct.abs_pressure = buf.get_f32_le();
        _struct.diff_pressure = buf.get_f32_le();
        _struct.pressure_alt = buf.get_f32_le();
        _struct.temperature = buf.get_f32_le();
        _struct.fields_updated = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.xacc);
        _tmp.put_f32_le(self.yacc);
        _tmp.put_f32_le(self.zacc);
        _tmp.put_f32_le(self.xgyro);
        _tmp.put_f32_le(self.ygyro);
        _tmp.put_f32_le(self.zgyro);
        _tmp.put_f32_le(self.xmag);
        _tmp.put_f32_le(self.ymag);
        _tmp.put_f32_le(self.zmag);
        _tmp.put_f32_le(self.abs_pressure);
        _tmp.put_f32_le(self.diff_pressure);
        _tmp.put_f32_le(self.pressure_alt);
        _tmp.put_f32_le(self.temperature);
        _tmp.put_u16_le(self.fields_updated);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPTICAL_FLOW_RAD_DATA {
    pub time_usec: u64,
    pub integration_time_us: u32,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub integrated_xgyro: f32,
    pub integrated_ygyro: f32,
    pub integrated_zgyro: f32,
    pub time_delta_distance_us: u32,
    pub distance: f32,
    pub temperature: i16,
    pub sensor_id: u8,
    pub quality: u8,
}
impl OPTICAL_FLOW_RAD_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPTICAL_FLOW_RAD_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPTICAL_FLOW_RAD_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.integration_time_us = buf.get_u32_le();
        _struct.integrated_x = buf.get_f32_le();
        _struct.integrated_y = buf.get_f32_le();
        _struct.integrated_xgyro = buf.get_f32_le();
        _struct.integrated_ygyro = buf.get_f32_le();
        _struct.integrated_zgyro = buf.get_f32_le();
        _struct.time_delta_distance_us = buf.get_u32_le();
        _struct.distance = buf.get_f32_le();
        _struct.temperature = buf.get_i16_le();
        _struct.sensor_id = buf.get_u8();
        _struct.quality = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.integration_time_us);
        _tmp.put_f32_le(self.integrated_x);
        _tmp.put_f32_le(self.integrated_y);
        _tmp.put_f32_le(self.integrated_xgyro);
        _tmp.put_f32_le(self.integrated_ygyro);
        _tmp.put_f32_le(self.integrated_zgyro);
        _tmp.put_u32_le(self.time_delta_distance_us);
        _tmp.put_f32_le(self.distance);
        _tmp.put_i16_le(self.temperature);
        _tmp.put_u8(self.sensor_id);
        _tmp.put_u8(self.quality);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_SENSOR_DATA {
    pub time_usec: u64,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub xmag: f32,
    pub ymag: f32,
    pub zmag: f32,
    pub abs_pressure: f32,
    pub diff_pressure: f32,
    pub pressure_alt: f32,
    pub temperature: f32,
    pub fields_updated: u32,
}
impl HIL_SENSOR_DATA {
    pub const ENCODED_LEN: usize = 64usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_SENSOR_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_SENSOR_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.xacc = buf.get_f32_le();
        _struct.yacc = buf.get_f32_le();
        _struct.zacc = buf.get_f32_le();
        _struct.xgyro = buf.get_f32_le();
        _struct.ygyro = buf.get_f32_le();
        _struct.zgyro = buf.get_f32_le();
        _struct.xmag = buf.get_f32_le();
        _struct.ymag = buf.get_f32_le();
        _struct.zmag = buf.get_f32_le();
        _struct.abs_pressure = buf.get_f32_le();
        _struct.diff_pressure = buf.get_f32_le();
        _struct.pressure_alt = buf.get_f32_le();
        _struct.temperature = buf.get_f32_le();
        _struct.fields_updated = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.xacc);
        _tmp.put_f32_le(self.yacc);
        _tmp.put_f32_le(self.zacc);
        _tmp.put_f32_le(self.xgyro);
        _tmp.put_f32_le(self.ygyro);
        _tmp.put_f32_le(self.zgyro);
        _tmp.put_f32_le(self.xmag);
        _tmp.put_f32_le(self.ymag);
        _tmp.put_f32_le(self.zmag);
        _tmp.put_f32_le(self.abs_pressure);
        _tmp.put_f32_le(self.diff_pressure);
        _tmp.put_f32_le(self.pressure_alt);
        _tmp.put_f32_le(self.temperature);
        _tmp.put_u32_le(self.fields_updated);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SIM_STATE_DATA {
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub q4: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub lat: f32,
    pub lon: f32,
    pub alt: f32,
    pub std_dev_horz: f32,
    pub std_dev_vert: f32,
    pub vn: f32,
    pub ve: f32,
    pub vd: f32,
}
impl SIM_STATE_DATA {
    pub const ENCODED_LEN: usize = 84usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SIM_STATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SIM_STATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.q1 = buf.get_f32_le();
        _struct.q2 = buf.get_f32_le();
        _struct.q3 = buf.get_f32_le();
        _struct.q4 = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.xacc = buf.get_f32_le();
        _struct.yacc = buf.get_f32_le();
        _struct.zacc = buf.get_f32_le();
        _struct.xgyro = buf.get_f32_le();
        _struct.ygyro = buf.get_f32_le();
        _struct.zgyro = buf.get_f32_le();
        _struct.lat = buf.get_f32_le();
        _struct.lon = buf.get_f32_le();
        _struct.alt = buf.get_f32_le();
        _struct.std_dev_horz = buf.get_f32_le();
        _struct.std_dev_vert = buf.get_f32_le();
        _struct.vn = buf.get_f32_le();
        _struct.ve = buf.get_f32_le();
        _struct.vd = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.q1);
        _tmp.put_f32_le(self.q2);
        _tmp.put_f32_le(self.q3);
        _tmp.put_f32_le(self.q4);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.xacc);
        _tmp.put_f32_le(self.yacc);
        _tmp.put_f32_le(self.zacc);
        _tmp.put_f32_le(self.xgyro);
        _tmp.put_f32_le(self.ygyro);
        _tmp.put_f32_le(self.zgyro);
        _tmp.put_f32_le(self.lat);
        _tmp.put_f32_le(self.lon);
        _tmp.put_f32_le(self.alt);
        _tmp.put_f32_le(self.std_dev_horz);
        _tmp.put_f32_le(self.std_dev_vert);
        _tmp.put_f32_le(self.vn);
        _tmp.put_f32_le(self.ve);
        _tmp.put_f32_le(self.vd);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RADIO_STATUS_DATA {
    pub rxerrors: u16,
    pub fixed: u16,
    pub rssi: u8,
    pub remrssi: u8,
    pub txbuf: u8,
    pub noise: u8,
    pub remnoise: u8,
}
impl RADIO_STATUS_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RADIO_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RADIO_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.rxerrors = buf.get_u16_le();
        _struct.fixed = buf.get_u16_le();
        _struct.rssi = buf.get_u8();
        _struct.remrssi = buf.get_u8();
        _struct.txbuf = buf.get_u8();
        _struct.noise = buf.get_u8();
        _struct.remnoise = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.rxerrors);
        _tmp.put_u16_le(self.fixed);
        _tmp.put_u8(self.rssi);
        _tmp.put_u8(self.remrssi);
        _tmp.put_u8(self.txbuf);
        _tmp.put_u8(self.noise);
        _tmp.put_u8(self.remnoise);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FILE_TRANSFER_PROTOCOL_DATA {
    pub target_network: u8,
    pub target_system: u8,
    pub target_component: u8,
    pub payload: Vec<u8>, /* 251 elements */
}
impl FILE_TRANSFER_PROTOCOL_DATA {
    pub const ENCODED_LEN: usize = 254usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FILE_TRANSFER_PROTOCOL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FILE_TRANSFER_PROTOCOL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_network = buf.get_u8();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..251usize {
            let val = buf.get_u8();
            _struct.payload.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_network);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.payload {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TIMESYNC_DATA {
    pub tc1: i64,
    pub ts1: i64,
}
impl TIMESYNC_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TIMESYNC_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TIMESYNC_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.tc1 = buf.get_i64_le();
        _struct.ts1 = buf.get_i64_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i64_le(self.tc1);
        _tmp.put_i64_le(self.ts1);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_TRIGGER_DATA {
    pub time_usec: u64,
    pub seq: u32,
}
impl CAMERA_TRIGGER_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_TRIGGER_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_TRIGGER_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.seq = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.seq);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_GPS_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub vn: i16,
    pub ve: i16,
    pub vd: i16,
    pub cog: u16,
    pub fix_type: u8,
    pub satellites_visible: u8,
}
impl HIL_GPS_DATA {
    pub const ENCODED_LEN: usize = 36usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_GPS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_GPS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.eph = buf.get_u16_le();
        _struct.epv = buf.get_u16_le();
        _struct.vel = buf.get_u16_le();
        _struct.vn = buf.get_i16_le();
        _struct.ve = buf.get_i16_le();
        _struct.vd = buf.get_i16_le();
        _struct.cog = buf.get_u16_le();
        _struct.fix_type = buf.get_u8();
        _struct.satellites_visible = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_u16_le(self.eph);
        _tmp.put_u16_le(self.epv);
        _tmp.put_u16_le(self.vel);
        _tmp.put_i16_le(self.vn);
        _tmp.put_i16_le(self.ve);
        _tmp.put_i16_le(self.vd);
        _tmp.put_u16_le(self.cog);
        _tmp.put_u8(self.fix_type);
        _tmp.put_u8(self.satellites_visible);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_OPTICAL_FLOW_DATA {
    pub time_usec: u64,
    pub integration_time_us: u32,
    pub integrated_x: f32,
    pub integrated_y: f32,
    pub integrated_xgyro: f32,
    pub integrated_ygyro: f32,
    pub integrated_zgyro: f32,
    pub time_delta_distance_us: u32,
    pub distance: f32,
    pub temperature: i16,
    pub sensor_id: u8,
    pub quality: u8,
}
impl HIL_OPTICAL_FLOW_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_OPTICAL_FLOW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_OPTICAL_FLOW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.integration_time_us = buf.get_u32_le();
        _struct.integrated_x = buf.get_f32_le();
        _struct.integrated_y = buf.get_f32_le();
        _struct.integrated_xgyro = buf.get_f32_le();
        _struct.integrated_ygyro = buf.get_f32_le();
        _struct.integrated_zgyro = buf.get_f32_le();
        _struct.time_delta_distance_us = buf.get_u32_le();
        _struct.distance = buf.get_f32_le();
        _struct.temperature = buf.get_i16_le();
        _struct.sensor_id = buf.get_u8();
        _struct.quality = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.integration_time_us);
        _tmp.put_f32_le(self.integrated_x);
        _tmp.put_f32_le(self.integrated_y);
        _tmp.put_f32_le(self.integrated_xgyro);
        _tmp.put_f32_le(self.integrated_ygyro);
        _tmp.put_f32_le(self.integrated_zgyro);
        _tmp.put_u32_le(self.time_delta_distance_us);
        _tmp.put_f32_le(self.distance);
        _tmp.put_i16_le(self.temperature);
        _tmp.put_u8(self.sensor_id);
        _tmp.put_u8(self.quality);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIL_STATE_QUATERNION_DATA {
    pub time_usec: u64,
    pub attitude_quaternion: [f32; 4],
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub ind_airspeed: u16,
    pub true_airspeed: u16,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
}
impl HIL_STATE_QUATERNION_DATA {
    pub const ENCODED_LEN: usize = 64usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIL_STATE_QUATERNION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIL_STATE_QUATERNION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.attitude_quaternion[idx] = val;
        }
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.vx = buf.get_i16_le();
        _struct.vy = buf.get_i16_le();
        _struct.vz = buf.get_i16_le();
        _struct.ind_airspeed = buf.get_u16_le();
        _struct.true_airspeed = buf.get_u16_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.attitude_quaternion {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i16_le(self.vx);
        _tmp.put_i16_le(self.vy);
        _tmp.put_i16_le(self.vz);
        _tmp.put_u16_le(self.ind_airspeed);
        _tmp.put_u16_le(self.true_airspeed);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_IMU2_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}
impl SCALED_IMU2_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_IMU2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_IMU2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        _struct.xgyro = buf.get_i16_le();
        _struct.ygyro = buf.get_i16_le();
        _struct.zgyro = buf.get_i16_le();
        _struct.xmag = buf.get_i16_le();
        _struct.ymag = buf.get_i16_le();
        _struct.zmag = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp.put_i16_le(self.xgyro);
        _tmp.put_i16_le(self.ygyro);
        _tmp.put_i16_le(self.zgyro);
        _tmp.put_i16_le(self.xmag);
        _tmp.put_i16_le(self.ymag);
        _tmp.put_i16_le(self.zmag);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOG_REQUEST_LIST_DATA {
    pub start: u16,
    pub end: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl LOG_REQUEST_LIST_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_REQUEST_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_REQUEST_LIST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.start = buf.get_u16_le();
        _struct.end = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.start);
        _tmp.put_u16_le(self.end);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOG_ENTRY_DATA {
    pub time_utc: u32,
    pub size: u32,
    pub id: u16,
    pub num_logs: u16,
    pub last_log_num: u16,
}
impl LOG_ENTRY_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_ENTRY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_ENTRY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_utc = buf.get_u32_le();
        _struct.size = buf.get_u32_le();
        _struct.id = buf.get_u16_le();
        _struct.num_logs = buf.get_u16_le();
        _struct.last_log_num = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_utc);
        _tmp.put_u32_le(self.size);
        _tmp.put_u16_le(self.id);
        _tmp.put_u16_le(self.num_logs);
        _tmp.put_u16_le(self.last_log_num);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOG_REQUEST_DATA_DATA {
    pub ofs: u32,
    pub count: u32,
    pub id: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl LOG_REQUEST_DATA_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_REQUEST_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_REQUEST_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.ofs = buf.get_u32_le();
        _struct.count = buf.get_u32_le();
        _struct.id = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ofs);
        _tmp.put_u32_le(self.count);
        _tmp.put_u16_le(self.id);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOG_DATA_DATA {
    pub ofs: u32,
    pub id: u16,
    pub count: u8,
    pub data: Vec<u8>, /* 90 elements */
}
impl LOG_DATA_DATA {
    pub const ENCODED_LEN: usize = 97usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.ofs = buf.get_u32_le();
        _struct.id = buf.get_u16_le();
        _struct.count = buf.get_u8();
        for _ in 0..90usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ofs);
        _tmp.put_u16_le(self.id);
        _tmp.put_u8(self.count);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOG_ERASE_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl LOG_ERASE_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_ERASE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_ERASE_DATA::ENCODED_LEN];
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
pub struct LOG_REQUEST_END_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl LOG_REQUEST_END_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOG_REQUEST_END_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOG_REQUEST_END_DATA::ENCODED_LEN];
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
pub struct GPS_INJECT_DATA_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub len: u8,
    pub data: Vec<u8>, /* 110 elements */
}
impl GPS_INJECT_DATA_DATA {
    pub const ENCODED_LEN: usize = 113usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_INJECT_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_INJECT_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.len = buf.get_u8();
        for _ in 0..110usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS2_RAW_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub dgps_age: u32,
    pub eph: u16,
    pub epv: u16,
    pub vel: u16,
    pub cog: u16,
    pub fix_type: GpsFixType,
    pub satellites_visible: u8,
    pub dgps_numch: u8,
}
impl GPS2_RAW_DATA {
    pub const ENCODED_LEN: usize = 35usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS2_RAW_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS2_RAW_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.dgps_age = buf.get_u32_le();
        _struct.eph = buf.get_u16_le();
        _struct.epv = buf.get_u16_le();
        _struct.vel = buf.get_u16_le();
        _struct.cog = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GpsFixType".to_string(),
            value: tmp as u32,
        })?;
        _struct.satellites_visible = buf.get_u8();
        _struct.dgps_numch = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_u32_le(self.dgps_age);
        _tmp.put_u16_le(self.eph);
        _tmp.put_u16_le(self.epv);
        _tmp.put_u16_le(self.vel);
        _tmp.put_u16_le(self.cog);
        _tmp.put_u8(self.fix_type as u8);
        _tmp.put_u8(self.satellites_visible);
        _tmp.put_u8(self.dgps_numch);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct POWER_STATUS_DATA {
    pub Vcc: u16,
    pub Vservo: u16,
    pub flags: MavPowerStatus,
}
impl POWER_STATUS_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < POWER_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; POWER_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.Vcc = buf.get_u16_le();
        _struct.Vservo = buf.get_u16_le();
        let tmp = buf.get_u16_le();
        _struct.flags = MavPowerStatus::from_bits(tmp & MavPowerStatus::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "MavPowerStatus".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.Vcc);
        _tmp.put_u16_le(self.Vservo);
        _tmp.put_u16_le(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SERIAL_CONTROL_DATA {
    pub baudrate: u32,
    pub timeout: u16,
    pub device: SerialControlDev,
    pub flags: SerialControlFlag,
    pub count: u8,
    pub data: Vec<u8>, /* 70 elements */
}
impl SERIAL_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 79usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SERIAL_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SERIAL_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.baudrate = buf.get_u32_le();
        _struct.timeout = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.device = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "SerialControlDev".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.flags = SerialControlFlag::from_bits(tmp & SerialControlFlag::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "SerialControlFlag".to_string(),
                value: tmp as u32,
            },
        )?;
        _struct.count = buf.get_u8();
        for _ in 0..70usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.baudrate);
        _tmp.put_u16_le(self.timeout);
        _tmp.put_u8(self.device as u8);
        _tmp.put_u8(self.flags.bits());
        _tmp.put_u8(self.count);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_RTK_DATA {
    pub time_last_baseline_ms: u32,
    pub tow: u32,
    pub baseline_a_mm: i32,
    pub baseline_b_mm: i32,
    pub baseline_c_mm: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
    pub wn: u16,
    pub rtk_receiver_id: u8,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_coords_type: RtkBaselineCoordinateSystem,
}
impl GPS_RTK_DATA {
    pub const ENCODED_LEN: usize = 35usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_RTK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_RTK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_last_baseline_ms = buf.get_u32_le();
        _struct.tow = buf.get_u32_le();
        _struct.baseline_a_mm = buf.get_i32_le();
        _struct.baseline_b_mm = buf.get_i32_le();
        _struct.baseline_c_mm = buf.get_i32_le();
        _struct.accuracy = buf.get_u32_le();
        _struct.iar_num_hypotheses = buf.get_i32_le();
        _struct.wn = buf.get_u16_le();
        _struct.rtk_receiver_id = buf.get_u8();
        _struct.rtk_health = buf.get_u8();
        _struct.rtk_rate = buf.get_u8();
        _struct.nsats = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.baseline_coords_type =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "RtkBaselineCoordinateSystem".to_string(),
                value: tmp as u32,
            })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_last_baseline_ms);
        _tmp.put_u32_le(self.tow);
        _tmp.put_i32_le(self.baseline_a_mm);
        _tmp.put_i32_le(self.baseline_b_mm);
        _tmp.put_i32_le(self.baseline_c_mm);
        _tmp.put_u32_le(self.accuracy);
        _tmp.put_i32_le(self.iar_num_hypotheses);
        _tmp.put_u16_le(self.wn);
        _tmp.put_u8(self.rtk_receiver_id);
        _tmp.put_u8(self.rtk_health);
        _tmp.put_u8(self.rtk_rate);
        _tmp.put_u8(self.nsats);
        _tmp.put_u8(self.baseline_coords_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS2_RTK_DATA {
    pub time_last_baseline_ms: u32,
    pub tow: u32,
    pub baseline_a_mm: i32,
    pub baseline_b_mm: i32,
    pub baseline_c_mm: i32,
    pub accuracy: u32,
    pub iar_num_hypotheses: i32,
    pub wn: u16,
    pub rtk_receiver_id: u8,
    pub rtk_health: u8,
    pub rtk_rate: u8,
    pub nsats: u8,
    pub baseline_coords_type: RtkBaselineCoordinateSystem,
}
impl GPS2_RTK_DATA {
    pub const ENCODED_LEN: usize = 35usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS2_RTK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS2_RTK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_last_baseline_ms = buf.get_u32_le();
        _struct.tow = buf.get_u32_le();
        _struct.baseline_a_mm = buf.get_i32_le();
        _struct.baseline_b_mm = buf.get_i32_le();
        _struct.baseline_c_mm = buf.get_i32_le();
        _struct.accuracy = buf.get_u32_le();
        _struct.iar_num_hypotheses = buf.get_i32_le();
        _struct.wn = buf.get_u16_le();
        _struct.rtk_receiver_id = buf.get_u8();
        _struct.rtk_health = buf.get_u8();
        _struct.rtk_rate = buf.get_u8();
        _struct.nsats = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.baseline_coords_type =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "RtkBaselineCoordinateSystem".to_string(),
                value: tmp as u32,
            })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_last_baseline_ms);
        _tmp.put_u32_le(self.tow);
        _tmp.put_i32_le(self.baseline_a_mm);
        _tmp.put_i32_le(self.baseline_b_mm);
        _tmp.put_i32_le(self.baseline_c_mm);
        _tmp.put_u32_le(self.accuracy);
        _tmp.put_i32_le(self.iar_num_hypotheses);
        _tmp.put_u16_le(self.wn);
        _tmp.put_u8(self.rtk_receiver_id);
        _tmp.put_u8(self.rtk_health);
        _tmp.put_u8(self.rtk_rate);
        _tmp.put_u8(self.nsats);
        _tmp.put_u8(self.baseline_coords_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_IMU3_DATA {
    pub time_boot_ms: u32,
    pub xacc: i16,
    pub yacc: i16,
    pub zacc: i16,
    pub xgyro: i16,
    pub ygyro: i16,
    pub zgyro: i16,
    pub xmag: i16,
    pub ymag: i16,
    pub zmag: i16,
}
impl SCALED_IMU3_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_IMU3_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_IMU3_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.xacc = buf.get_i16_le();
        _struct.yacc = buf.get_i16_le();
        _struct.zacc = buf.get_i16_le();
        _struct.xgyro = buf.get_i16_le();
        _struct.ygyro = buf.get_i16_le();
        _struct.zgyro = buf.get_i16_le();
        _struct.xmag = buf.get_i16_le();
        _struct.ymag = buf.get_i16_le();
        _struct.zmag = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i16_le(self.xacc);
        _tmp.put_i16_le(self.yacc);
        _tmp.put_i16_le(self.zacc);
        _tmp.put_i16_le(self.xgyro);
        _tmp.put_i16_le(self.ygyro);
        _tmp.put_i16_le(self.zgyro);
        _tmp.put_i16_le(self.xmag);
        _tmp.put_i16_le(self.ymag);
        _tmp.put_i16_le(self.zmag);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA_TRANSMISSION_HANDSHAKE_DATA {
    pub size: u32,
    pub width: u16,
    pub height: u16,
    pub packets: u16,
    pub mavtype: MavlinkDataStreamType,
    pub payload: u8,
    pub jpg_quality: u8,
}
impl DATA_TRANSMISSION_HANDSHAKE_DATA {
    pub const ENCODED_LEN: usize = 13usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA_TRANSMISSION_HANDSHAKE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA_TRANSMISSION_HANDSHAKE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.size = buf.get_u32_le();
        _struct.width = buf.get_u16_le();
        _struct.height = buf.get_u16_le();
        _struct.packets = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavlinkDataStreamType".to_string(),
            value: tmp as u32,
        })?;
        _struct.payload = buf.get_u8();
        _struct.jpg_quality = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.size);
        _tmp.put_u16_le(self.width);
        _tmp.put_u16_le(self.height);
        _tmp.put_u16_le(self.packets);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.payload);
        _tmp.put_u8(self.jpg_quality);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ENCAPSULATED_DATA_DATA {
    pub seqnr: u16,
    pub data: Vec<u8>, /* 253 elements */
}
impl ENCAPSULATED_DATA_DATA {
    pub const ENCODED_LEN: usize = 255usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ENCAPSULATED_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ENCAPSULATED_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.seqnr = buf.get_u16_le();
        for _ in 0..253usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.seqnr);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DISTANCE_SENSOR_DATA {
    pub time_boot_ms: u32,
    pub min_distance: u16,
    pub max_distance: u16,
    pub current_distance: u16,
    pub mavtype: MavDistanceSensor,
    pub id: u8,
    pub orientation: MavSensorOrientation,
    pub covariance: u8,
}
impl DISTANCE_SENSOR_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DISTANCE_SENSOR_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DISTANCE_SENSOR_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.min_distance = buf.get_u16_le();
        _struct.max_distance = buf.get_u16_le();
        _struct.current_distance = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavDistanceSensor".to_string(),
            value: tmp as u32,
        })?;
        _struct.id = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.orientation = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavSensorOrientation".to_string(),
            value: tmp as u32,
        })?;
        _struct.covariance = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u16_le(self.min_distance);
        _tmp.put_u16_le(self.max_distance);
        _tmp.put_u16_le(self.current_distance);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.id);
        _tmp.put_u8(self.orientation as u8);
        _tmp.put_u8(self.covariance);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TERRAIN_REQUEST_DATA {
    pub mask: u64,
    pub lat: i32,
    pub lon: i32,
    pub grid_spacing: u16,
}
impl TERRAIN_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TERRAIN_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TERRAIN_REQUEST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mask = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.grid_spacing = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.mask);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_u16_le(self.grid_spacing);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TERRAIN_DATA_DATA {
    pub lat: i32,
    pub lon: i32,
    pub grid_spacing: u16,
    pub data: [i16; 16],
    pub gridbit: u8,
}
impl TERRAIN_DATA_DATA {
    pub const ENCODED_LEN: usize = 43usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TERRAIN_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TERRAIN_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.grid_spacing = buf.get_u16_le();
        for idx in 0..16usize {
            let val = buf.get_i16_le();
            _struct.data[idx] = val;
        }
        _struct.gridbit = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_u16_le(self.grid_spacing);
        for val in &self.data {
            _tmp.put_i16_le(*val);
        }
        _tmp.put_u8(self.gridbit);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TERRAIN_CHECK_DATA {
    pub lat: i32,
    pub lon: i32,
}
impl TERRAIN_CHECK_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TERRAIN_CHECK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TERRAIN_CHECK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TERRAIN_REPORT_DATA {
    pub lat: i32,
    pub lon: i32,
    pub terrain_height: f32,
    pub current_height: f32,
    pub spacing: u16,
    pub pending: u16,
    pub loaded: u16,
}
impl TERRAIN_REPORT_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TERRAIN_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TERRAIN_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.terrain_height = buf.get_f32_le();
        _struct.current_height = buf.get_f32_le();
        _struct.spacing = buf.get_u16_le();
        _struct.pending = buf.get_u16_le();
        _struct.loaded = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_f32_le(self.terrain_height);
        _tmp.put_f32_le(self.current_height);
        _tmp.put_u16_le(self.spacing);
        _tmp.put_u16_le(self.pending);
        _tmp.put_u16_le(self.loaded);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_PRESSURE2_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}
impl SCALED_PRESSURE2_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_PRESSURE2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_PRESSURE2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.press_abs = buf.get_f32_le();
        _struct.press_diff = buf.get_f32_le();
        _struct.temperature = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.press_abs);
        _tmp.put_f32_le(self.press_diff);
        _tmp.put_i16_le(self.temperature);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ATT_POS_MOCAP_DATA {
    pub time_usec: u64,
    pub q: [f32; 4],
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ATT_POS_MOCAP_DATA {
    pub const ENCODED_LEN: usize = 36usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ATT_POS_MOCAP_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ATT_POS_MOCAP_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_ACTUATOR_CONTROL_TARGET_DATA {
    pub time_usec: u64,
    pub controls: [f32; 8],
    pub group_mlx: u8,
    pub target_system: u8,
    pub target_component: u8,
}
impl SET_ACTUATOR_CONTROL_TARGET_DATA {
    pub const ENCODED_LEN: usize = 43usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_ACTUATOR_CONTROL_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_ACTUATOR_CONTROL_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..8usize {
            let val = buf.get_f32_le();
            _struct.controls[idx] = val;
        }
        _struct.group_mlx = buf.get_u8();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.controls {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.group_mlx);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ACTUATOR_CONTROL_TARGET_DATA {
    pub time_usec: u64,
    pub controls: [f32; 8],
    pub group_mlx: u8,
}
impl ACTUATOR_CONTROL_TARGET_DATA {
    pub const ENCODED_LEN: usize = 41usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ACTUATOR_CONTROL_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ACTUATOR_CONTROL_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..8usize {
            let val = buf.get_f32_le();
            _struct.controls[idx] = val;
        }
        _struct.group_mlx = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.controls {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.group_mlx);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ALTITUDE_DATA {
    pub time_usec: u64,
    pub altitude_monotonic: f32,
    pub altitude_amsl: f32,
    pub altitude_local: f32,
    pub altitude_relative: f32,
    pub altitude_terrain: f32,
    pub bottom_clearance: f32,
}
impl ALTITUDE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ALTITUDE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ALTITUDE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.altitude_monotonic = buf.get_f32_le();
        _struct.altitude_amsl = buf.get_f32_le();
        _struct.altitude_local = buf.get_f32_le();
        _struct.altitude_relative = buf.get_f32_le();
        _struct.altitude_terrain = buf.get_f32_le();
        _struct.bottom_clearance = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.altitude_monotonic);
        _tmp.put_f32_le(self.altitude_amsl);
        _tmp.put_f32_le(self.altitude_local);
        _tmp.put_f32_le(self.altitude_relative);
        _tmp.put_f32_le(self.altitude_terrain);
        _tmp.put_f32_le(self.bottom_clearance);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RESOURCE_REQUEST_DATA {
    pub request_id: u8,
    pub uri_type: u8,
    pub uri: Vec<u8>, /* 120 elements */
    pub transfer_type: u8,
    pub storage: Vec<u8>, /* 120 elements */
}
impl RESOURCE_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 243usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RESOURCE_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RESOURCE_REQUEST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.request_id = buf.get_u8();
        _struct.uri_type = buf.get_u8();
        for _ in 0..120usize {
            let val = buf.get_u8();
            _struct.uri.push(val);
        }
        _struct.transfer_type = buf.get_u8();
        for _ in 0..120usize {
            let val = buf.get_u8();
            _struct.storage.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.request_id);
        _tmp.put_u8(self.uri_type);
        for val in &self.uri {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.transfer_type);
        for val in &self.storage {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SCALED_PRESSURE3_DATA {
    pub time_boot_ms: u32,
    pub press_abs: f32,
    pub press_diff: f32,
    pub temperature: i16,
}
impl SCALED_PRESSURE3_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SCALED_PRESSURE3_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SCALED_PRESSURE3_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.press_abs = buf.get_f32_le();
        _struct.press_diff = buf.get_f32_le();
        _struct.temperature = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.press_abs);
        _tmp.put_f32_le(self.press_diff);
        _tmp.put_i16_le(self.temperature);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FOLLOW_TARGET_DATA {
    pub timestamp: u64,
    pub custom_state: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: f32,
    pub vel: [f32; 3],
    pub acc: [f32; 3],
    pub attitude_q: [f32; 4],
    pub rates: [f32; 3],
    pub position_cov: [f32; 3],
    pub est_capabilities: u8,
}
impl FOLLOW_TARGET_DATA {
    pub const ENCODED_LEN: usize = 93usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FOLLOW_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FOLLOW_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.custom_state = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_f32_le();
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.vel[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.acc[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.attitude_q[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.rates[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.position_cov[idx] = val;
        }
        _struct.est_capabilities = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u64_le(self.custom_state);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_f32_le(self.alt);
        for val in &self.vel {
            _tmp.put_f32_le(*val);
        }
        for val in &self.acc {
            _tmp.put_f32_le(*val);
        }
        for val in &self.attitude_q {
            _tmp.put_f32_le(*val);
        }
        for val in &self.rates {
            _tmp.put_f32_le(*val);
        }
        for val in &self.position_cov {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.est_capabilities);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CONTROL_SYSTEM_STATE_DATA {
    pub time_usec: u64,
    pub x_acc: f32,
    pub y_acc: f32,
    pub z_acc: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub z_vel: f32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub z_pos: f32,
    pub airspeed: f32,
    pub vel_variance: [f32; 3],
    pub pos_variance: [f32; 3],
    pub q: [f32; 4],
    pub roll_rate: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
}
impl CONTROL_SYSTEM_STATE_DATA {
    pub const ENCODED_LEN: usize = 100usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CONTROL_SYSTEM_STATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CONTROL_SYSTEM_STATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.x_acc = buf.get_f32_le();
        _struct.y_acc = buf.get_f32_le();
        _struct.z_acc = buf.get_f32_le();
        _struct.x_vel = buf.get_f32_le();
        _struct.y_vel = buf.get_f32_le();
        _struct.z_vel = buf.get_f32_le();
        _struct.x_pos = buf.get_f32_le();
        _struct.y_pos = buf.get_f32_le();
        _struct.z_pos = buf.get_f32_le();
        _struct.airspeed = buf.get_f32_le();
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.vel_variance[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.pos_variance[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.roll_rate = buf.get_f32_le();
        _struct.pitch_rate = buf.get_f32_le();
        _struct.yaw_rate = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.x_acc);
        _tmp.put_f32_le(self.y_acc);
        _tmp.put_f32_le(self.z_acc);
        _tmp.put_f32_le(self.x_vel);
        _tmp.put_f32_le(self.y_vel);
        _tmp.put_f32_le(self.z_vel);
        _tmp.put_f32_le(self.x_pos);
        _tmp.put_f32_le(self.y_pos);
        _tmp.put_f32_le(self.z_pos);
        _tmp.put_f32_le(self.airspeed);
        for val in &self.vel_variance {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_variance {
            _tmp.put_f32_le(*val);
        }
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.roll_rate);
        _tmp.put_f32_le(self.pitch_rate);
        _tmp.put_f32_le(self.yaw_rate);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BATTERY_STATUS_DATA {
    pub current_consumed: i32,
    pub energy_consumed: i32,
    pub temperature: i16,
    pub voltages: [u16; 10],
    pub current_battery: i16,
    pub id: u8,
    pub battery_function: MavBatteryFunction,
    pub mavtype: MavBatteryType,
    pub battery_remaining: i8,
}
impl BATTERY_STATUS_DATA {
    pub const ENCODED_LEN: usize = 36usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < BATTERY_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; BATTERY_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.current_consumed = buf.get_i32_le();
        _struct.energy_consumed = buf.get_i32_le();
        _struct.temperature = buf.get_i16_le();
        for idx in 0..10usize {
            let val = buf.get_u16_le();
            _struct.voltages[idx] = val;
        }
        _struct.current_battery = buf.get_i16_le();
        _struct.id = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.battery_function = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavBatteryFunction".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavBatteryType".to_string(),
            value: tmp as u32,
        })?;
        _struct.battery_remaining = buf.get_i8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.current_consumed);
        _tmp.put_i32_le(self.energy_consumed);
        _tmp.put_i16_le(self.temperature);
        for val in &self.voltages {
            _tmp.put_u16_le(*val);
        }
        _tmp.put_i16_le(self.current_battery);
        _tmp.put_u8(self.id);
        _tmp.put_u8(self.battery_function as u8);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_i8(self.battery_remaining);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AUTOPILOT_VERSION_DATA {
    pub capabilities: MavProtocolCapability,
    pub uid: u64,
    pub flight_sw_version: u32,
    pub middleware_sw_version: u32,
    pub os_sw_version: u32,
    pub board_version: u32,
    pub vendor_id: u16,
    pub product_id: u16,
    pub flight_custom_version: [u8; 8],
    pub middleware_custom_version: [u8; 8],
    pub os_custom_version: [u8; 8],
}
impl AUTOPILOT_VERSION_DATA {
    pub const ENCODED_LEN: usize = 60usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AUTOPILOT_VERSION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AUTOPILOT_VERSION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u64_le();
        _struct.capabilities = MavProtocolCapability::from_bits(
            tmp & MavProtocolCapability::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavProtocolCapability".to_string(),
            value: tmp as u32,
        })?;
        _struct.uid = buf.get_u64_le();
        _struct.flight_sw_version = buf.get_u32_le();
        _struct.middleware_sw_version = buf.get_u32_le();
        _struct.os_sw_version = buf.get_u32_le();
        _struct.board_version = buf.get_u32_le();
        _struct.vendor_id = buf.get_u16_le();
        _struct.product_id = buf.get_u16_le();
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.flight_custom_version[idx] = val;
        }
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.middleware_custom_version[idx] = val;
        }
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.os_custom_version[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.capabilities.bits());
        _tmp.put_u64_le(self.uid);
        _tmp.put_u32_le(self.flight_sw_version);
        _tmp.put_u32_le(self.middleware_sw_version);
        _tmp.put_u32_le(self.os_sw_version);
        _tmp.put_u32_le(self.board_version);
        _tmp.put_u16_le(self.vendor_id);
        _tmp.put_u16_le(self.product_id);
        for val in &self.flight_custom_version {
            _tmp.put_u8(*val);
        }
        for val in &self.middleware_custom_version {
            _tmp.put_u8(*val);
        }
        for val in &self.os_custom_version {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LANDING_TARGET_DATA {
    pub time_usec: u64,
    pub angle_x: f32,
    pub angle_y: f32,
    pub distance: f32,
    pub size_x: f32,
    pub size_y: f32,
    pub target_num: u8,
    pub frame: MavFrame,
}
impl LANDING_TARGET_DATA {
    pub const ENCODED_LEN: usize = 30usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LANDING_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LANDING_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.angle_x = buf.get_f32_le();
        _struct.angle_y = buf.get_f32_le();
        _struct.distance = buf.get_f32_le();
        _struct.size_x = buf.get_f32_le();
        _struct.size_y = buf.get_f32_le();
        _struct.target_num = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.angle_x);
        _tmp.put_f32_le(self.angle_y);
        _tmp.put_f32_le(self.distance);
        _tmp.put_f32_le(self.size_x);
        _tmp.put_f32_le(self.size_y);
        _tmp.put_u8(self.target_num);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FENCE_STATUS_DATA {
    pub breach_time: u32,
    pub breach_count: u16,
    pub breach_status: u8,
    pub breach_type: FenceBreach,
}
impl FENCE_STATUS_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FENCE_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FENCE_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.breach_time = buf.get_u32_le();
        _struct.breach_count = buf.get_u16_le();
        _struct.breach_status = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.breach_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "FenceBreach".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.breach_time);
        _tmp.put_u16_le(self.breach_count);
        _tmp.put_u8(self.breach_status);
        _tmp.put_u8(self.breach_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ESTIMATOR_STATUS_DATA {
    pub time_usec: u64,
    pub vel_ratio: f32,
    pub pos_horiz_ratio: f32,
    pub pos_vert_ratio: f32,
    pub mag_ratio: f32,
    pub hagl_ratio: f32,
    pub tas_ratio: f32,
    pub pos_horiz_accuracy: f32,
    pub pos_vert_accuracy: f32,
    pub flags: EstimatorStatusFlags,
}
impl ESTIMATOR_STATUS_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ESTIMATOR_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ESTIMATOR_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.vel_ratio = buf.get_f32_le();
        _struct.pos_horiz_ratio = buf.get_f32_le();
        _struct.pos_vert_ratio = buf.get_f32_le();
        _struct.mag_ratio = buf.get_f32_le();
        _struct.hagl_ratio = buf.get_f32_le();
        _struct.tas_ratio = buf.get_f32_le();
        _struct.pos_horiz_accuracy = buf.get_f32_le();
        _struct.pos_vert_accuracy = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.flags = EstimatorStatusFlags::from_bits(tmp & EstimatorStatusFlags::all().bits())
            .ok_or(ParserError::InvalidFlag {
            flag_type: "EstimatorStatusFlags".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.vel_ratio);
        _tmp.put_f32_le(self.pos_horiz_ratio);
        _tmp.put_f32_le(self.pos_vert_ratio);
        _tmp.put_f32_le(self.mag_ratio);
        _tmp.put_f32_le(self.hagl_ratio);
        _tmp.put_f32_le(self.tas_ratio);
        _tmp.put_f32_le(self.pos_horiz_accuracy);
        _tmp.put_f32_le(self.pos_vert_accuracy);
        _tmp.put_u16_le(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WIND_COV_DATA {
    pub time_usec: u64,
    pub wind_x: f32,
    pub wind_y: f32,
    pub wind_z: f32,
    pub var_horiz: f32,
    pub var_vert: f32,
    pub wind_alt: f32,
    pub horiz_accuracy: f32,
    pub vert_accuracy: f32,
}
impl WIND_COV_DATA {
    pub const ENCODED_LEN: usize = 40usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < WIND_COV_DATA::ENCODED_LEN {
            let mut payload_buf = [0; WIND_COV_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.wind_x = buf.get_f32_le();
        _struct.wind_y = buf.get_f32_le();
        _struct.wind_z = buf.get_f32_le();
        _struct.var_horiz = buf.get_f32_le();
        _struct.var_vert = buf.get_f32_le();
        _struct.wind_alt = buf.get_f32_le();
        _struct.horiz_accuracy = buf.get_f32_le();
        _struct.vert_accuracy = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.wind_x);
        _tmp.put_f32_le(self.wind_y);
        _tmp.put_f32_le(self.wind_z);
        _tmp.put_f32_le(self.var_horiz);
        _tmp.put_f32_le(self.var_vert);
        _tmp.put_f32_le(self.wind_alt);
        _tmp.put_f32_le(self.horiz_accuracy);
        _tmp.put_f32_le(self.vert_accuracy);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_INPUT_DATA {
    pub time_usec: u64,
    pub time_week_ms: u32,
    pub lat: i32,
    pub lon: i32,
    pub alt: f32,
    pub hdop: f32,
    pub vdop: f32,
    pub vn: f32,
    pub ve: f32,
    pub vd: f32,
    pub speed_accuracy: f32,
    pub horiz_accuracy: f32,
    pub vert_accuracy: f32,
    pub ignore_flags: GpsInputIgnoreFlags,
    pub time_week: u16,
    pub gps_id: u8,
    pub fix_type: u8,
    pub satellites_visible: u8,
}
impl GPS_INPUT_DATA {
    pub const ENCODED_LEN: usize = 63usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_INPUT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_INPUT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.time_week_ms = buf.get_u32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_f32_le();
        _struct.hdop = buf.get_f32_le();
        _struct.vdop = buf.get_f32_le();
        _struct.vn = buf.get_f32_le();
        _struct.ve = buf.get_f32_le();
        _struct.vd = buf.get_f32_le();
        _struct.speed_accuracy = buf.get_f32_le();
        _struct.horiz_accuracy = buf.get_f32_le();
        _struct.vert_accuracy = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.ignore_flags = GpsInputIgnoreFlags::from_bits(
            tmp & GpsInputIgnoreFlags::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "GpsInputIgnoreFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.time_week = buf.get_u16_le();
        _struct.gps_id = buf.get_u8();
        _struct.fix_type = buf.get_u8();
        _struct.satellites_visible = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.time_week_ms);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_f32_le(self.alt);
        _tmp.put_f32_le(self.hdop);
        _tmp.put_f32_le(self.vdop);
        _tmp.put_f32_le(self.vn);
        _tmp.put_f32_le(self.ve);
        _tmp.put_f32_le(self.vd);
        _tmp.put_f32_le(self.speed_accuracy);
        _tmp.put_f32_le(self.horiz_accuracy);
        _tmp.put_f32_le(self.vert_accuracy);
        _tmp.put_u16_le(self.ignore_flags.bits());
        _tmp.put_u16_le(self.time_week);
        _tmp.put_u8(self.gps_id);
        _tmp.put_u8(self.fix_type);
        _tmp.put_u8(self.satellites_visible);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GPS_RTCM_DATA_DATA {
    pub flags: u8,
    pub len: u8,
    pub data: Vec<u8>, /* 180 elements */
}
impl GPS_RTCM_DATA_DATA {
    pub const ENCODED_LEN: usize = 182usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GPS_RTCM_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GPS_RTCM_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.flags = buf.get_u8();
        _struct.len = buf.get_u8();
        for _ in 0..180usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.flags);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIGH_LATENCY_DATA {
    pub custom_mode: u32,
    pub latitude: i32,
    pub longitude: i32,
    pub roll: i16,
    pub pitch: i16,
    pub heading: u16,
    pub heading_sp: i16,
    pub altitude_amsl: i16,
    pub altitude_sp: i16,
    pub wp_distance: u16,
    pub base_mode: MavModeFlag,
    pub landed_state: MavLandedState,
    pub throttle: i8,
    pub airspeed: u8,
    pub airspeed_sp: u8,
    pub groundspeed: u8,
    pub climb_rate: i8,
    pub gps_nsat: u8,
    pub gps_fix_type: GpsFixType,
    pub battery_remaining: u8,
    pub temperature: i8,
    pub temperature_air: i8,
    pub failsafe: u8,
    pub wp_num: u8,
}
impl HIGH_LATENCY_DATA {
    pub const ENCODED_LEN: usize = 40usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIGH_LATENCY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIGH_LATENCY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.custom_mode = buf.get_u32_le();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.roll = buf.get_i16_le();
        _struct.pitch = buf.get_i16_le();
        _struct.heading = buf.get_u16_le();
        _struct.heading_sp = buf.get_i16_le();
        _struct.altitude_amsl = buf.get_i16_le();
        _struct.altitude_sp = buf.get_i16_le();
        _struct.wp_distance = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.base_mode = MavModeFlag::from_bits(tmp & MavModeFlag::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "MavModeFlag".to_string(),
                value: tmp as u32,
            },
        )?;
        let tmp = buf.get_u8();
        _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavLandedState".to_string(),
            value: tmp as u32,
        })?;
        _struct.throttle = buf.get_i8();
        _struct.airspeed = buf.get_u8();
        _struct.airspeed_sp = buf.get_u8();
        _struct.groundspeed = buf.get_u8();
        _struct.climb_rate = buf.get_i8();
        _struct.gps_nsat = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.gps_fix_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GpsFixType".to_string(),
            value: tmp as u32,
        })?;
        _struct.battery_remaining = buf.get_u8();
        _struct.temperature = buf.get_i8();
        _struct.temperature_air = buf.get_i8();
        _struct.failsafe = buf.get_u8();
        _struct.wp_num = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.custom_mode);
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_i16_le(self.roll);
        _tmp.put_i16_le(self.pitch);
        _tmp.put_u16_le(self.heading);
        _tmp.put_i16_le(self.heading_sp);
        _tmp.put_i16_le(self.altitude_amsl);
        _tmp.put_i16_le(self.altitude_sp);
        _tmp.put_u16_le(self.wp_distance);
        _tmp.put_u8(self.base_mode.bits());
        _tmp.put_u8(self.landed_state as u8);
        _tmp.put_i8(self.throttle);
        _tmp.put_u8(self.airspeed);
        _tmp.put_u8(self.airspeed_sp);
        _tmp.put_u8(self.groundspeed);
        _tmp.put_i8(self.climb_rate);
        _tmp.put_u8(self.gps_nsat);
        _tmp.put_u8(self.gps_fix_type as u8);
        _tmp.put_u8(self.battery_remaining);
        _tmp.put_i8(self.temperature);
        _tmp.put_i8(self.temperature_air);
        _tmp.put_u8(self.failsafe);
        _tmp.put_u8(self.wp_num);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HIGH_LATENCY2_DATA {
    pub timestamp: u32,
    pub latitude: i32,
    pub longitude: i32,
    pub custom_mode: u16,
    pub altitude: i16,
    pub target_altitude: i16,
    pub target_distance: u16,
    pub wp_num: u16,
    pub failure_flags: HlFailureFlag,
    pub mavtype: MavType,
    pub autopilot: MavAutopilot,
    pub heading: u8,
    pub target_heading: u8,
    pub throttle: u8,
    pub airspeed: u8,
    pub airspeed_sp: u8,
    pub groundspeed: u8,
    pub windspeed: u8,
    pub wind_heading: u8,
    pub eph: u8,
    pub epv: u8,
    pub temperature_air: i8,
    pub climb_rate: i8,
    pub battery: i8,
    pub custom0: i8,
    pub custom1: i8,
    pub custom2: i8,
}
impl HIGH_LATENCY2_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HIGH_LATENCY2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HIGH_LATENCY2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u32_le();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.custom_mode = buf.get_u16_le();
        _struct.altitude = buf.get_i16_le();
        _struct.target_altitude = buf.get_i16_le();
        _struct.target_distance = buf.get_u16_le();
        _struct.wp_num = buf.get_u16_le();
        let tmp = buf.get_u16_le();
        _struct.failure_flags = HlFailureFlag::from_bits(tmp & HlFailureFlag::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "HlFailureFlag".to_string(),
                value: tmp as u32,
            },
        )?;
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
        _struct.heading = buf.get_u8();
        _struct.target_heading = buf.get_u8();
        _struct.throttle = buf.get_u8();
        _struct.airspeed = buf.get_u8();
        _struct.airspeed_sp = buf.get_u8();
        _struct.groundspeed = buf.get_u8();
        _struct.windspeed = buf.get_u8();
        _struct.wind_heading = buf.get_u8();
        _struct.eph = buf.get_u8();
        _struct.epv = buf.get_u8();
        _struct.temperature_air = buf.get_i8();
        _struct.climb_rate = buf.get_i8();
        _struct.battery = buf.get_i8();
        _struct.custom0 = buf.get_i8();
        _struct.custom1 = buf.get_i8();
        _struct.custom2 = buf.get_i8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.timestamp);
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_u16_le(self.custom_mode);
        _tmp.put_i16_le(self.altitude);
        _tmp.put_i16_le(self.target_altitude);
        _tmp.put_u16_le(self.target_distance);
        _tmp.put_u16_le(self.wp_num);
        _tmp.put_u16_le(self.failure_flags.bits());
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.autopilot as u8);
        _tmp.put_u8(self.heading);
        _tmp.put_u8(self.target_heading);
        _tmp.put_u8(self.throttle);
        _tmp.put_u8(self.airspeed);
        _tmp.put_u8(self.airspeed_sp);
        _tmp.put_u8(self.groundspeed);
        _tmp.put_u8(self.windspeed);
        _tmp.put_u8(self.wind_heading);
        _tmp.put_u8(self.eph);
        _tmp.put_u8(self.epv);
        _tmp.put_i8(self.temperature_air);
        _tmp.put_i8(self.climb_rate);
        _tmp.put_i8(self.battery);
        _tmp.put_i8(self.custom0);
        _tmp.put_i8(self.custom1);
        _tmp.put_i8(self.custom2);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VIBRATION_DATA {
    pub time_usec: u64,
    pub vibration_x: f32,
    pub vibration_y: f32,
    pub vibration_z: f32,
    pub clipping_0: u32,
    pub clipping_1: u32,
    pub clipping_2: u32,
}
impl VIBRATION_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VIBRATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VIBRATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.vibration_x = buf.get_f32_le();
        _struct.vibration_y = buf.get_f32_le();
        _struct.vibration_z = buf.get_f32_le();
        _struct.clipping_0 = buf.get_u32_le();
        _struct.clipping_1 = buf.get_u32_le();
        _struct.clipping_2 = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.vibration_x);
        _tmp.put_f32_le(self.vibration_y);
        _tmp.put_f32_le(self.vibration_z);
        _tmp.put_u32_le(self.clipping_0);
        _tmp.put_u32_le(self.clipping_1);
        _tmp.put_u32_le(self.clipping_2);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HOME_POSITION_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: [f32; 4],
    pub approach_x: f32,
    pub approach_y: f32,
    pub approach_z: f32,
}
impl HOME_POSITION_DATA {
    pub const ENCODED_LEN: usize = 52usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HOME_POSITION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HOME_POSITION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.altitude = buf.get_i32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.approach_x = buf.get_f32_le();
        _struct.approach_y = buf.get_f32_le();
        _struct.approach_z = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_i32_le(self.altitude);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.approach_x);
        _tmp.put_f32_le(self.approach_y);
        _tmp.put_f32_le(self.approach_z);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_HOME_POSITION_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: [f32; 4],
    pub approach_x: f32,
    pub approach_y: f32,
    pub approach_z: f32,
    pub target_system: u8,
}
impl SET_HOME_POSITION_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_HOME_POSITION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_HOME_POSITION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.altitude = buf.get_i32_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.approach_x = buf.get_f32_le();
        _struct.approach_y = buf.get_f32_le();
        _struct.approach_z = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_i32_le(self.altitude);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.approach_x);
        _tmp.put_f32_le(self.approach_y);
        _tmp.put_f32_le(self.approach_z);
        _tmp.put_u8(self.target_system);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MESSAGE_INTERVAL_DATA {
    pub interval_us: i32,
    pub message_id: u16,
}
impl MESSAGE_INTERVAL_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MESSAGE_INTERVAL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MESSAGE_INTERVAL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.interval_us = buf.get_i32_le();
        _struct.message_id = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.interval_us);
        _tmp.put_u16_le(self.message_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EXTENDED_SYS_STATE_DATA {
    pub vtol_state: MavVtolState,
    pub landed_state: MavLandedState,
}
impl EXTENDED_SYS_STATE_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < EXTENDED_SYS_STATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; EXTENDED_SYS_STATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.vtol_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavVtolState".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavLandedState".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.vtol_state as u8);
        _tmp.put_u8(self.landed_state as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ADSB_VEHICLE_DATA {
    pub ICAO_address: u32,
    pub lat: i32,
    pub lon: i32,
    pub altitude: i32,
    pub heading: u16,
    pub hor_velocity: u16,
    pub ver_velocity: i16,
    pub flags: AdsbFlags,
    pub squawk: u16,
    pub altitude_type: AdsbAltitudeType,
    pub callsign: [char; 9],
    pub emitter_type: AdsbEmitterType,
    pub tslc: u8,
}
impl ADSB_VEHICLE_DATA {
    pub const ENCODED_LEN: usize = 38usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ADSB_VEHICLE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ADSB_VEHICLE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.ICAO_address = buf.get_u32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.altitude = buf.get_i32_le();
        _struct.heading = buf.get_u16_le();
        _struct.hor_velocity = buf.get_u16_le();
        _struct.ver_velocity = buf.get_i16_le();
        let tmp = buf.get_u16_le();
        _struct.flags = AdsbFlags::from_bits(tmp & AdsbFlags::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "AdsbFlags".to_string(),
                value: tmp as u32,
            },
        )?;
        _struct.squawk = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.altitude_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "AdsbAltitudeType".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..9usize {
            let val = buf.get_u8() as char;
            _struct.callsign[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.emitter_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "AdsbEmitterType".to_string(),
            value: tmp as u32,
        })?;
        _struct.tslc = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.ICAO_address);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.altitude);
        _tmp.put_u16_le(self.heading);
        _tmp.put_u16_le(self.hor_velocity);
        _tmp.put_i16_le(self.ver_velocity);
        _tmp.put_u16_le(self.flags.bits());
        _tmp.put_u16_le(self.squawk);
        _tmp.put_u8(self.altitude_type as u8);
        for val in &self.callsign {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.emitter_type as u8);
        _tmp.put_u8(self.tslc);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COLLISION_DATA {
    pub id: u32,
    pub time_to_minimum_delta: f32,
    pub altitude_minimum_delta: f32,
    pub horizontal_minimum_delta: f32,
    pub src: MavCollisionSrc,
    pub action: MavCollisionAction,
    pub threat_level: MavCollisionThreatLevel,
}
impl COLLISION_DATA {
    pub const ENCODED_LEN: usize = 19usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COLLISION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COLLISION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.id = buf.get_u32_le();
        _struct.time_to_minimum_delta = buf.get_f32_le();
        _struct.altitude_minimum_delta = buf.get_f32_le();
        _struct.horizontal_minimum_delta = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.src = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCollisionSrc".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.action = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCollisionAction".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.threat_level = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavCollisionThreatLevel".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.id);
        _tmp.put_f32_le(self.time_to_minimum_delta);
        _tmp.put_f32_le(self.altitude_minimum_delta);
        _tmp.put_f32_le(self.horizontal_minimum_delta);
        _tmp.put_u8(self.src as u8);
        _tmp.put_u8(self.action as u8);
        _tmp.put_u8(self.threat_level as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct V2_EXTENSION_DATA {
    pub message_type: u16,
    pub target_network: u8,
    pub target_system: u8,
    pub target_component: u8,
    pub payload: Vec<u8>, /* 249 elements */
}
impl V2_EXTENSION_DATA {
    pub const ENCODED_LEN: usize = 254usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < V2_EXTENSION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; V2_EXTENSION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.message_type = buf.get_u16_le();
        _struct.target_network = buf.get_u8();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..249usize {
            let val = buf.get_u8();
            _struct.payload.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.message_type);
        _tmp.put_u8(self.target_network);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.payload {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MEMORY_VECT_DATA {
    pub address: u16,
    pub ver: u8,
    pub mavtype: u8,
    pub value: [i8; 32],
}
impl MEMORY_VECT_DATA {
    pub const ENCODED_LEN: usize = 36usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MEMORY_VECT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MEMORY_VECT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.address = buf.get_u16_le();
        _struct.ver = buf.get_u8();
        _struct.mavtype = buf.get_u8();
        for idx in 0..32usize {
            let val = buf.get_i8();
            _struct.value[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.address);
        _tmp.put_u8(self.ver);
        _tmp.put_u8(self.mavtype);
        for val in &self.value {
            _tmp.put_i8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEBUG_VECT_DATA {
    pub time_usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub name: [char; 10],
}
impl DEBUG_VECT_DATA {
    pub const ENCODED_LEN: usize = 30usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEBUG_VECT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEBUG_VECT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        for idx in 0..10usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NAMED_VALUE_FLOAT_DATA {
    pub time_boot_ms: u32,
    pub value: f32,
    pub name: [char; 10],
}
impl NAMED_VALUE_FLOAT_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < NAMED_VALUE_FLOAT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; NAMED_VALUE_FLOAT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.value = buf.get_f32_le();
        for idx in 0..10usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.value);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NAMED_VALUE_INT_DATA {
    pub time_boot_ms: u32,
    pub value: i32,
    pub name: [char; 10],
}
impl NAMED_VALUE_INT_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < NAMED_VALUE_INT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; NAMED_VALUE_INT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.value = buf.get_i32_le();
        for idx in 0..10usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.value);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct STATUSTEXT_DATA {
    pub severity: MavSeverity,
    pub text: Vec<char>, /* 50 elements */
}
impl STATUSTEXT_DATA {
    pub const ENCODED_LEN: usize = 51usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < STATUSTEXT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; STATUSTEXT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.severity = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavSeverity".to_string(),
            value: tmp as u32,
        })?;
        for _ in 0..50usize {
            let val = buf.get_u8() as char;
            _struct.text.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.severity as u8);
        for val in &self.text {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEBUG_DATA {
    pub time_boot_ms: u32,
    pub value: f32,
    pub ind: u8,
}
impl DEBUG_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEBUG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEBUG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.value = buf.get_f32_le();
        _struct.ind = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.value);
        _tmp.put_u8(self.ind);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SETUP_SIGNING_DATA {
    pub initial_timestamp: u64,
    pub target_system: u8,
    pub target_component: u8,
    pub secret_key: [u8; 32],
}
impl SETUP_SIGNING_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SETUP_SIGNING_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SETUP_SIGNING_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.initial_timestamp = buf.get_u64_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.secret_key[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.initial_timestamp);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.secret_key {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BUTTON_CHANGE_DATA {
    pub time_boot_ms: u32,
    pub last_change_ms: u32,
    pub state: u8,
}
impl BUTTON_CHANGE_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < BUTTON_CHANGE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; BUTTON_CHANGE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.last_change_ms = buf.get_u32_le();
        _struct.state = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.last_change_ms);
        _tmp.put_u8(self.state);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PLAY_TUNE_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub tune: [char; 30],
}
impl PLAY_TUNE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PLAY_TUNE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PLAY_TUNE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..30usize {
            let val = buf.get_u8() as char;
            _struct.tune[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.tune {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_INFORMATION_DATA {
    pub time_boot_ms: u32,
    pub firmware_version: u32,
    pub focal_length: f32,
    pub sensor_size_h: f32,
    pub sensor_size_v: f32,
    pub flags: CameraCapFlags,
    pub resolution_h: u16,
    pub resolution_v: u16,
    pub cam_definition_version: u16,
    pub vendor_name: [u8; 32],
    pub model_name: [u8; 32],
    pub lens_id: u8,
    pub cam_definition_uri: Vec<char>, /* 140 elements */
}
impl CAMERA_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 235usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.firmware_version = buf.get_u32_le();
        _struct.focal_length = buf.get_f32_le();
        _struct.sensor_size_h = buf.get_f32_le();
        _struct.sensor_size_v = buf.get_f32_le();
        let tmp = buf.get_u32_le();
        _struct.flags = CameraCapFlags::from_bits(tmp & CameraCapFlags::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "CameraCapFlags".to_string(),
                value: tmp as u32,
            },
        )?;
        _struct.resolution_h = buf.get_u16_le();
        _struct.resolution_v = buf.get_u16_le();
        _struct.cam_definition_version = buf.get_u16_le();
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.vendor_name[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.model_name[idx] = val;
        }
        _struct.lens_id = buf.get_u8();
        for _ in 0..140usize {
            let val = buf.get_u8() as char;
            _struct.cam_definition_uri.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.firmware_version);
        _tmp.put_f32_le(self.focal_length);
        _tmp.put_f32_le(self.sensor_size_h);
        _tmp.put_f32_le(self.sensor_size_v);
        _tmp.put_u32_le(self.flags.bits());
        _tmp.put_u16_le(self.resolution_h);
        _tmp.put_u16_le(self.resolution_v);
        _tmp.put_u16_le(self.cam_definition_version);
        for val in &self.vendor_name {
            _tmp.put_u8(*val);
        }
        for val in &self.model_name {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.lens_id);
        for val in &self.cam_definition_uri {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_SETTINGS_DATA {
    pub time_boot_ms: u32,
    pub mode_id: CameraMode,
}
impl CAMERA_SETTINGS_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_SETTINGS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_SETTINGS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        let tmp = buf.get_u8();
        _struct.mode_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CameraMode".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u8(self.mode_id as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct STORAGE_INFORMATION_DATA {
    pub time_boot_ms: u32,
    pub total_capacity: f32,
    pub used_capacity: f32,
    pub available_capacity: f32,
    pub read_speed: f32,
    pub write_speed: f32,
    pub storage_id: u8,
    pub storage_count: u8,
    pub status: StorageStatus,
}
impl STORAGE_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 27usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < STORAGE_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; STORAGE_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.total_capacity = buf.get_f32_le();
        _struct.used_capacity = buf.get_f32_le();
        _struct.available_capacity = buf.get_f32_le();
        _struct.read_speed = buf.get_f32_le();
        _struct.write_speed = buf.get_f32_le();
        _struct.storage_id = buf.get_u8();
        _struct.storage_count = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "StorageStatus".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.total_capacity);
        _tmp.put_f32_le(self.used_capacity);
        _tmp.put_f32_le(self.available_capacity);
        _tmp.put_f32_le(self.read_speed);
        _tmp.put_f32_le(self.write_speed);
        _tmp.put_u8(self.storage_id);
        _tmp.put_u8(self.storage_count);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_CAPTURE_STATUS_DATA {
    pub time_boot_ms: u32,
    pub image_interval: f32,
    pub recording_time_ms: u32,
    pub available_capacity: f32,
    pub image_status: u8,
    pub video_status: u8,
}
impl CAMERA_CAPTURE_STATUS_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_CAPTURE_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_CAPTURE_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.image_interval = buf.get_f32_le();
        _struct.recording_time_ms = buf.get_u32_le();
        _struct.available_capacity = buf.get_f32_le();
        _struct.image_status = buf.get_u8();
        _struct.video_status = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.image_interval);
        _tmp.put_u32_le(self.recording_time_ms);
        _tmp.put_f32_le(self.available_capacity);
        _tmp.put_u8(self.image_status);
        _tmp.put_u8(self.video_status);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_IMAGE_CAPTURED_DATA {
    pub time_utc: u64,
    pub time_boot_ms: u32,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub q: [f32; 4],
    pub image_index: i32,
    pub camera_id: u8,
    pub capture_result: i8,
    pub file_url: Vec<char>, /* 205 elements */
}
impl CAMERA_IMAGE_CAPTURED_DATA {
    pub const ENCODED_LEN: usize = 255usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_IMAGE_CAPTURED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_IMAGE_CAPTURED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_utc = buf.get_u64_le();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.relative_alt = buf.get_i32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.image_index = buf.get_i32_le();
        _struct.camera_id = buf.get_u8();
        _struct.capture_result = buf.get_i8();
        for _ in 0..205usize {
            let val = buf.get_u8() as char;
            _struct.file_url.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_utc);
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i32_le(self.relative_alt);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_i32_le(self.image_index);
        _tmp.put_u8(self.camera_id);
        _tmp.put_i8(self.capture_result);
        for val in &self.file_url {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FLIGHT_INFORMATION_DATA {
    pub arming_time_utc: u64,
    pub takeoff_time_utc: u64,
    pub flight_uuid: u64,
    pub time_boot_ms: u32,
}
impl FLIGHT_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FLIGHT_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FLIGHT_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.arming_time_utc = buf.get_u64_le();
        _struct.takeoff_time_utc = buf.get_u64_le();
        _struct.flight_uuid = buf.get_u64_le();
        _struct.time_boot_ms = buf.get_u32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.arming_time_utc);
        _tmp.put_u64_le(self.takeoff_time_utc);
        _tmp.put_u64_le(self.flight_uuid);
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MOUNT_ORIENTATION_DATA {
    pub time_boot_ms: u32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}
impl MOUNT_ORIENTATION_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MOUNT_ORIENTATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MOUNT_ORIENTATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOGGING_DATA_DATA {
    pub sequence: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub length: u8,
    pub first_message_offset: u8,
    pub data: Vec<u8>, /* 249 elements */
}
impl LOGGING_DATA_DATA {
    pub const ENCODED_LEN: usize = 255usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOGGING_DATA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOGGING_DATA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sequence = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.length = buf.get_u8();
        _struct.first_message_offset = buf.get_u8();
        for _ in 0..249usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.length);
        _tmp.put_u8(self.first_message_offset);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOGGING_DATA_ACKED_DATA {
    pub sequence: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub length: u8,
    pub first_message_offset: u8,
    pub data: Vec<u8>, /* 249 elements */
}
impl LOGGING_DATA_ACKED_DATA {
    pub const ENCODED_LEN: usize = 255usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOGGING_DATA_ACKED_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOGGING_DATA_ACKED_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sequence = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.length = buf.get_u8();
        _struct.first_message_offset = buf.get_u8();
        for _ in 0..249usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.length);
        _tmp.put_u8(self.first_message_offset);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LOGGING_ACK_DATA {
    pub sequence: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl LOGGING_ACK_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LOGGING_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LOGGING_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.sequence = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.sequence);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VIDEO_STREAM_INFORMATION_DATA {
    pub framerate: f32,
    pub bitrate: u32,
    pub flags: VideoStreamStatusFlags,
    pub resolution_h: u16,
    pub resolution_v: u16,
    pub rotation: u16,
    pub hfov: u16,
    pub stream_id: u8,
    pub count: u8,
    pub mavtype: VideoStreamType,
    pub name: [char; 32],
    pub uri: Vec<char>, /* 160 elements */
}
impl VIDEO_STREAM_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 213usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VIDEO_STREAM_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VIDEO_STREAM_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.framerate = buf.get_f32_le();
        _struct.bitrate = buf.get_u32_le();
        let tmp = buf.get_u16_le();
        _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "VideoStreamStatusFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.resolution_h = buf.get_u16_le();
        _struct.resolution_v = buf.get_u16_le();
        _struct.rotation = buf.get_u16_le();
        _struct.hfov = buf.get_u16_le();
        _struct.stream_id = buf.get_u8();
        _struct.count = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "VideoStreamType".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        for _ in 0..160usize {
            let val = buf.get_u8() as char;
            _struct.uri.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.framerate);
        _tmp.put_u32_le(self.bitrate);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u16_le(self.resolution_h);
        _tmp.put_u16_le(self.resolution_v);
        _tmp.put_u16_le(self.rotation);
        _tmp.put_u16_le(self.hfov);
        _tmp.put_u8(self.stream_id);
        _tmp.put_u8(self.count);
        _tmp.put_u8(self.mavtype as u8);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.uri {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VIDEO_STREAM_STATUS_DATA {
    pub framerate: f32,
    pub bitrate: u32,
    pub flags: VideoStreamStatusFlags,
    pub resolution_h: u16,
    pub resolution_v: u16,
    pub rotation: u16,
    pub hfov: u16,
    pub stream_id: u8,
}
impl VIDEO_STREAM_STATUS_DATA {
    pub const ENCODED_LEN: usize = 19usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VIDEO_STREAM_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VIDEO_STREAM_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.framerate = buf.get_f32_le();
        _struct.bitrate = buf.get_u32_le();
        let tmp = buf.get_u16_le();
        _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "VideoStreamStatusFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.resolution_h = buf.get_u16_le();
        _struct.resolution_v = buf.get_u16_le();
        _struct.rotation = buf.get_u16_le();
        _struct.hfov = buf.get_u16_le();
        _struct.stream_id = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.framerate);
        _tmp.put_u32_le(self.bitrate);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u16_le(self.resolution_h);
        _tmp.put_u16_le(self.resolution_v);
        _tmp.put_u16_le(self.rotation);
        _tmp.put_u16_le(self.hfov);
        _tmp.put_u8(self.stream_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_MANAGER_INFORMATION_DATA {
    pub time_boot_ms: u32,
    pub cap_flags: GimbalManagerCapFlags,
    pub tilt_max: f32,
    pub tilt_min: f32,
    pub tilt_rate_max: f32,
    pub pan_max: f32,
    pub pan_min: f32,
    pub pan_rate_max: f32,
    pub gimbal_device_id: u8,
}
impl GIMBAL_MANAGER_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 33usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_MANAGER_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_MANAGER_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        let tmp = buf.get_u32_le();
        _struct.cap_flags = GimbalManagerCapFlags::from_bits(
            tmp & GimbalManagerCapFlags::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "GimbalManagerCapFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.tilt_max = buf.get_f32_le();
        _struct.tilt_min = buf.get_f32_le();
        _struct.tilt_rate_max = buf.get_f32_le();
        _struct.pan_max = buf.get_f32_le();
        _struct.pan_min = buf.get_f32_le();
        _struct.pan_rate_max = buf.get_f32_le();
        _struct.gimbal_device_id = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.cap_flags.bits());
        _tmp.put_f32_le(self.tilt_max);
        _tmp.put_f32_le(self.tilt_min);
        _tmp.put_f32_le(self.tilt_rate_max);
        _tmp.put_f32_le(self.pan_max);
        _tmp.put_f32_le(self.pan_min);
        _tmp.put_f32_le(self.pan_rate_max);
        _tmp.put_u8(self.gimbal_device_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_MANAGER_STATUS_DATA {
    pub time_boot_ms: u32,
    pub flags: GimbalManagerFlags,
    pub gimbal_device_id: u8,
}
impl GIMBAL_MANAGER_STATUS_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_MANAGER_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_MANAGER_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        let tmp = buf.get_u32_le();
        _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GimbalManagerFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.gimbal_device_id = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.flags as u32);
        _tmp.put_u8(self.gimbal_device_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_MANAGER_SET_ATTITUDE_DATA {
    pub flags: GimbalManagerFlags,
    pub q: [f32; 4],
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub gimbal_device_id: u8,
}
impl GIMBAL_MANAGER_SET_ATTITUDE_DATA {
    pub const ENCODED_LEN: usize = 35usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_MANAGER_SET_ATTITUDE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_MANAGER_SET_ATTITUDE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GimbalManagerFlags".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.angular_velocity_x = buf.get_f32_le();
        _struct.angular_velocity_y = buf.get_f32_le();
        _struct.angular_velocity_z = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.gimbal_device_id = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.flags as u32);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.angular_velocity_x);
        _tmp.put_f32_le(self.angular_velocity_y);
        _tmp.put_f32_le(self.angular_velocity_z);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.gimbal_device_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_DEVICE_INFORMATION_DATA {
    pub time_boot_ms: u32,
    pub firmware_version: u32,
    pub tilt_max: f32,
    pub tilt_min: f32,
    pub tilt_rate_max: f32,
    pub pan_max: f32,
    pub pan_min: f32,
    pub pan_rate_max: f32,
    pub cap_flags: GimbalDeviceCapFlags,
    pub vendor_name: [u8; 32],
    pub model_name: [u8; 32],
}
impl GIMBAL_DEVICE_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 98usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_DEVICE_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_DEVICE_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        _struct.firmware_version = buf.get_u32_le();
        _struct.tilt_max = buf.get_f32_le();
        _struct.tilt_min = buf.get_f32_le();
        _struct.tilt_rate_max = buf.get_f32_le();
        _struct.pan_max = buf.get_f32_le();
        _struct.pan_min = buf.get_f32_le();
        _struct.pan_rate_max = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.cap_flags = GimbalDeviceCapFlags::from_bits(
            tmp & GimbalDeviceCapFlags::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "GimbalDeviceCapFlags".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.vendor_name[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.model_name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.firmware_version);
        _tmp.put_f32_le(self.tilt_max);
        _tmp.put_f32_le(self.tilt_min);
        _tmp.put_f32_le(self.tilt_rate_max);
        _tmp.put_f32_le(self.pan_max);
        _tmp.put_f32_le(self.pan_min);
        _tmp.put_f32_le(self.pan_rate_max);
        _tmp.put_u16_le(self.cap_flags.bits());
        for val in &self.vendor_name {
            _tmp.put_u8(*val);
        }
        for val in &self.model_name {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_DEVICE_SET_ATTITUDE_DATA {
    pub q: [f32; 4],
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub flags: GimbalDeviceFlags,
    pub target_system: u8,
    pub target_component: u8,
}
impl GIMBAL_DEVICE_SET_ATTITUDE_DATA {
    pub const ENCODED_LEN: usize = 32usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_DEVICE_SET_ATTITUDE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_DEVICE_SET_ATTITUDE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.angular_velocity_x = buf.get_f32_le();
        _struct.angular_velocity_y = buf.get_f32_le();
        _struct.angular_velocity_z = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.flags = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GimbalDeviceFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.angular_velocity_x);
        _tmp.put_f32_le(self.angular_velocity_y);
        _tmp.put_f32_le(self.angular_velocity_z);
        _tmp.put_u16_le(self.flags as u16);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_DEVICE_ATTITUDE_STATUS_DATA {
    pub time_boot_ms: u32,
    pub q: [f32; 4],
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,
    pub failure_flags: u32,
    pub flags: u16,
    pub target_system: u8,
    pub target_component: u8,
}
impl GIMBAL_DEVICE_ATTITUDE_STATUS_DATA {
    pub const ENCODED_LEN: usize = 40usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_DEVICE_ATTITUDE_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_DEVICE_ATTITUDE_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.angular_velocity_x = buf.get_f32_le();
        _struct.angular_velocity_y = buf.get_f32_le();
        _struct.angular_velocity_z = buf.get_f32_le();
        _struct.failure_flags = buf.get_u32_le();
        _struct.flags = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.angular_velocity_x);
        _tmp.put_f32_le(self.angular_velocity_y);
        _tmp.put_f32_le(self.angular_velocity_z);
        _tmp.put_u32_le(self.failure_flags);
        _tmp.put_u16_le(self.flags);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA {
    pub time_boot_us: u64,
    pub q: [f32; 4],
    pub q_estimated_delay_us: u32,
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub v_estimated_delay_us: u32,
    pub feed_forward_angular_velocity_z: f32,
    pub estimator_status: EstimatorStatusFlags,
    pub target_system: u8,
    pub target_component: u8,
    pub landed_state: MavLandedState,
}
impl AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_us = buf.get_u64_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.q_estimated_delay_us = buf.get_u32_le();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.v_estimated_delay_us = buf.get_u32_le();
        _struct.feed_forward_angular_velocity_z = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.estimator_status = EstimatorStatusFlags::from_bits(
            tmp & EstimatorStatusFlags::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "EstimatorStatusFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.landed_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavLandedState".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_boot_us);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u32_le(self.q_estimated_delay_us);
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_u32_le(self.v_estimated_delay_us);
        _tmp.put_f32_le(self.feed_forward_angular_velocity_z);
        _tmp.put_u16_le(self.estimator_status.bits());
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.landed_state as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_MANAGER_SET_TILTPAN_DATA {
    pub flags: GimbalManagerFlags,
    pub tilt: f32,
    pub pan: f32,
    pub tilt_rate: f32,
    pub pan_rate: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub gimbal_device_id: u8,
}
impl GIMBAL_MANAGER_SET_TILTPAN_DATA {
    pub const ENCODED_LEN: usize = 23usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_MANAGER_SET_TILTPAN_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_MANAGER_SET_TILTPAN_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.flags = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GimbalManagerFlags".to_string(),
            value: tmp as u32,
        })?;
        _struct.tilt = buf.get_f32_le();
        _struct.pan = buf.get_f32_le();
        _struct.tilt_rate = buf.get_f32_le();
        _struct.pan_rate = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.gimbal_device_id = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.flags as u32);
        _tmp.put_f32_le(self.tilt);
        _tmp.put_f32_le(self.pan);
        _tmp.put_f32_le(self.tilt_rate);
        _tmp.put_f32_le(self.pan_rate);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.gimbal_device_id);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WIFI_CONFIG_AP_DATA {
    pub ssid: [char; 32],
    pub password: Vec<char>, /* 64 elements */
}
impl WIFI_CONFIG_AP_DATA {
    pub const ENCODED_LEN: usize = 96usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < WIFI_CONFIG_AP_DATA::ENCODED_LEN {
            let mut payload_buf = [0; WIFI_CONFIG_AP_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.ssid[idx] = val;
        }
        for _ in 0..64usize {
            let val = buf.get_u8() as char;
            _struct.password.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.ssid {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.password {
            _tmp.put_u8(*val as u8);
        }
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
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AIS_VESSEL_DATA {
    pub MMSI: u32,
    pub lat: i32,
    pub lon: i32,
    pub COG: u16,
    pub heading: u16,
    pub velocity: u16,
    pub dimension_bow: u16,
    pub dimension_stern: u16,
    pub tslc: u16,
    pub flags: AisFlags,
    pub turn_rate: i8,
    pub navigational_status: AisNavStatus,
    pub mavtype: AisType,
    pub dimension_port: u8,
    pub dimension_starboard: u8,
    pub callsign: [char; 7],
    pub name: [char; 20],
}
impl AIS_VESSEL_DATA {
    pub const ENCODED_LEN: usize = 58usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AIS_VESSEL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AIS_VESSEL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.MMSI = buf.get_u32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.COG = buf.get_u16_le();
        _struct.heading = buf.get_u16_le();
        _struct.velocity = buf.get_u16_le();
        _struct.dimension_bow = buf.get_u16_le();
        _struct.dimension_stern = buf.get_u16_le();
        _struct.tslc = buf.get_u16_le();
        let tmp = buf.get_u16_le();
        _struct.flags =
            AisFlags::from_bits(tmp & AisFlags::all().bits()).ok_or(ParserError::InvalidFlag {
                flag_type: "AisFlags".to_string(),
                value: tmp as u32,
            })?;
        _struct.turn_rate = buf.get_i8();
        let tmp = buf.get_u8();
        _struct.navigational_status =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "AisNavStatus".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "AisType".to_string(),
            value: tmp as u32,
        })?;
        _struct.dimension_port = buf.get_u8();
        _struct.dimension_starboard = buf.get_u8();
        for idx in 0..7usize {
            let val = buf.get_u8() as char;
            _struct.callsign[idx] = val;
        }
        for idx in 0..20usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.MMSI);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_u16_le(self.COG);
        _tmp.put_u16_le(self.heading);
        _tmp.put_u16_le(self.velocity);
        _tmp.put_u16_le(self.dimension_bow);
        _tmp.put_u16_le(self.dimension_stern);
        _tmp.put_u16_le(self.tslc);
        _tmp.put_u16_le(self.flags.bits());
        _tmp.put_i8(self.turn_rate);
        _tmp.put_u8(self.navigational_status as u8);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.dimension_port);
        _tmp.put_u8(self.dimension_starboard);
        for val in &self.callsign {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAVCAN_NODE_STATUS_DATA {
    pub time_usec: u64,
    pub uptime_sec: u32,
    pub vendor_specific_status_code: u16,
    pub health: UavcanNodeHealth,
    pub mode: UavcanNodeMode,
    pub sub_mode: u8,
}
impl UAVCAN_NODE_STATUS_DATA {
    pub const ENCODED_LEN: usize = 17usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAVCAN_NODE_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAVCAN_NODE_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.uptime_sec = buf.get_u32_le();
        _struct.vendor_specific_status_code = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.health = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavcanNodeHealth".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UavcanNodeMode".to_string(),
            value: tmp as u32,
        })?;
        _struct.sub_mode = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.uptime_sec);
        _tmp.put_u16_le(self.vendor_specific_status_code);
        _tmp.put_u8(self.health as u8);
        _tmp.put_u8(self.mode as u8);
        _tmp.put_u8(self.sub_mode);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UAVCAN_NODE_INFO_DATA {
    pub time_usec: u64,
    pub uptime_sec: u32,
    pub sw_vcs_commit: u32,
    pub name: Vec<char>, /* 80 elements */
    pub hw_version_major: u8,
    pub hw_version_minor: u8,
    pub hw_unique_id: [u8; 16],
    pub sw_version_major: u8,
    pub sw_version_minor: u8,
}
impl UAVCAN_NODE_INFO_DATA {
    pub const ENCODED_LEN: usize = 116usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UAVCAN_NODE_INFO_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UAVCAN_NODE_INFO_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.uptime_sec = buf.get_u32_le();
        _struct.sw_vcs_commit = buf.get_u32_le();
        for _ in 0..80usize {
            let val = buf.get_u8() as char;
            _struct.name.push(val);
        }
        _struct.hw_version_major = buf.get_u8();
        _struct.hw_version_minor = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8();
            _struct.hw_unique_id[idx] = val;
        }
        _struct.sw_version_major = buf.get_u8();
        _struct.sw_version_minor = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.uptime_sec);
        _tmp.put_u32_le(self.sw_vcs_commit);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.hw_version_major);
        _tmp.put_u8(self.hw_version_minor);
        for val in &self.hw_unique_id {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.sw_version_major);
        _tmp.put_u8(self.sw_version_minor);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_EXT_REQUEST_READ_DATA {
    pub param_index: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: [char; 16],
}
impl PARAM_EXT_REQUEST_READ_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_EXT_REQUEST_READ_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_EXT_REQUEST_READ_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_index = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.param_index);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_EXT_REQUEST_LIST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl PARAM_EXT_REQUEST_LIST_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_EXT_REQUEST_LIST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_EXT_REQUEST_LIST_DATA::ENCODED_LEN];
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
pub struct PARAM_EXT_VALUE_DATA {
    pub param_count: u16,
    pub param_index: u16,
    pub param_id: [char; 16],
    pub param_value: Vec<char>, /* 128 elements */
    pub param_type: MavParamExtType,
}
impl PARAM_EXT_VALUE_DATA {
    pub const ENCODED_LEN: usize = 149usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_EXT_VALUE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_EXT_VALUE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.param_count = buf.get_u16_le();
        _struct.param_index = buf.get_u16_le();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        for _ in 0..128usize {
            let val = buf.get_u8() as char;
            _struct.param_value.push(val);
        }
        let tmp = buf.get_u8();
        _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavParamExtType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.param_count);
        _tmp.put_u16_le(self.param_index);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.param_value {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_EXT_SET_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub param_id: [char; 16],
    pub param_value: Vec<char>, /* 128 elements */
    pub param_type: MavParamExtType,
}
impl PARAM_EXT_SET_DATA {
    pub const ENCODED_LEN: usize = 147usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_EXT_SET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_EXT_SET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        for _ in 0..128usize {
            let val = buf.get_u8() as char;
            _struct.param_value.push(val);
        }
        let tmp = buf.get_u8();
        _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavParamExtType".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.param_value {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PARAM_EXT_ACK_DATA {
    pub param_id: [char; 16],
    pub param_value: Vec<char>, /* 128 elements */
    pub param_type: MavParamExtType,
    pub param_result: ParamAck,
}
impl PARAM_EXT_ACK_DATA {
    pub const ENCODED_LEN: usize = 146usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PARAM_EXT_ACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PARAM_EXT_ACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..16usize {
            let val = buf.get_u8() as char;
            _struct.param_id[idx] = val;
        }
        for _ in 0..128usize {
            let val = buf.get_u8() as char;
            _struct.param_value.push(val);
        }
        let tmp = buf.get_u8();
        _struct.param_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavParamExtType".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.param_result = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "ParamAck".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.param_id {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.param_value {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.param_type as u8);
        _tmp.put_u8(self.param_result as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OBSTACLE_DISTANCE_DATA {
    pub time_usec: u64,
    pub distances: Vec<u16>, /* 72 elements */
    pub min_distance: u16,
    pub max_distance: u16,
    pub sensor_type: MavDistanceSensor,
    pub increment: u8,
}
impl OBSTACLE_DISTANCE_DATA {
    pub const ENCODED_LEN: usize = 158usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OBSTACLE_DISTANCE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OBSTACLE_DISTANCE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for _ in 0..72usize {
            let val = buf.get_u16_le();
            _struct.distances.push(val);
        }
        _struct.min_distance = buf.get_u16_le();
        _struct.max_distance = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.sensor_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavDistanceSensor".to_string(),
            value: tmp as u32,
        })?;
        _struct.increment = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.distances {
            _tmp.put_u16_le(*val);
        }
        _tmp.put_u16_le(self.min_distance);
        _tmp.put_u16_le(self.max_distance);
        _tmp.put_u8(self.sensor_type as u8);
        _tmp.put_u8(self.increment);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ODOMETRY_DATA {
    pub time_usec: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: [f32; 4],
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub rollspeed: f32,
    pub pitchspeed: f32,
    pub yawspeed: f32,
    pub pose_covariance: [f32; 21],
    pub velocity_covariance: [f32; 21],
    pub frame_id: MavFrame,
    pub child_frame_id: MavFrame,
}
impl ODOMETRY_DATA {
    pub const ENCODED_LEN: usize = 230usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ODOMETRY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ODOMETRY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.x = buf.get_f32_le();
        _struct.y = buf.get_f32_le();
        _struct.z = buf.get_f32_le();
        for idx in 0..4usize {
            let val = buf.get_f32_le();
            _struct.q[idx] = val;
        }
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.rollspeed = buf.get_f32_le();
        _struct.pitchspeed = buf.get_f32_le();
        _struct.yawspeed = buf.get_f32_le();
        for idx in 0..21usize {
            let val = buf.get_f32_le();
            _struct.pose_covariance[idx] = val;
        }
        for idx in 0..21usize {
            let val = buf.get_f32_le();
            _struct.velocity_covariance[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.frame_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.child_frame_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.x);
        _tmp.put_f32_le(self.y);
        _tmp.put_f32_le(self.z);
        for val in &self.q {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.rollspeed);
        _tmp.put_f32_le(self.pitchspeed);
        _tmp.put_f32_le(self.yawspeed);
        for val in &self.pose_covariance {
            _tmp.put_f32_le(*val);
        }
        for val in &self.velocity_covariance {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.frame_id as u8);
        _tmp.put_u8(self.child_frame_id as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA {
    pub time_usec: u64,
    pub pos_x: [f32; 5],
    pub pos_y: [f32; 5],
    pub pos_z: [f32; 5],
    pub vel_x: [f32; 5],
    pub vel_y: [f32; 5],
    pub vel_z: [f32; 5],
    pub acc_x: [f32; 5],
    pub acc_y: [f32; 5],
    pub acc_z: [f32; 5],
    pub pos_yaw: [f32; 5],
    pub vel_yaw: [f32; 5],
    pub command: [u16; 5],
    pub valid_points: u8,
}
impl TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA {
    pub const ENCODED_LEN: usize = 239usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_x[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_y[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_z[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.vel_x[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.vel_y[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.vel_z[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.acc_x[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.acc_y[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.acc_z[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_yaw[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.vel_yaw[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_u16_le();
            _struct.command[idx] = val;
        }
        _struct.valid_points = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.pos_x {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_y {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_z {
            _tmp.put_f32_le(*val);
        }
        for val in &self.vel_x {
            _tmp.put_f32_le(*val);
        }
        for val in &self.vel_y {
            _tmp.put_f32_le(*val);
        }
        for val in &self.vel_z {
            _tmp.put_f32_le(*val);
        }
        for val in &self.acc_x {
            _tmp.put_f32_le(*val);
        }
        for val in &self.acc_y {
            _tmp.put_f32_le(*val);
        }
        for val in &self.acc_z {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_yaw {
            _tmp.put_f32_le(*val);
        }
        for val in &self.vel_yaw {
            _tmp.put_f32_le(*val);
        }
        for val in &self.command {
            _tmp.put_u16_le(*val);
        }
        _tmp.put_u8(self.valid_points);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TRAJECTORY_REPRESENTATION_BEZIER_DATA {
    pub time_usec: u64,
    pub pos_x: [f32; 5],
    pub pos_y: [f32; 5],
    pub pos_z: [f32; 5],
    pub delta: [f32; 5],
    pub pos_yaw: [f32; 5],
    pub valid_points: u8,
}
impl TRAJECTORY_REPRESENTATION_BEZIER_DATA {
    pub const ENCODED_LEN: usize = 109usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TRAJECTORY_REPRESENTATION_BEZIER_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TRAJECTORY_REPRESENTATION_BEZIER_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_x[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_y[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_z[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.delta[idx] = val;
        }
        for idx in 0..5usize {
            let val = buf.get_f32_le();
            _struct.pos_yaw[idx] = val;
        }
        _struct.valid_points = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.pos_x {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_y {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_z {
            _tmp.put_f32_le(*val);
        }
        for val in &self.delta {
            _tmp.put_f32_le(*val);
        }
        for val in &self.pos_yaw {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_u8(self.valid_points);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CELLULAR_STATUS_DATA {
    pub mcc: u16,
    pub mnc: u16,
    pub lac: u16,
    pub status: CellularStatusFlag,
    pub failure_reason: CellularNetworkFailedReason,
    pub mavtype: CellularNetworkRadioType,
    pub quality: u8,
}
impl CELLULAR_STATUS_DATA {
    pub const ENCODED_LEN: usize = 10usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CELLULAR_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CELLULAR_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mcc = buf.get_u16_le();
        _struct.mnc = buf.get_u16_le();
        _struct.lac = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CellularStatusFlag".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.failure_reason = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CellularNetworkFailedReason".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.mavtype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CellularNetworkRadioType".to_string(),
            value: tmp as u32,
        })?;
        _struct.quality = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.mcc);
        _tmp.put_u16_le(self.mnc);
        _tmp.put_u16_le(self.lac);
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.failure_reason as u8);
        _tmp.put_u8(self.mavtype as u8);
        _tmp.put_u8(self.quality);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ISBD_LINK_STATUS_DATA {
    pub timestamp: u64,
    pub last_heartbeat: u64,
    pub failed_sessions: u16,
    pub successful_sessions: u16,
    pub signal_quality: u8,
    pub ring_pending: u8,
    pub tx_session_pending: u8,
    pub rx_session_pending: u8,
}
impl ISBD_LINK_STATUS_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ISBD_LINK_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ISBD_LINK_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u64_le();
        _struct.last_heartbeat = buf.get_u64_le();
        _struct.failed_sessions = buf.get_u16_le();
        _struct.successful_sessions = buf.get_u16_le();
        _struct.signal_quality = buf.get_u8();
        _struct.ring_pending = buf.get_u8();
        _struct.tx_session_pending = buf.get_u8();
        _struct.rx_session_pending = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.timestamp);
        _tmp.put_u64_le(self.last_heartbeat);
        _tmp.put_u16_le(self.failed_sessions);
        _tmp.put_u16_le(self.successful_sessions);
        _tmp.put_u8(self.signal_quality);
        _tmp.put_u8(self.ring_pending);
        _tmp.put_u8(self.tx_session_pending);
        _tmp.put_u8(self.rx_session_pending);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CELLULAR_CONFIG_DATA {
    pub enable_pin: u8,
    pub pin: [char; 32],
    pub apn: [char; 32],
    pub puk: [char; 32],
    pub roaming: u8,
    pub response: CellularConfigResponse,
}
impl CELLULAR_CONFIG_DATA {
    pub const ENCODED_LEN: usize = 99usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CELLULAR_CONFIG_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CELLULAR_CONFIG_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.enable_pin = buf.get_u8();
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.pin[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.apn[idx] = val;
        }
        for idx in 0..32usize {
            let val = buf.get_u8() as char;
            _struct.puk[idx] = val;
        }
        _struct.roaming = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.response = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CellularConfigResponse".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.enable_pin);
        for val in &self.pin {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.apn {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.puk {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.roaming);
        _tmp.put_u8(self.response as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RAW_RPM_DATA {
    pub frequency: f32,
    pub index: u8,
}
impl RAW_RPM_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RAW_RPM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RAW_RPM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.frequency = buf.get_f32_le();
        _struct.index = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.frequency);
        _tmp.put_u8(self.index);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UTM_GLOBAL_POSITION_DATA {
    pub time: u64,
    pub lat: i32,
    pub lon: i32,
    pub alt: i32,
    pub relative_alt: i32,
    pub next_lat: i32,
    pub next_lon: i32,
    pub next_alt: i32,
    pub vx: i16,
    pub vy: i16,
    pub vz: i16,
    pub h_acc: u16,
    pub v_acc: u16,
    pub vel_acc: u16,
    pub update_rate: u16,
    pub uas_id: [u8; 18],
    pub flight_state: UtmFlightState,
    pub flags: UtmDataAvailFlags,
}
impl UTM_GLOBAL_POSITION_DATA {
    pub const ENCODED_LEN: usize = 70usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < UTM_GLOBAL_POSITION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; UTM_GLOBAL_POSITION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lon = buf.get_i32_le();
        _struct.alt = buf.get_i32_le();
        _struct.relative_alt = buf.get_i32_le();
        _struct.next_lat = buf.get_i32_le();
        _struct.next_lon = buf.get_i32_le();
        _struct.next_alt = buf.get_i32_le();
        _struct.vx = buf.get_i16_le();
        _struct.vy = buf.get_i16_le();
        _struct.vz = buf.get_i16_le();
        _struct.h_acc = buf.get_u16_le();
        _struct.v_acc = buf.get_u16_le();
        _struct.vel_acc = buf.get_u16_le();
        _struct.update_rate = buf.get_u16_le();
        for idx in 0..18usize {
            let val = buf.get_u8();
            _struct.uas_id[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.flight_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "UtmFlightState".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.flags = UtmDataAvailFlags::from_bits(tmp & UtmDataAvailFlags::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "UtmDataAvailFlags".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lon);
        _tmp.put_i32_le(self.alt);
        _tmp.put_i32_le(self.relative_alt);
        _tmp.put_i32_le(self.next_lat);
        _tmp.put_i32_le(self.next_lon);
        _tmp.put_i32_le(self.next_alt);
        _tmp.put_i16_le(self.vx);
        _tmp.put_i16_le(self.vy);
        _tmp.put_i16_le(self.vz);
        _tmp.put_u16_le(self.h_acc);
        _tmp.put_u16_le(self.v_acc);
        _tmp.put_u16_le(self.vel_acc);
        _tmp.put_u16_le(self.update_rate);
        for val in &self.uas_id {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.flight_state as u8);
        _tmp.put_u8(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEBUG_FLOAT_ARRAY_DATA {
    pub time_usec: u64,
    pub array_id: u16,
    pub name: [char; 10],
}
impl DEBUG_FLOAT_ARRAY_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEBUG_FLOAT_ARRAY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEBUG_FLOAT_ARRAY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.array_id = buf.get_u16_le();
        for idx in 0..10usize {
            let val = buf.get_u8() as char;
            _struct.name[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u16_le(self.array_id);
        for val in &self.name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ORBIT_EXECUTION_STATUS_DATA {
    pub time_usec: u64,
    pub radius: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
    pub frame: MavFrame,
}
impl ORBIT_EXECUTION_STATUS_DATA {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ORBIT_EXECUTION_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ORBIT_EXECUTION_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.radius = buf.get_f32_le();
        _struct.x = buf.get_i32_le();
        _struct.y = buf.get_i32_le();
        _struct.z = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.frame = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavFrame".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.radius);
        _tmp.put_i32_le(self.x);
        _tmp.put_i32_le(self.y);
        _tmp.put_f32_le(self.z);
        _tmp.put_u8(self.frame as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMART_BATTERY_INFO_DATA {
    pub capacity_full_specification: i32,
    pub capacity_full: i32,
    pub serial_number: i32,
    pub cycle_count: u16,
    pub weight: u16,
    pub discharge_minimum_voltage: u16,
    pub charging_minimum_voltage: u16,
    pub resting_minimum_voltage: u16,
    pub id: u8,
    pub device_name: Vec<char>, /* 50 elements */
}
impl SMART_BATTERY_INFO_DATA {
    pub const ENCODED_LEN: usize = 73usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SMART_BATTERY_INFO_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SMART_BATTERY_INFO_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.capacity_full_specification = buf.get_i32_le();
        _struct.capacity_full = buf.get_i32_le();
        _struct.serial_number = buf.get_i32_le();
        _struct.cycle_count = buf.get_u16_le();
        _struct.weight = buf.get_u16_le();
        _struct.discharge_minimum_voltage = buf.get_u16_le();
        _struct.charging_minimum_voltage = buf.get_u16_le();
        _struct.resting_minimum_voltage = buf.get_u16_le();
        _struct.id = buf.get_u8();
        for _ in 0..50usize {
            let val = buf.get_u8() as char;
            _struct.device_name.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.capacity_full_specification);
        _tmp.put_i32_le(self.capacity_full);
        _tmp.put_i32_le(self.serial_number);
        _tmp.put_u16_le(self.cycle_count);
        _tmp.put_u16_le(self.weight);
        _tmp.put_u16_le(self.discharge_minimum_voltage);
        _tmp.put_u16_le(self.charging_minimum_voltage);
        _tmp.put_u16_le(self.resting_minimum_voltage);
        _tmp.put_u8(self.id);
        for val in &self.device_name {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SMART_BATTERY_STATUS_DATA {
    pub fault_bitmask: MavSmartBatteryFault,
    pub time_remaining: i32,
    pub id: u16,
    pub capacity_remaining: i16,
    pub current: i16,
    pub temperature: i16,
    pub cell_offset: u16,
    pub voltages: [u16; 16],
}
impl SMART_BATTERY_STATUS_DATA {
    pub const ENCODED_LEN: usize = 50usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SMART_BATTERY_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SMART_BATTERY_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_i32_le();
        _struct.fault_bitmask = MavSmartBatteryFault::from_bits(
            tmp & MavSmartBatteryFault::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavSmartBatteryFault".to_string(),
            value: tmp as u32,
        })?;
        _struct.time_remaining = buf.get_i32_le();
        _struct.id = buf.get_u16_le();
        _struct.capacity_remaining = buf.get_i16_le();
        _struct.current = buf.get_i16_le();
        _struct.temperature = buf.get_i16_le();
        _struct.cell_offset = buf.get_u16_le();
        for idx in 0..16usize {
            let val = buf.get_u16_le();
            _struct.voltages[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.fault_bitmask.bits());
        _tmp.put_i32_le(self.time_remaining);
        _tmp.put_u16_le(self.id);
        _tmp.put_i16_le(self.capacity_remaining);
        _tmp.put_i16_le(self.current);
        _tmp.put_i16_le(self.temperature);
        _tmp.put_u16_le(self.cell_offset);
        for val in &self.voltages {
            _tmp.put_u16_le(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GENERATOR_STATUS_DATA {
    pub status: MavGeneratorStatusFlag,
    pub battery_current: f32,
    pub load_current: f32,
    pub power_generated: f32,
    pub bus_voltage: f32,
    pub bat_current_setpoint: f32,
    pub generator_speed: u16,
    pub rectifier_temperature: i16,
    pub generator_temperature: i16,
}
impl GENERATOR_STATUS_DATA {
    pub const ENCODED_LEN: usize = 34usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GENERATOR_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GENERATOR_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u64_le();
        _struct.status = MavGeneratorStatusFlag::from_bits(
            tmp & MavGeneratorStatusFlag::all().bits(),
        )
        .ok_or(ParserError::InvalidFlag {
            flag_type: "MavGeneratorStatusFlag".to_string(),
            value: tmp as u32,
        })?;
        _struct.battery_current = buf.get_f32_le();
        _struct.load_current = buf.get_f32_le();
        _struct.power_generated = buf.get_f32_le();
        _struct.bus_voltage = buf.get_f32_le();
        _struct.bat_current_setpoint = buf.get_f32_le();
        _struct.generator_speed = buf.get_u16_le();
        _struct.rectifier_temperature = buf.get_i16_le();
        _struct.generator_temperature = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.status.bits());
        _tmp.put_f32_le(self.battery_current);
        _tmp.put_f32_le(self.load_current);
        _tmp.put_f32_le(self.power_generated);
        _tmp.put_f32_le(self.bus_voltage);
        _tmp.put_f32_le(self.bat_current_setpoint);
        _tmp.put_u16_le(self.generator_speed);
        _tmp.put_i16_le(self.rectifier_temperature);
        _tmp.put_i16_le(self.generator_temperature);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ACTUATOR_OUTPUT_STATUS_DATA {
    pub time_usec: u64,
    pub active: u32,
    pub actuator: [f32; 32],
}
impl ACTUATOR_OUTPUT_STATUS_DATA {
    pub const ENCODED_LEN: usize = 140usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ACTUATOR_OUTPUT_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ACTUATOR_OUTPUT_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.active = buf.get_u32_le();
        for idx in 0..32usize {
            let val = buf.get_f32_le();
            _struct.actuator[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.active);
        for val in &self.actuator {
            _tmp.put_f32_le(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TIME_ESTIMATE_TO_TARGET_DATA {
    pub safe_return: i32,
    pub land: i32,
    pub mission_next_item: i32,
    pub mission_end: i32,
    pub commanded_action: i32,
}
impl TIME_ESTIMATE_TO_TARGET_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TIME_ESTIMATE_TO_TARGET_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TIME_ESTIMATE_TO_TARGET_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.safe_return = buf.get_i32_le();
        _struct.land = buf.get_i32_le();
        _struct.mission_next_item = buf.get_i32_le();
        _struct.mission_end = buf.get_i32_le();
        _struct.commanded_action = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.safe_return);
        _tmp.put_i32_le(self.land);
        _tmp.put_i32_le(self.mission_next_item);
        _tmp.put_i32_le(self.mission_end);
        _tmp.put_i32_le(self.commanded_action);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TUNNEL_DATA {
    pub payload_type: MavTunnelPayloadType,
    pub target_system: u8,
    pub target_component: u8,
    pub payload_length: u8,
    pub payload: Vec<u8>, /* 128 elements */
}
impl TUNNEL_DATA {
    pub const ENCODED_LEN: usize = 133usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < TUNNEL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; TUNNEL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u16_le();
        _struct.payload_type = FromPrimitive::from_u16(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavTunnelPayloadType".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.payload_length = buf.get_u8();
        for _ in 0..128usize {
            let val = buf.get_u8();
            _struct.payload.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.payload_type as u16);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.payload_length);
        for val in &self.payload {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ONBOARD_COMPUTER_STATUS_DATA {
    pub time_usec: u64,
    pub uptime: u32,
    pub ram_usage: u32,
    pub ram_total: u32,
    pub storage_type: [u32; 4],
    pub storage_usage: [u32; 4],
    pub storage_total: [u32; 4],
    pub link_type: [u32; 6],
    pub link_tx_rate: [u32; 6],
    pub link_rx_rate: [u32; 6],
    pub link_tx_max: [u32; 6],
    pub link_rx_max: [u32; 6],
    pub fan_speed: [i16; 4],
    pub mavtype: u8,
    pub cpu_cores: [u8; 8],
    pub cpu_combined: [u8; 10],
    pub gpu_cores: [u8; 4],
    pub gpu_combined: [u8; 10],
    pub temperature_board: i8,
    pub temperature_core: [i8; 8],
}
impl ONBOARD_COMPUTER_STATUS_DATA {
    pub const ENCODED_LEN: usize = 238usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ONBOARD_COMPUTER_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ONBOARD_COMPUTER_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.uptime = buf.get_u32_le();
        _struct.ram_usage = buf.get_u32_le();
        _struct.ram_total = buf.get_u32_le();
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.storage_type[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.storage_usage[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u32_le();
            _struct.storage_total[idx] = val;
        }
        for idx in 0..6usize {
            let val = buf.get_u32_le();
            _struct.link_type[idx] = val;
        }
        for idx in 0..6usize {
            let val = buf.get_u32_le();
            _struct.link_tx_rate[idx] = val;
        }
        for idx in 0..6usize {
            let val = buf.get_u32_le();
            _struct.link_rx_rate[idx] = val;
        }
        for idx in 0..6usize {
            let val = buf.get_u32_le();
            _struct.link_tx_max[idx] = val;
        }
        for idx in 0..6usize {
            let val = buf.get_u32_le();
            _struct.link_rx_max[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_i16_le();
            _struct.fan_speed[idx] = val;
        }
        _struct.mavtype = buf.get_u8();
        for idx in 0..8usize {
            let val = buf.get_u8();
            _struct.cpu_cores[idx] = val;
        }
        for idx in 0..10usize {
            let val = buf.get_u8();
            _struct.cpu_combined[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.gpu_cores[idx] = val;
        }
        for idx in 0..10usize {
            let val = buf.get_u8();
            _struct.gpu_combined[idx] = val;
        }
        _struct.temperature_board = buf.get_i8();
        for idx in 0..8usize {
            let val = buf.get_i8();
            _struct.temperature_core[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u32_le(self.uptime);
        _tmp.put_u32_le(self.ram_usage);
        _tmp.put_u32_le(self.ram_total);
        for val in &self.storage_type {
            _tmp.put_u32_le(*val);
        }
        for val in &self.storage_usage {
            _tmp.put_u32_le(*val);
        }
        for val in &self.storage_total {
            _tmp.put_u32_le(*val);
        }
        for val in &self.link_type {
            _tmp.put_u32_le(*val);
        }
        for val in &self.link_tx_rate {
            _tmp.put_u32_le(*val);
        }
        for val in &self.link_rx_rate {
            _tmp.put_u32_le(*val);
        }
        for val in &self.link_tx_max {
            _tmp.put_u32_le(*val);
        }
        for val in &self.link_rx_max {
            _tmp.put_u32_le(*val);
        }
        for val in &self.fan_speed {
            _tmp.put_i16_le(*val);
        }
        _tmp.put_u8(self.mavtype);
        for val in &self.cpu_cores {
            _tmp.put_u8(*val);
        }
        for val in &self.cpu_combined {
            _tmp.put_u8(*val);
        }
        for val in &self.gpu_cores {
            _tmp.put_u8(*val);
        }
        for val in &self.gpu_combined {
            _tmp.put_u8(*val);
        }
        _tmp.put_i8(self.temperature_board);
        for val in &self.temperature_core {
            _tmp.put_i8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMPONENT_INFORMATION_DATA {
    pub time_boot_ms: u32,
    pub metadata_type: CompMetadataType,
    pub metadata_uid: u32,
    pub translation_uid: u32,
    pub metadata_uri: Vec<char>,    /* 70 elements */
    pub translation_uri: Vec<char>, /* 70 elements */
}
impl COMPONENT_INFORMATION_DATA {
    pub const ENCODED_LEN: usize = 156usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMPONENT_INFORMATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMPONENT_INFORMATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_boot_ms = buf.get_u32_le();
        let tmp = buf.get_u32_le();
        _struct.metadata_type = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CompMetadataType".to_string(),
            value: tmp as u32,
        })?;
        _struct.metadata_uid = buf.get_u32_le();
        _struct.translation_uid = buf.get_u32_le();
        for _ in 0..70usize {
            let val = buf.get_u8() as char;
            _struct.metadata_uri.push(val);
        }
        for _ in 0..70usize {
            let val = buf.get_u8() as char;
            _struct.translation_uri.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.time_boot_ms);
        _tmp.put_u32_le(self.metadata_type as u32);
        _tmp.put_u32_le(self.metadata_uid);
        _tmp.put_u32_le(self.translation_uid);
        for val in &self.metadata_uri {
            _tmp.put_u8(*val as u8);
        }
        for val in &self.translation_uri {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PLAY_TUNE_V2_DATA {
    pub format: TuneFormat,
    pub target_system: u8,
    pub target_component: u8,
    pub tune: Vec<char>, /* 248 elements */
}
impl PLAY_TUNE_V2_DATA {
    pub const ENCODED_LEN: usize = 254usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PLAY_TUNE_V2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PLAY_TUNE_V2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.format = TuneFormat::from_bits(tmp & TuneFormat::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "TuneFormat".to_string(),
                value: tmp as u32,
            },
        )?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..248usize {
            let val = buf.get_u8() as char;
            _struct.tune.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.format.bits());
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.tune {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SUPPORTED_TUNES_DATA {
    pub format: TuneFormat,
    pub target_system: u8,
    pub target_component: u8,
}
impl SUPPORTED_TUNES_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SUPPORTED_TUNES_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SUPPORTED_TUNES_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.format = TuneFormat::from_bits(tmp & TuneFormat::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "TuneFormat".to_string(),
                value: tmp as u32,
            },
        )?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.format.bits());
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WHEEL_DISTANCE_DATA {
    pub time_usec: u64,
    pub distance: [f64; 16],
    pub count: u8,
}
impl WHEEL_DISTANCE_DATA {
    pub const ENCODED_LEN: usize = 137usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < WHEEL_DISTANCE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; WHEEL_DISTANCE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        for idx in 0..16usize {
            let val = buf.get_f64_le();
            _struct.distance[idx] = val;
        }
        _struct.count = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        for val in &self.distance {
            _tmp.put_f64_le(*val);
        }
        _tmp.put_u8(self.count);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_BASIC_ID_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub id_type: MavOdidIdType,
    pub ua_type: MavOdidUaType,
    pub uas_id: [u8; 20],
}
impl OPEN_DRONE_ID_BASIC_ID_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_BASIC_ID_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_BASIC_ID_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.id_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidIdType".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.ua_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidUaType".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.uas_id[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.id_type as u8);
        _tmp.put_u8(self.ua_type as u8);
        for val in &self.uas_id {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_LOCATION_DATA {
    pub latitude: i32,
    pub longitude: i32,
    pub altitude_barometric: f32,
    pub altitude_geodetic: f32,
    pub height: f32,
    pub timestamp: f32,
    pub direction: u16,
    pub speed_horizontal: u16,
    pub speed_vertical: i16,
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub status: MavOdidStatus,
    pub height_reference: MavOdidHeightRef,
    pub horizontal_accuracy: MavOdidHorAcc,
    pub vertical_accuracy: MavOdidVerAcc,
    pub barometer_accuracy: MavOdidVerAcc,
    pub speed_accuracy: MavOdidSpeedAcc,
    pub timestamp_accuracy: MavOdidTimeAcc,
}
impl OPEN_DRONE_ID_LOCATION_DATA {
    pub const ENCODED_LEN: usize = 59usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_LOCATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_LOCATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.latitude = buf.get_i32_le();
        _struct.longitude = buf.get_i32_le();
        _struct.altitude_barometric = buf.get_f32_le();
        _struct.altitude_geodetic = buf.get_f32_le();
        _struct.height = buf.get_f32_le();
        _struct.timestamp = buf.get_f32_le();
        _struct.direction = buf.get_u16_le();
        _struct.speed_horizontal = buf.get_u16_le();
        _struct.speed_vertical = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidStatus".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.height_reference = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidHeightRef".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.horizontal_accuracy =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidHorAcc".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.vertical_accuracy =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidVerAcc".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.barometer_accuracy =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidVerAcc".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.speed_accuracy = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidSpeedAcc".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.timestamp_accuracy =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidTimeAcc".to_string(),
                value: tmp as u32,
            })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.latitude);
        _tmp.put_i32_le(self.longitude);
        _tmp.put_f32_le(self.altitude_barometric);
        _tmp.put_f32_le(self.altitude_geodetic);
        _tmp.put_f32_le(self.height);
        _tmp.put_f32_le(self.timestamp);
        _tmp.put_u16_le(self.direction);
        _tmp.put_u16_le(self.speed_horizontal);
        _tmp.put_i16_le(self.speed_vertical);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.height_reference as u8);
        _tmp.put_u8(self.horizontal_accuracy as u8);
        _tmp.put_u8(self.vertical_accuracy as u8);
        _tmp.put_u8(self.barometer_accuracy as u8);
        _tmp.put_u8(self.speed_accuracy as u8);
        _tmp.put_u8(self.timestamp_accuracy as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_AUTHENTICATION_DATA {
    pub timestamp: u32,
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub authentication_type: MavOdidAuthType,
    pub data_page: u8,
    pub page_count: u8,
    pub length: u8,
    pub authentication_data: [u8; 23],
}
impl OPEN_DRONE_ID_AUTHENTICATION_DATA {
    pub const ENCODED_LEN: usize = 53usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_AUTHENTICATION_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_AUTHENTICATION_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.timestamp = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.authentication_type =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidAuthType".to_string(),
                value: tmp as u32,
            })?;
        _struct.data_page = buf.get_u8();
        _struct.page_count = buf.get_u8();
        _struct.length = buf.get_u8();
        for idx in 0..23usize {
            let val = buf.get_u8();
            _struct.authentication_data[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.timestamp);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.authentication_type as u8);
        _tmp.put_u8(self.data_page);
        _tmp.put_u8(self.page_count);
        _tmp.put_u8(self.length);
        for val in &self.authentication_data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_SELF_ID_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub description_type: MavOdidDescType,
    pub description: [char; 23],
}
impl OPEN_DRONE_ID_SELF_ID_DATA {
    pub const ENCODED_LEN: usize = 46usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_SELF_ID_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_SELF_ID_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.description_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidDescType".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..23usize {
            let val = buf.get_u8() as char;
            _struct.description[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.description_type as u8);
        for val in &self.description {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_SYSTEM_DATA {
    pub operator_latitude: i32,
    pub operator_longitude: i32,
    pub area_ceiling: f32,
    pub area_floor: f32,
    pub area_count: u16,
    pub area_radius: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub operator_location_type: MavOdidOperatorLocationType,
    pub classification_type: MavOdidClassificationType,
    pub category_eu: MavOdidCategoryEu,
    pub class_eu: MavOdidClassEu,
}
impl OPEN_DRONE_ID_SYSTEM_DATA {
    pub const ENCODED_LEN: usize = 46usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_SYSTEM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_SYSTEM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.operator_latitude = buf.get_i32_le();
        _struct.operator_longitude = buf.get_i32_le();
        _struct.area_ceiling = buf.get_f32_le();
        _struct.area_floor = buf.get_f32_le();
        _struct.area_count = buf.get_u16_le();
        _struct.area_radius = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.operator_location_type =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidOperatorLocationType".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.classification_type =
            FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
                enum_type: "MavOdidClassificationType".to_string(),
                value: tmp as u32,
            })?;
        let tmp = buf.get_u8();
        _struct.category_eu = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidCategoryEu".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.class_eu = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidClassEu".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.operator_latitude);
        _tmp.put_i32_le(self.operator_longitude);
        _tmp.put_f32_le(self.area_ceiling);
        _tmp.put_f32_le(self.area_floor);
        _tmp.put_u16_le(self.area_count);
        _tmp.put_u16_le(self.area_radius);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.operator_location_type as u8);
        _tmp.put_u8(self.classification_type as u8);
        _tmp.put_u8(self.category_eu as u8);
        _tmp.put_u8(self.class_eu as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_OPERATOR_ID_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub id_or_mac: [u8; 20],
    pub operator_id_type: MavOdidOperatorIdType,
    pub operator_id: [char; 20],
}
impl OPEN_DRONE_ID_OPERATOR_ID_DATA {
    pub const ENCODED_LEN: usize = 43usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_OPERATOR_ID_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_OPERATOR_ID_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for idx in 0..20usize {
            let val = buf.get_u8();
            _struct.id_or_mac[idx] = val;
        }
        let tmp = buf.get_u8();
        _struct.operator_id_type = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavOdidOperatorIdType".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..20usize {
            let val = buf.get_u8() as char;
            _struct.operator_id[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.id_or_mac {
            _tmp.put_u8(*val);
        }
        _tmp.put_u8(self.operator_id_type as u8);
        for val in &self.operator_id {
            _tmp.put_u8(*val as u8);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OPEN_DRONE_ID_MESSAGE_PACK_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub single_message_size: u8,
    pub msg_pack_size: u8,
    pub messages: Vec<u8>, /* 250 elements */
}
impl OPEN_DRONE_ID_MESSAGE_PACK_DATA {
    pub const ENCODED_LEN: usize = 254usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < OPEN_DRONE_ID_MESSAGE_PACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; OPEN_DRONE_ID_MESSAGE_PACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.single_message_size = buf.get_u8();
        _struct.msg_pack_size = buf.get_u8();
        for _ in 0..250usize {
            let val = buf.get_u8();
            _struct.messages.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.single_message_size);
        _tmp.put_u8(self.msg_pack_size);
        for val in &self.messages {
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
    SYS_STATUS(SYS_STATUS_DATA),
    SYSTEM_TIME(SYSTEM_TIME_DATA),
    PING(PING_DATA),
    CHANGE_OPERATOR_CONTROL(CHANGE_OPERATOR_CONTROL_DATA),
    CHANGE_OPERATOR_CONTROL_ACK(CHANGE_OPERATOR_CONTROL_ACK_DATA),
    AUTH_KEY(AUTH_KEY_DATA),
    LINK_NODE_STATUS(LINK_NODE_STATUS_DATA),
    SET_MODE(SET_MODE_DATA),
    PARAM_REQUEST_READ(PARAM_REQUEST_READ_DATA),
    PARAM_REQUEST_LIST(PARAM_REQUEST_LIST_DATA),
    PARAM_VALUE(PARAM_VALUE_DATA),
    PARAM_SET(PARAM_SET_DATA),
    GPS_RAW_INT(GPS_RAW_INT_DATA),
    GPS_STATUS(GPS_STATUS_DATA),
    SCALED_IMU(SCALED_IMU_DATA),
    RAW_IMU(RAW_IMU_DATA),
    RAW_PRESSURE(RAW_PRESSURE_DATA),
    SCALED_PRESSURE(SCALED_PRESSURE_DATA),
    ATTITUDE(ATTITUDE_DATA),
    ATTITUDE_QUATERNION(ATTITUDE_QUATERNION_DATA),
    LOCAL_POSITION_NED(LOCAL_POSITION_NED_DATA),
    GLOBAL_POSITION_INT(GLOBAL_POSITION_INT_DATA),
    RC_CHANNELS_SCALED(RC_CHANNELS_SCALED_DATA),
    RC_CHANNELS_RAW(RC_CHANNELS_RAW_DATA),
    SERVO_OUTPUT_RAW(SERVO_OUTPUT_RAW_DATA),
    MISSION_REQUEST_PARTIAL_LIST(MISSION_REQUEST_PARTIAL_LIST_DATA),
    MISSION_WRITE_PARTIAL_LIST(MISSION_WRITE_PARTIAL_LIST_DATA),
    MISSION_ITEM(MISSION_ITEM_DATA),
    MISSION_REQUEST(MISSION_REQUEST_DATA),
    MISSION_SET_CURRENT(MISSION_SET_CURRENT_DATA),
    MISSION_CURRENT(MISSION_CURRENT_DATA),
    MISSION_REQUEST_LIST(MISSION_REQUEST_LIST_DATA),
    MISSION_COUNT(MISSION_COUNT_DATA),
    MISSION_CLEAR_ALL(MISSION_CLEAR_ALL_DATA),
    MISSION_ITEM_REACHED(MISSION_ITEM_REACHED_DATA),
    MISSION_ACK(MISSION_ACK_DATA),
    SET_GPS_GLOBAL_ORIGIN(SET_GPS_GLOBAL_ORIGIN_DATA),
    GPS_GLOBAL_ORIGIN(GPS_GLOBAL_ORIGIN_DATA),
    PARAM_MAP_RC(PARAM_MAP_RC_DATA),
    MISSION_REQUEST_INT(MISSION_REQUEST_INT_DATA),
    MISSION_CHANGED(MISSION_CHANGED_DATA),
    SAFETY_SET_ALLOWED_AREA(SAFETY_SET_ALLOWED_AREA_DATA),
    SAFETY_ALLOWED_AREA(SAFETY_ALLOWED_AREA_DATA),
    ATTITUDE_QUATERNION_COV(ATTITUDE_QUATERNION_COV_DATA),
    NAV_CONTROLLER_OUTPUT(NAV_CONTROLLER_OUTPUT_DATA),
    GLOBAL_POSITION_INT_COV(GLOBAL_POSITION_INT_COV_DATA),
    LOCAL_POSITION_NED_COV(LOCAL_POSITION_NED_COV_DATA),
    RC_CHANNELS(RC_CHANNELS_DATA),
    REQUEST_DATA_STREAM(REQUEST_DATA_STREAM_DATA),
    DATA_STREAM(DATA_STREAM_DATA),
    MANUAL_CONTROL(MANUAL_CONTROL_DATA),
    RC_CHANNELS_OVERRIDE(RC_CHANNELS_OVERRIDE_DATA),
    MISSION_ITEM_INT(MISSION_ITEM_INT_DATA),
    VFR_HUD(VFR_HUD_DATA),
    COMMAND_INT(COMMAND_INT_DATA),
    COMMAND_LONG(COMMAND_LONG_DATA),
    COMMAND_ACK(COMMAND_ACK_DATA),
    COMMAND_CANCEL(COMMAND_CANCEL_DATA),
    MANUAL_SETPOINT(MANUAL_SETPOINT_DATA),
    SET_ATTITUDE_TARGET(SET_ATTITUDE_TARGET_DATA),
    ATTITUDE_TARGET(ATTITUDE_TARGET_DATA),
    SET_POSITION_TARGET_LOCAL_NED(SET_POSITION_TARGET_LOCAL_NED_DATA),
    POSITION_TARGET_LOCAL_NED(POSITION_TARGET_LOCAL_NED_DATA),
    SET_POSITION_TARGET_GLOBAL_INT(SET_POSITION_TARGET_GLOBAL_INT_DATA),
    POSITION_TARGET_GLOBAL_INT(POSITION_TARGET_GLOBAL_INT_DATA),
    LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA),
    HIL_STATE(HIL_STATE_DATA),
    HIL_CONTROLS(HIL_CONTROLS_DATA),
    HIL_RC_INPUTS_RAW(HIL_RC_INPUTS_RAW_DATA),
    HIL_ACTUATOR_CONTROLS(HIL_ACTUATOR_CONTROLS_DATA),
    OPTICAL_FLOW(OPTICAL_FLOW_DATA),
    GLOBAL_VISION_POSITION_ESTIMATE(GLOBAL_VISION_POSITION_ESTIMATE_DATA),
    VISION_POSITION_ESTIMATE(VISION_POSITION_ESTIMATE_DATA),
    VISION_SPEED_ESTIMATE(VISION_SPEED_ESTIMATE_DATA),
    VICON_POSITION_ESTIMATE(VICON_POSITION_ESTIMATE_DATA),
    HIGHRES_IMU(HIGHRES_IMU_DATA),
    OPTICAL_FLOW_RAD(OPTICAL_FLOW_RAD_DATA),
    HIL_SENSOR(HIL_SENSOR_DATA),
    SIM_STATE(SIM_STATE_DATA),
    RADIO_STATUS(RADIO_STATUS_DATA),
    FILE_TRANSFER_PROTOCOL(FILE_TRANSFER_PROTOCOL_DATA),
    TIMESYNC(TIMESYNC_DATA),
    CAMERA_TRIGGER(CAMERA_TRIGGER_DATA),
    HIL_GPS(HIL_GPS_DATA),
    HIL_OPTICAL_FLOW(HIL_OPTICAL_FLOW_DATA),
    HIL_STATE_QUATERNION(HIL_STATE_QUATERNION_DATA),
    SCALED_IMU2(SCALED_IMU2_DATA),
    LOG_REQUEST_LIST(LOG_REQUEST_LIST_DATA),
    LOG_ENTRY(LOG_ENTRY_DATA),
    LOG_REQUEST_DATA(LOG_REQUEST_DATA_DATA),
    LOG_DATA(LOG_DATA_DATA),
    LOG_ERASE(LOG_ERASE_DATA),
    LOG_REQUEST_END(LOG_REQUEST_END_DATA),
    GPS_INJECT_DATA(GPS_INJECT_DATA_DATA),
    GPS2_RAW(GPS2_RAW_DATA),
    POWER_STATUS(POWER_STATUS_DATA),
    SERIAL_CONTROL(SERIAL_CONTROL_DATA),
    GPS_RTK(GPS_RTK_DATA),
    GPS2_RTK(GPS2_RTK_DATA),
    SCALED_IMU3(SCALED_IMU3_DATA),
    DATA_TRANSMISSION_HANDSHAKE(DATA_TRANSMISSION_HANDSHAKE_DATA),
    ENCAPSULATED_DATA(ENCAPSULATED_DATA_DATA),
    DISTANCE_SENSOR(DISTANCE_SENSOR_DATA),
    TERRAIN_REQUEST(TERRAIN_REQUEST_DATA),
    TERRAIN_DATA(TERRAIN_DATA_DATA),
    TERRAIN_CHECK(TERRAIN_CHECK_DATA),
    TERRAIN_REPORT(TERRAIN_REPORT_DATA),
    SCALED_PRESSURE2(SCALED_PRESSURE2_DATA),
    ATT_POS_MOCAP(ATT_POS_MOCAP_DATA),
    SET_ACTUATOR_CONTROL_TARGET(SET_ACTUATOR_CONTROL_TARGET_DATA),
    ACTUATOR_CONTROL_TARGET(ACTUATOR_CONTROL_TARGET_DATA),
    ALTITUDE(ALTITUDE_DATA),
    RESOURCE_REQUEST(RESOURCE_REQUEST_DATA),
    SCALED_PRESSURE3(SCALED_PRESSURE3_DATA),
    FOLLOW_TARGET(FOLLOW_TARGET_DATA),
    CONTROL_SYSTEM_STATE(CONTROL_SYSTEM_STATE_DATA),
    BATTERY_STATUS(BATTERY_STATUS_DATA),
    AUTOPILOT_VERSION(AUTOPILOT_VERSION_DATA),
    LANDING_TARGET(LANDING_TARGET_DATA),
    FENCE_STATUS(FENCE_STATUS_DATA),
    ESTIMATOR_STATUS(ESTIMATOR_STATUS_DATA),
    WIND_COV(WIND_COV_DATA),
    GPS_INPUT(GPS_INPUT_DATA),
    GPS_RTCM_DATA(GPS_RTCM_DATA_DATA),
    HIGH_LATENCY(HIGH_LATENCY_DATA),
    HIGH_LATENCY2(HIGH_LATENCY2_DATA),
    VIBRATION(VIBRATION_DATA),
    HOME_POSITION(HOME_POSITION_DATA),
    SET_HOME_POSITION(SET_HOME_POSITION_DATA),
    MESSAGE_INTERVAL(MESSAGE_INTERVAL_DATA),
    EXTENDED_SYS_STATE(EXTENDED_SYS_STATE_DATA),
    ADSB_VEHICLE(ADSB_VEHICLE_DATA),
    COLLISION(COLLISION_DATA),
    V2_EXTENSION(V2_EXTENSION_DATA),
    MEMORY_VECT(MEMORY_VECT_DATA),
    DEBUG_VECT(DEBUG_VECT_DATA),
    NAMED_VALUE_FLOAT(NAMED_VALUE_FLOAT_DATA),
    NAMED_VALUE_INT(NAMED_VALUE_INT_DATA),
    STATUSTEXT(STATUSTEXT_DATA),
    DEBUG(DEBUG_DATA),
    SETUP_SIGNING(SETUP_SIGNING_DATA),
    BUTTON_CHANGE(BUTTON_CHANGE_DATA),
    PLAY_TUNE(PLAY_TUNE_DATA),
    CAMERA_INFORMATION(CAMERA_INFORMATION_DATA),
    CAMERA_SETTINGS(CAMERA_SETTINGS_DATA),
    STORAGE_INFORMATION(STORAGE_INFORMATION_DATA),
    CAMERA_CAPTURE_STATUS(CAMERA_CAPTURE_STATUS_DATA),
    CAMERA_IMAGE_CAPTURED(CAMERA_IMAGE_CAPTURED_DATA),
    FLIGHT_INFORMATION(FLIGHT_INFORMATION_DATA),
    MOUNT_ORIENTATION(MOUNT_ORIENTATION_DATA),
    LOGGING_DATA(LOGGING_DATA_DATA),
    LOGGING_DATA_ACKED(LOGGING_DATA_ACKED_DATA),
    LOGGING_ACK(LOGGING_ACK_DATA),
    VIDEO_STREAM_INFORMATION(VIDEO_STREAM_INFORMATION_DATA),
    VIDEO_STREAM_STATUS(VIDEO_STREAM_STATUS_DATA),
    GIMBAL_MANAGER_INFORMATION(GIMBAL_MANAGER_INFORMATION_DATA),
    GIMBAL_MANAGER_STATUS(GIMBAL_MANAGER_STATUS_DATA),
    GIMBAL_MANAGER_SET_ATTITUDE(GIMBAL_MANAGER_SET_ATTITUDE_DATA),
    GIMBAL_DEVICE_INFORMATION(GIMBAL_DEVICE_INFORMATION_DATA),
    GIMBAL_DEVICE_SET_ATTITUDE(GIMBAL_DEVICE_SET_ATTITUDE_DATA),
    GIMBAL_DEVICE_ATTITUDE_STATUS(GIMBAL_DEVICE_ATTITUDE_STATUS_DATA),
    AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA),
    GIMBAL_MANAGER_SET_TILTPAN(GIMBAL_MANAGER_SET_TILTPAN_DATA),
    WIFI_CONFIG_AP(WIFI_CONFIG_AP_DATA),
    PROTOCOL_VERSION(PROTOCOL_VERSION_DATA),
    AIS_VESSEL(AIS_VESSEL_DATA),
    UAVCAN_NODE_STATUS(UAVCAN_NODE_STATUS_DATA),
    UAVCAN_NODE_INFO(UAVCAN_NODE_INFO_DATA),
    PARAM_EXT_REQUEST_READ(PARAM_EXT_REQUEST_READ_DATA),
    PARAM_EXT_REQUEST_LIST(PARAM_EXT_REQUEST_LIST_DATA),
    PARAM_EXT_VALUE(PARAM_EXT_VALUE_DATA),
    PARAM_EXT_SET(PARAM_EXT_SET_DATA),
    PARAM_EXT_ACK(PARAM_EXT_ACK_DATA),
    OBSTACLE_DISTANCE(OBSTACLE_DISTANCE_DATA),
    ODOMETRY(ODOMETRY_DATA),
    TRAJECTORY_REPRESENTATION_WAYPOINTS(TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA),
    TRAJECTORY_REPRESENTATION_BEZIER(TRAJECTORY_REPRESENTATION_BEZIER_DATA),
    CELLULAR_STATUS(CELLULAR_STATUS_DATA),
    ISBD_LINK_STATUS(ISBD_LINK_STATUS_DATA),
    CELLULAR_CONFIG(CELLULAR_CONFIG_DATA),
    RAW_RPM(RAW_RPM_DATA),
    UTM_GLOBAL_POSITION(UTM_GLOBAL_POSITION_DATA),
    DEBUG_FLOAT_ARRAY(DEBUG_FLOAT_ARRAY_DATA),
    ORBIT_EXECUTION_STATUS(ORBIT_EXECUTION_STATUS_DATA),
    SMART_BATTERY_INFO(SMART_BATTERY_INFO_DATA),
    SMART_BATTERY_STATUS(SMART_BATTERY_STATUS_DATA),
    GENERATOR_STATUS(GENERATOR_STATUS_DATA),
    ACTUATOR_OUTPUT_STATUS(ACTUATOR_OUTPUT_STATUS_DATA),
    TIME_ESTIMATE_TO_TARGET(TIME_ESTIMATE_TO_TARGET_DATA),
    TUNNEL(TUNNEL_DATA),
    ONBOARD_COMPUTER_STATUS(ONBOARD_COMPUTER_STATUS_DATA),
    COMPONENT_INFORMATION(COMPONENT_INFORMATION_DATA),
    PLAY_TUNE_V2(PLAY_TUNE_V2_DATA),
    SUPPORTED_TUNES(SUPPORTED_TUNES_DATA),
    WHEEL_DISTANCE(WHEEL_DISTANCE_DATA),
    OPEN_DRONE_ID_BASIC_ID(OPEN_DRONE_ID_BASIC_ID_DATA),
    OPEN_DRONE_ID_LOCATION(OPEN_DRONE_ID_LOCATION_DATA),
    OPEN_DRONE_ID_AUTHENTICATION(OPEN_DRONE_ID_AUTHENTICATION_DATA),
    OPEN_DRONE_ID_SELF_ID(OPEN_DRONE_ID_SELF_ID_DATA),
    OPEN_DRONE_ID_SYSTEM(OPEN_DRONE_ID_SYSTEM_DATA),
    OPEN_DRONE_ID_OPERATOR_ID(OPEN_DRONE_ID_OPERATOR_ID_DATA),
    OPEN_DRONE_ID_MESSAGE_PACK(OPEN_DRONE_ID_MESSAGE_PACK_DATA),
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            0 => HEARTBEAT_DATA::deser(version, payload).map(|s| MavMessage::HEARTBEAT(s)),
            1 => SYS_STATUS_DATA::deser(version, payload).map(|s| MavMessage::SYS_STATUS(s)),
            2 => SYSTEM_TIME_DATA::deser(version, payload).map(|s| MavMessage::SYSTEM_TIME(s)),
            4 => PING_DATA::deser(version, payload).map(|s| MavMessage::PING(s)),
            5 => CHANGE_OPERATOR_CONTROL_DATA::deser(version, payload)
                .map(|s| MavMessage::CHANGE_OPERATOR_CONTROL(s)),
            6 => CHANGE_OPERATOR_CONTROL_ACK_DATA::deser(version, payload)
                .map(|s| MavMessage::CHANGE_OPERATOR_CONTROL_ACK(s)),
            7 => AUTH_KEY_DATA::deser(version, payload).map(|s| MavMessage::AUTH_KEY(s)),
            8 => LINK_NODE_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::LINK_NODE_STATUS(s)),
            11 => SET_MODE_DATA::deser(version, payload).map(|s| MavMessage::SET_MODE(s)),
            20 => PARAM_REQUEST_READ_DATA::deser(version, payload)
                .map(|s| MavMessage::PARAM_REQUEST_READ(s)),
            21 => PARAM_REQUEST_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::PARAM_REQUEST_LIST(s)),
            22 => PARAM_VALUE_DATA::deser(version, payload).map(|s| MavMessage::PARAM_VALUE(s)),
            23 => PARAM_SET_DATA::deser(version, payload).map(|s| MavMessage::PARAM_SET(s)),
            24 => GPS_RAW_INT_DATA::deser(version, payload).map(|s| MavMessage::GPS_RAW_INT(s)),
            25 => GPS_STATUS_DATA::deser(version, payload).map(|s| MavMessage::GPS_STATUS(s)),
            26 => SCALED_IMU_DATA::deser(version, payload).map(|s| MavMessage::SCALED_IMU(s)),
            27 => RAW_IMU_DATA::deser(version, payload).map(|s| MavMessage::RAW_IMU(s)),
            28 => RAW_PRESSURE_DATA::deser(version, payload).map(|s| MavMessage::RAW_PRESSURE(s)),
            29 => SCALED_PRESSURE_DATA::deser(version, payload)
                .map(|s| MavMessage::SCALED_PRESSURE(s)),
            30 => ATTITUDE_DATA::deser(version, payload).map(|s| MavMessage::ATTITUDE(s)),
            31 => ATTITUDE_QUATERNION_DATA::deser(version, payload)
                .map(|s| MavMessage::ATTITUDE_QUATERNION(s)),
            32 => LOCAL_POSITION_NED_DATA::deser(version, payload)
                .map(|s| MavMessage::LOCAL_POSITION_NED(s)),
            33 => GLOBAL_POSITION_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::GLOBAL_POSITION_INT(s)),
            34 => RC_CHANNELS_SCALED_DATA::deser(version, payload)
                .map(|s| MavMessage::RC_CHANNELS_SCALED(s)),
            35 => RC_CHANNELS_RAW_DATA::deser(version, payload)
                .map(|s| MavMessage::RC_CHANNELS_RAW(s)),
            36 => SERVO_OUTPUT_RAW_DATA::deser(version, payload)
                .map(|s| MavMessage::SERVO_OUTPUT_RAW(s)),
            37 => MISSION_REQUEST_PARTIAL_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_REQUEST_PARTIAL_LIST(s)),
            38 => MISSION_WRITE_PARTIAL_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_WRITE_PARTIAL_LIST(s)),
            39 => MISSION_ITEM_DATA::deser(version, payload).map(|s| MavMessage::MISSION_ITEM(s)),
            40 => MISSION_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_REQUEST(s)),
            41 => MISSION_SET_CURRENT_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_SET_CURRENT(s)),
            42 => MISSION_CURRENT_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_CURRENT(s)),
            43 => MISSION_REQUEST_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_REQUEST_LIST(s)),
            44 => MISSION_COUNT_DATA::deser(version, payload).map(|s| MavMessage::MISSION_COUNT(s)),
            45 => MISSION_CLEAR_ALL_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_CLEAR_ALL(s)),
            46 => MISSION_ITEM_REACHED_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_ITEM_REACHED(s)),
            47 => MISSION_ACK_DATA::deser(version, payload).map(|s| MavMessage::MISSION_ACK(s)),
            48 => SET_GPS_GLOBAL_ORIGIN_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_GPS_GLOBAL_ORIGIN(s)),
            49 => GPS_GLOBAL_ORIGIN_DATA::deser(version, payload)
                .map(|s| MavMessage::GPS_GLOBAL_ORIGIN(s)),
            50 => PARAM_MAP_RC_DATA::deser(version, payload).map(|s| MavMessage::PARAM_MAP_RC(s)),
            51 => MISSION_REQUEST_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_REQUEST_INT(s)),
            52 => MISSION_CHANGED_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_CHANGED(s)),
            54 => SAFETY_SET_ALLOWED_AREA_DATA::deser(version, payload)
                .map(|s| MavMessage::SAFETY_SET_ALLOWED_AREA(s)),
            55 => SAFETY_ALLOWED_AREA_DATA::deser(version, payload)
                .map(|s| MavMessage::SAFETY_ALLOWED_AREA(s)),
            61 => ATTITUDE_QUATERNION_COV_DATA::deser(version, payload)
                .map(|s| MavMessage::ATTITUDE_QUATERNION_COV(s)),
            62 => NAV_CONTROLLER_OUTPUT_DATA::deser(version, payload)
                .map(|s| MavMessage::NAV_CONTROLLER_OUTPUT(s)),
            63 => GLOBAL_POSITION_INT_COV_DATA::deser(version, payload)
                .map(|s| MavMessage::GLOBAL_POSITION_INT_COV(s)),
            64 => LOCAL_POSITION_NED_COV_DATA::deser(version, payload)
                .map(|s| MavMessage::LOCAL_POSITION_NED_COV(s)),
            65 => RC_CHANNELS_DATA::deser(version, payload).map(|s| MavMessage::RC_CHANNELS(s)),
            66 => REQUEST_DATA_STREAM_DATA::deser(version, payload)
                .map(|s| MavMessage::REQUEST_DATA_STREAM(s)),
            67 => DATA_STREAM_DATA::deser(version, payload).map(|s| MavMessage::DATA_STREAM(s)),
            69 => {
                MANUAL_CONTROL_DATA::deser(version, payload).map(|s| MavMessage::MANUAL_CONTROL(s))
            }
            70 => RC_CHANNELS_OVERRIDE_DATA::deser(version, payload)
                .map(|s| MavMessage::RC_CHANNELS_OVERRIDE(s)),
            73 => MISSION_ITEM_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::MISSION_ITEM_INT(s)),
            74 => VFR_HUD_DATA::deser(version, payload).map(|s| MavMessage::VFR_HUD(s)),
            75 => COMMAND_INT_DATA::deser(version, payload).map(|s| MavMessage::COMMAND_INT(s)),
            76 => COMMAND_LONG_DATA::deser(version, payload).map(|s| MavMessage::COMMAND_LONG(s)),
            77 => COMMAND_ACK_DATA::deser(version, payload).map(|s| MavMessage::COMMAND_ACK(s)),
            80 => {
                COMMAND_CANCEL_DATA::deser(version, payload).map(|s| MavMessage::COMMAND_CANCEL(s))
            }
            81 => MANUAL_SETPOINT_DATA::deser(version, payload)
                .map(|s| MavMessage::MANUAL_SETPOINT(s)),
            82 => SET_ATTITUDE_TARGET_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_ATTITUDE_TARGET(s)),
            83 => ATTITUDE_TARGET_DATA::deser(version, payload)
                .map(|s| MavMessage::ATTITUDE_TARGET(s)),
            84 => SET_POSITION_TARGET_LOCAL_NED_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_POSITION_TARGET_LOCAL_NED(s)),
            85 => POSITION_TARGET_LOCAL_NED_DATA::deser(version, payload)
                .map(|s| MavMessage::POSITION_TARGET_LOCAL_NED(s)),
            86 => SET_POSITION_TARGET_GLOBAL_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_POSITION_TARGET_GLOBAL_INT(s)),
            87 => POSITION_TARGET_GLOBAL_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::POSITION_TARGET_GLOBAL_INT(s)),
            89 => LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA::deser(version, payload)
                .map(|s| MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(s)),
            90 => HIL_STATE_DATA::deser(version, payload).map(|s| MavMessage::HIL_STATE(s)),
            91 => HIL_CONTROLS_DATA::deser(version, payload).map(|s| MavMessage::HIL_CONTROLS(s)),
            92 => HIL_RC_INPUTS_RAW_DATA::deser(version, payload)
                .map(|s| MavMessage::HIL_RC_INPUTS_RAW(s)),
            93 => HIL_ACTUATOR_CONTROLS_DATA::deser(version, payload)
                .map(|s| MavMessage::HIL_ACTUATOR_CONTROLS(s)),
            100 => OPTICAL_FLOW_DATA::deser(version, payload).map(|s| MavMessage::OPTICAL_FLOW(s)),
            101 => GLOBAL_VISION_POSITION_ESTIMATE_DATA::deser(version, payload)
                .map(|s| MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(s)),
            102 => VISION_POSITION_ESTIMATE_DATA::deser(version, payload)
                .map(|s| MavMessage::VISION_POSITION_ESTIMATE(s)),
            103 => VISION_SPEED_ESTIMATE_DATA::deser(version, payload)
                .map(|s| MavMessage::VISION_SPEED_ESTIMATE(s)),
            104 => VICON_POSITION_ESTIMATE_DATA::deser(version, payload)
                .map(|s| MavMessage::VICON_POSITION_ESTIMATE(s)),
            105 => HIGHRES_IMU_DATA::deser(version, payload).map(|s| MavMessage::HIGHRES_IMU(s)),
            106 => OPTICAL_FLOW_RAD_DATA::deser(version, payload)
                .map(|s| MavMessage::OPTICAL_FLOW_RAD(s)),
            107 => HIL_SENSOR_DATA::deser(version, payload).map(|s| MavMessage::HIL_SENSOR(s)),
            108 => SIM_STATE_DATA::deser(version, payload).map(|s| MavMessage::SIM_STATE(s)),
            109 => RADIO_STATUS_DATA::deser(version, payload).map(|s| MavMessage::RADIO_STATUS(s)),
            110 => FILE_TRANSFER_PROTOCOL_DATA::deser(version, payload)
                .map(|s| MavMessage::FILE_TRANSFER_PROTOCOL(s)),
            111 => TIMESYNC_DATA::deser(version, payload).map(|s| MavMessage::TIMESYNC(s)),
            112 => {
                CAMERA_TRIGGER_DATA::deser(version, payload).map(|s| MavMessage::CAMERA_TRIGGER(s))
            }
            113 => HIL_GPS_DATA::deser(version, payload).map(|s| MavMessage::HIL_GPS(s)),
            114 => HIL_OPTICAL_FLOW_DATA::deser(version, payload)
                .map(|s| MavMessage::HIL_OPTICAL_FLOW(s)),
            115 => HIL_STATE_QUATERNION_DATA::deser(version, payload)
                .map(|s| MavMessage::HIL_STATE_QUATERNION(s)),
            116 => SCALED_IMU2_DATA::deser(version, payload).map(|s| MavMessage::SCALED_IMU2(s)),
            117 => LOG_REQUEST_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::LOG_REQUEST_LIST(s)),
            118 => LOG_ENTRY_DATA::deser(version, payload).map(|s| MavMessage::LOG_ENTRY(s)),
            119 => LOG_REQUEST_DATA_DATA::deser(version, payload)
                .map(|s| MavMessage::LOG_REQUEST_DATA(s)),
            120 => LOG_DATA_DATA::deser(version, payload).map(|s| MavMessage::LOG_DATA(s)),
            121 => LOG_ERASE_DATA::deser(version, payload).map(|s| MavMessage::LOG_ERASE(s)),
            122 => LOG_REQUEST_END_DATA::deser(version, payload)
                .map(|s| MavMessage::LOG_REQUEST_END(s)),
            123 => GPS_INJECT_DATA_DATA::deser(version, payload)
                .map(|s| MavMessage::GPS_INJECT_DATA(s)),
            124 => GPS2_RAW_DATA::deser(version, payload).map(|s| MavMessage::GPS2_RAW(s)),
            125 => POWER_STATUS_DATA::deser(version, payload).map(|s| MavMessage::POWER_STATUS(s)),
            126 => {
                SERIAL_CONTROL_DATA::deser(version, payload).map(|s| MavMessage::SERIAL_CONTROL(s))
            }
            127 => GPS_RTK_DATA::deser(version, payload).map(|s| MavMessage::GPS_RTK(s)),
            128 => GPS2_RTK_DATA::deser(version, payload).map(|s| MavMessage::GPS2_RTK(s)),
            129 => SCALED_IMU3_DATA::deser(version, payload).map(|s| MavMessage::SCALED_IMU3(s)),
            130 => DATA_TRANSMISSION_HANDSHAKE_DATA::deser(version, payload)
                .map(|s| MavMessage::DATA_TRANSMISSION_HANDSHAKE(s)),
            131 => ENCAPSULATED_DATA_DATA::deser(version, payload)
                .map(|s| MavMessage::ENCAPSULATED_DATA(s)),
            132 => DISTANCE_SENSOR_DATA::deser(version, payload)
                .map(|s| MavMessage::DISTANCE_SENSOR(s)),
            133 => TERRAIN_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::TERRAIN_REQUEST(s)),
            134 => TERRAIN_DATA_DATA::deser(version, payload).map(|s| MavMessage::TERRAIN_DATA(s)),
            135 => {
                TERRAIN_CHECK_DATA::deser(version, payload).map(|s| MavMessage::TERRAIN_CHECK(s))
            }
            136 => {
                TERRAIN_REPORT_DATA::deser(version, payload).map(|s| MavMessage::TERRAIN_REPORT(s))
            }
            137 => SCALED_PRESSURE2_DATA::deser(version, payload)
                .map(|s| MavMessage::SCALED_PRESSURE2(s)),
            138 => {
                ATT_POS_MOCAP_DATA::deser(version, payload).map(|s| MavMessage::ATT_POS_MOCAP(s))
            }
            139 => SET_ACTUATOR_CONTROL_TARGET_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_ACTUATOR_CONTROL_TARGET(s)),
            140 => ACTUATOR_CONTROL_TARGET_DATA::deser(version, payload)
                .map(|s| MavMessage::ACTUATOR_CONTROL_TARGET(s)),
            141 => ALTITUDE_DATA::deser(version, payload).map(|s| MavMessage::ALTITUDE(s)),
            142 => RESOURCE_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::RESOURCE_REQUEST(s)),
            143 => SCALED_PRESSURE3_DATA::deser(version, payload)
                .map(|s| MavMessage::SCALED_PRESSURE3(s)),
            144 => {
                FOLLOW_TARGET_DATA::deser(version, payload).map(|s| MavMessage::FOLLOW_TARGET(s))
            }
            146 => CONTROL_SYSTEM_STATE_DATA::deser(version, payload)
                .map(|s| MavMessage::CONTROL_SYSTEM_STATE(s)),
            147 => {
                BATTERY_STATUS_DATA::deser(version, payload).map(|s| MavMessage::BATTERY_STATUS(s))
            }
            148 => AUTOPILOT_VERSION_DATA::deser(version, payload)
                .map(|s| MavMessage::AUTOPILOT_VERSION(s)),
            149 => {
                LANDING_TARGET_DATA::deser(version, payload).map(|s| MavMessage::LANDING_TARGET(s))
            }
            162 => FENCE_STATUS_DATA::deser(version, payload).map(|s| MavMessage::FENCE_STATUS(s)),
            230 => ESTIMATOR_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::ESTIMATOR_STATUS(s)),
            231 => WIND_COV_DATA::deser(version, payload).map(|s| MavMessage::WIND_COV(s)),
            232 => GPS_INPUT_DATA::deser(version, payload).map(|s| MavMessage::GPS_INPUT(s)),
            233 => {
                GPS_RTCM_DATA_DATA::deser(version, payload).map(|s| MavMessage::GPS_RTCM_DATA(s))
            }
            234 => HIGH_LATENCY_DATA::deser(version, payload).map(|s| MavMessage::HIGH_LATENCY(s)),
            235 => {
                HIGH_LATENCY2_DATA::deser(version, payload).map(|s| MavMessage::HIGH_LATENCY2(s))
            }
            241 => VIBRATION_DATA::deser(version, payload).map(|s| MavMessage::VIBRATION(s)),
            242 => {
                HOME_POSITION_DATA::deser(version, payload).map(|s| MavMessage::HOME_POSITION(s))
            }
            243 => SET_HOME_POSITION_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_HOME_POSITION(s)),
            244 => MESSAGE_INTERVAL_DATA::deser(version, payload)
                .map(|s| MavMessage::MESSAGE_INTERVAL(s)),
            245 => EXTENDED_SYS_STATE_DATA::deser(version, payload)
                .map(|s| MavMessage::EXTENDED_SYS_STATE(s)),
            246 => ADSB_VEHICLE_DATA::deser(version, payload).map(|s| MavMessage::ADSB_VEHICLE(s)),
            247 => COLLISION_DATA::deser(version, payload).map(|s| MavMessage::COLLISION(s)),
            248 => V2_EXTENSION_DATA::deser(version, payload).map(|s| MavMessage::V2_EXTENSION(s)),
            249 => MEMORY_VECT_DATA::deser(version, payload).map(|s| MavMessage::MEMORY_VECT(s)),
            250 => DEBUG_VECT_DATA::deser(version, payload).map(|s| MavMessage::DEBUG_VECT(s)),
            251 => NAMED_VALUE_FLOAT_DATA::deser(version, payload)
                .map(|s| MavMessage::NAMED_VALUE_FLOAT(s)),
            252 => NAMED_VALUE_INT_DATA::deser(version, payload)
                .map(|s| MavMessage::NAMED_VALUE_INT(s)),
            253 => STATUSTEXT_DATA::deser(version, payload).map(|s| MavMessage::STATUSTEXT(s)),
            254 => DEBUG_DATA::deser(version, payload).map(|s| MavMessage::DEBUG(s)),
            256 => {
                SETUP_SIGNING_DATA::deser(version, payload).map(|s| MavMessage::SETUP_SIGNING(s))
            }
            257 => {
                BUTTON_CHANGE_DATA::deser(version, payload).map(|s| MavMessage::BUTTON_CHANGE(s))
            }
            258 => PLAY_TUNE_DATA::deser(version, payload).map(|s| MavMessage::PLAY_TUNE(s)),
            259 => CAMERA_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::CAMERA_INFORMATION(s)),
            260 => CAMERA_SETTINGS_DATA::deser(version, payload)
                .map(|s| MavMessage::CAMERA_SETTINGS(s)),
            261 => STORAGE_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::STORAGE_INFORMATION(s)),
            262 => CAMERA_CAPTURE_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::CAMERA_CAPTURE_STATUS(s)),
            263 => CAMERA_IMAGE_CAPTURED_DATA::deser(version, payload)
                .map(|s| MavMessage::CAMERA_IMAGE_CAPTURED(s)),
            264 => FLIGHT_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::FLIGHT_INFORMATION(s)),
            265 => MOUNT_ORIENTATION_DATA::deser(version, payload)
                .map(|s| MavMessage::MOUNT_ORIENTATION(s)),
            266 => LOGGING_DATA_DATA::deser(version, payload).map(|s| MavMessage::LOGGING_DATA(s)),
            267 => LOGGING_DATA_ACKED_DATA::deser(version, payload)
                .map(|s| MavMessage::LOGGING_DATA_ACKED(s)),
            268 => LOGGING_ACK_DATA::deser(version, payload).map(|s| MavMessage::LOGGING_ACK(s)),
            269 => VIDEO_STREAM_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::VIDEO_STREAM_INFORMATION(s)),
            270 => VIDEO_STREAM_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::VIDEO_STREAM_STATUS(s)),
            280 => GIMBAL_MANAGER_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_MANAGER_INFORMATION(s)),
            281 => GIMBAL_MANAGER_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_MANAGER_STATUS(s)),
            282 => GIMBAL_MANAGER_SET_ATTITUDE_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(s)),
            283 => GIMBAL_DEVICE_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_DEVICE_INFORMATION(s)),
            284 => GIMBAL_DEVICE_SET_ATTITUDE_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(s)),
            285 => GIMBAL_DEVICE_ATTITUDE_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(s)),
            286 => AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA::deser(version, payload)
                .map(|s| MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(s)),
            287 => GIMBAL_MANAGER_SET_TILTPAN_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_MANAGER_SET_TILTPAN(s)),
            299 => {
                WIFI_CONFIG_AP_DATA::deser(version, payload).map(|s| MavMessage::WIFI_CONFIG_AP(s))
            }
            300 => PROTOCOL_VERSION_DATA::deser(version, payload)
                .map(|s| MavMessage::PROTOCOL_VERSION(s)),
            301 => AIS_VESSEL_DATA::deser(version, payload).map(|s| MavMessage::AIS_VESSEL(s)),
            310 => UAVCAN_NODE_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::UAVCAN_NODE_STATUS(s)),
            311 => UAVCAN_NODE_INFO_DATA::deser(version, payload)
                .map(|s| MavMessage::UAVCAN_NODE_INFO(s)),
            320 => PARAM_EXT_REQUEST_READ_DATA::deser(version, payload)
                .map(|s| MavMessage::PARAM_EXT_REQUEST_READ(s)),
            321 => PARAM_EXT_REQUEST_LIST_DATA::deser(version, payload)
                .map(|s| MavMessage::PARAM_EXT_REQUEST_LIST(s)),
            322 => PARAM_EXT_VALUE_DATA::deser(version, payload)
                .map(|s| MavMessage::PARAM_EXT_VALUE(s)),
            323 => {
                PARAM_EXT_SET_DATA::deser(version, payload).map(|s| MavMessage::PARAM_EXT_SET(s))
            }
            324 => {
                PARAM_EXT_ACK_DATA::deser(version, payload).map(|s| MavMessage::PARAM_EXT_ACK(s))
            }
            330 => OBSTACLE_DISTANCE_DATA::deser(version, payload)
                .map(|s| MavMessage::OBSTACLE_DISTANCE(s)),
            331 => ODOMETRY_DATA::deser(version, payload).map(|s| MavMessage::ODOMETRY(s)),
            332 => TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA::deser(version, payload)
                .map(|s| MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(s)),
            333 => TRAJECTORY_REPRESENTATION_BEZIER_DATA::deser(version, payload)
                .map(|s| MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(s)),
            334 => CELLULAR_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::CELLULAR_STATUS(s)),
            335 => ISBD_LINK_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::ISBD_LINK_STATUS(s)),
            336 => CELLULAR_CONFIG_DATA::deser(version, payload)
                .map(|s| MavMessage::CELLULAR_CONFIG(s)),
            339 => RAW_RPM_DATA::deser(version, payload).map(|s| MavMessage::RAW_RPM(s)),
            340 => UTM_GLOBAL_POSITION_DATA::deser(version, payload)
                .map(|s| MavMessage::UTM_GLOBAL_POSITION(s)),
            350 => DEBUG_FLOAT_ARRAY_DATA::deser(version, payload)
                .map(|s| MavMessage::DEBUG_FLOAT_ARRAY(s)),
            360 => ORBIT_EXECUTION_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::ORBIT_EXECUTION_STATUS(s)),
            370 => SMART_BATTERY_INFO_DATA::deser(version, payload)
                .map(|s| MavMessage::SMART_BATTERY_INFO(s)),
            371 => SMART_BATTERY_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::SMART_BATTERY_STATUS(s)),
            373 => GENERATOR_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::GENERATOR_STATUS(s)),
            375 => ACTUATOR_OUTPUT_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::ACTUATOR_OUTPUT_STATUS(s)),
            380 => TIME_ESTIMATE_TO_TARGET_DATA::deser(version, payload)
                .map(|s| MavMessage::TIME_ESTIMATE_TO_TARGET(s)),
            385 => TUNNEL_DATA::deser(version, payload).map(|s| MavMessage::TUNNEL(s)),
            390 => ONBOARD_COMPUTER_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::ONBOARD_COMPUTER_STATUS(s)),
            395 => COMPONENT_INFORMATION_DATA::deser(version, payload)
                .map(|s| MavMessage::COMPONENT_INFORMATION(s)),
            400 => PLAY_TUNE_V2_DATA::deser(version, payload).map(|s| MavMessage::PLAY_TUNE_V2(s)),
            401 => SUPPORTED_TUNES_DATA::deser(version, payload)
                .map(|s| MavMessage::SUPPORTED_TUNES(s)),
            9000 => {
                WHEEL_DISTANCE_DATA::deser(version, payload).map(|s| MavMessage::WHEEL_DISTANCE(s))
            }
            12900 => OPEN_DRONE_ID_BASIC_ID_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_BASIC_ID(s)),
            12901 => OPEN_DRONE_ID_LOCATION_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_LOCATION(s)),
            12902 => OPEN_DRONE_ID_AUTHENTICATION_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_AUTHENTICATION(s)),
            12903 => OPEN_DRONE_ID_SELF_ID_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_SELF_ID(s)),
            12904 => OPEN_DRONE_ID_SYSTEM_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_SYSTEM(s)),
            12905 => OPEN_DRONE_ID_OPERATOR_ID_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_OPERATOR_ID(s)),
            12915 => OPEN_DRONE_ID_MESSAGE_PACK_DATA::deser(version, payload)
                .map(|s| MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(s)),
            _ => Err(ParserError::UnknownMessage { id }),
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::HEARTBEAT(..) => "HEARTBEAT",
            MavMessage::SYS_STATUS(..) => "SYS_STATUS",
            MavMessage::SYSTEM_TIME(..) => "SYSTEM_TIME",
            MavMessage::PING(..) => "PING",
            MavMessage::CHANGE_OPERATOR_CONTROL(..) => "CHANGE_OPERATOR_CONTROL",
            MavMessage::CHANGE_OPERATOR_CONTROL_ACK(..) => "CHANGE_OPERATOR_CONTROL_ACK",
            MavMessage::AUTH_KEY(..) => "AUTH_KEY",
            MavMessage::LINK_NODE_STATUS(..) => "LINK_NODE_STATUS",
            MavMessage::SET_MODE(..) => "SET_MODE",
            MavMessage::PARAM_REQUEST_READ(..) => "PARAM_REQUEST_READ",
            MavMessage::PARAM_REQUEST_LIST(..) => "PARAM_REQUEST_LIST",
            MavMessage::PARAM_VALUE(..) => "PARAM_VALUE",
            MavMessage::PARAM_SET(..) => "PARAM_SET",
            MavMessage::GPS_RAW_INT(..) => "GPS_RAW_INT",
            MavMessage::GPS_STATUS(..) => "GPS_STATUS",
            MavMessage::SCALED_IMU(..) => "SCALED_IMU",
            MavMessage::RAW_IMU(..) => "RAW_IMU",
            MavMessage::RAW_PRESSURE(..) => "RAW_PRESSURE",
            MavMessage::SCALED_PRESSURE(..) => "SCALED_PRESSURE",
            MavMessage::ATTITUDE(..) => "ATTITUDE",
            MavMessage::ATTITUDE_QUATERNION(..) => "ATTITUDE_QUATERNION",
            MavMessage::LOCAL_POSITION_NED(..) => "LOCAL_POSITION_NED",
            MavMessage::GLOBAL_POSITION_INT(..) => "GLOBAL_POSITION_INT",
            MavMessage::RC_CHANNELS_SCALED(..) => "RC_CHANNELS_SCALED",
            MavMessage::RC_CHANNELS_RAW(..) => "RC_CHANNELS_RAW",
            MavMessage::SERVO_OUTPUT_RAW(..) => "SERVO_OUTPUT_RAW",
            MavMessage::MISSION_REQUEST_PARTIAL_LIST(..) => "MISSION_REQUEST_PARTIAL_LIST",
            MavMessage::MISSION_WRITE_PARTIAL_LIST(..) => "MISSION_WRITE_PARTIAL_LIST",
            MavMessage::MISSION_ITEM(..) => "MISSION_ITEM",
            MavMessage::MISSION_REQUEST(..) => "MISSION_REQUEST",
            MavMessage::MISSION_SET_CURRENT(..) => "MISSION_SET_CURRENT",
            MavMessage::MISSION_CURRENT(..) => "MISSION_CURRENT",
            MavMessage::MISSION_REQUEST_LIST(..) => "MISSION_REQUEST_LIST",
            MavMessage::MISSION_COUNT(..) => "MISSION_COUNT",
            MavMessage::MISSION_CLEAR_ALL(..) => "MISSION_CLEAR_ALL",
            MavMessage::MISSION_ITEM_REACHED(..) => "MISSION_ITEM_REACHED",
            MavMessage::MISSION_ACK(..) => "MISSION_ACK",
            MavMessage::SET_GPS_GLOBAL_ORIGIN(..) => "SET_GPS_GLOBAL_ORIGIN",
            MavMessage::GPS_GLOBAL_ORIGIN(..) => "GPS_GLOBAL_ORIGIN",
            MavMessage::PARAM_MAP_RC(..) => "PARAM_MAP_RC",
            MavMessage::MISSION_REQUEST_INT(..) => "MISSION_REQUEST_INT",
            MavMessage::MISSION_CHANGED(..) => "MISSION_CHANGED",
            MavMessage::SAFETY_SET_ALLOWED_AREA(..) => "SAFETY_SET_ALLOWED_AREA",
            MavMessage::SAFETY_ALLOWED_AREA(..) => "SAFETY_ALLOWED_AREA",
            MavMessage::ATTITUDE_QUATERNION_COV(..) => "ATTITUDE_QUATERNION_COV",
            MavMessage::NAV_CONTROLLER_OUTPUT(..) => "NAV_CONTROLLER_OUTPUT",
            MavMessage::GLOBAL_POSITION_INT_COV(..) => "GLOBAL_POSITION_INT_COV",
            MavMessage::LOCAL_POSITION_NED_COV(..) => "LOCAL_POSITION_NED_COV",
            MavMessage::RC_CHANNELS(..) => "RC_CHANNELS",
            MavMessage::REQUEST_DATA_STREAM(..) => "REQUEST_DATA_STREAM",
            MavMessage::DATA_STREAM(..) => "DATA_STREAM",
            MavMessage::MANUAL_CONTROL(..) => "MANUAL_CONTROL",
            MavMessage::RC_CHANNELS_OVERRIDE(..) => "RC_CHANNELS_OVERRIDE",
            MavMessage::MISSION_ITEM_INT(..) => "MISSION_ITEM_INT",
            MavMessage::VFR_HUD(..) => "VFR_HUD",
            MavMessage::COMMAND_INT(..) => "COMMAND_INT",
            MavMessage::COMMAND_LONG(..) => "COMMAND_LONG",
            MavMessage::COMMAND_ACK(..) => "COMMAND_ACK",
            MavMessage::COMMAND_CANCEL(..) => "COMMAND_CANCEL",
            MavMessage::MANUAL_SETPOINT(..) => "MANUAL_SETPOINT",
            MavMessage::SET_ATTITUDE_TARGET(..) => "SET_ATTITUDE_TARGET",
            MavMessage::ATTITUDE_TARGET(..) => "ATTITUDE_TARGET",
            MavMessage::SET_POSITION_TARGET_LOCAL_NED(..) => "SET_POSITION_TARGET_LOCAL_NED",
            MavMessage::POSITION_TARGET_LOCAL_NED(..) => "POSITION_TARGET_LOCAL_NED",
            MavMessage::SET_POSITION_TARGET_GLOBAL_INT(..) => "SET_POSITION_TARGET_GLOBAL_INT",
            MavMessage::POSITION_TARGET_GLOBAL_INT(..) => "POSITION_TARGET_GLOBAL_INT",
            MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(..) => {
                "LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET"
            }
            MavMessage::HIL_STATE(..) => "HIL_STATE",
            MavMessage::HIL_CONTROLS(..) => "HIL_CONTROLS",
            MavMessage::HIL_RC_INPUTS_RAW(..) => "HIL_RC_INPUTS_RAW",
            MavMessage::HIL_ACTUATOR_CONTROLS(..) => "HIL_ACTUATOR_CONTROLS",
            MavMessage::OPTICAL_FLOW(..) => "OPTICAL_FLOW",
            MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(..) => "GLOBAL_VISION_POSITION_ESTIMATE",
            MavMessage::VISION_POSITION_ESTIMATE(..) => "VISION_POSITION_ESTIMATE",
            MavMessage::VISION_SPEED_ESTIMATE(..) => "VISION_SPEED_ESTIMATE",
            MavMessage::VICON_POSITION_ESTIMATE(..) => "VICON_POSITION_ESTIMATE",
            MavMessage::HIGHRES_IMU(..) => "HIGHRES_IMU",
            MavMessage::OPTICAL_FLOW_RAD(..) => "OPTICAL_FLOW_RAD",
            MavMessage::HIL_SENSOR(..) => "HIL_SENSOR",
            MavMessage::SIM_STATE(..) => "SIM_STATE",
            MavMessage::RADIO_STATUS(..) => "RADIO_STATUS",
            MavMessage::FILE_TRANSFER_PROTOCOL(..) => "FILE_TRANSFER_PROTOCOL",
            MavMessage::TIMESYNC(..) => "TIMESYNC",
            MavMessage::CAMERA_TRIGGER(..) => "CAMERA_TRIGGER",
            MavMessage::HIL_GPS(..) => "HIL_GPS",
            MavMessage::HIL_OPTICAL_FLOW(..) => "HIL_OPTICAL_FLOW",
            MavMessage::HIL_STATE_QUATERNION(..) => "HIL_STATE_QUATERNION",
            MavMessage::SCALED_IMU2(..) => "SCALED_IMU2",
            MavMessage::LOG_REQUEST_LIST(..) => "LOG_REQUEST_LIST",
            MavMessage::LOG_ENTRY(..) => "LOG_ENTRY",
            MavMessage::LOG_REQUEST_DATA(..) => "LOG_REQUEST_DATA",
            MavMessage::LOG_DATA(..) => "LOG_DATA",
            MavMessage::LOG_ERASE(..) => "LOG_ERASE",
            MavMessage::LOG_REQUEST_END(..) => "LOG_REQUEST_END",
            MavMessage::GPS_INJECT_DATA(..) => "GPS_INJECT_DATA",
            MavMessage::GPS2_RAW(..) => "GPS2_RAW",
            MavMessage::POWER_STATUS(..) => "POWER_STATUS",
            MavMessage::SERIAL_CONTROL(..) => "SERIAL_CONTROL",
            MavMessage::GPS_RTK(..) => "GPS_RTK",
            MavMessage::GPS2_RTK(..) => "GPS2_RTK",
            MavMessage::SCALED_IMU3(..) => "SCALED_IMU3",
            MavMessage::DATA_TRANSMISSION_HANDSHAKE(..) => "DATA_TRANSMISSION_HANDSHAKE",
            MavMessage::ENCAPSULATED_DATA(..) => "ENCAPSULATED_DATA",
            MavMessage::DISTANCE_SENSOR(..) => "DISTANCE_SENSOR",
            MavMessage::TERRAIN_REQUEST(..) => "TERRAIN_REQUEST",
            MavMessage::TERRAIN_DATA(..) => "TERRAIN_DATA",
            MavMessage::TERRAIN_CHECK(..) => "TERRAIN_CHECK",
            MavMessage::TERRAIN_REPORT(..) => "TERRAIN_REPORT",
            MavMessage::SCALED_PRESSURE2(..) => "SCALED_PRESSURE2",
            MavMessage::ATT_POS_MOCAP(..) => "ATT_POS_MOCAP",
            MavMessage::SET_ACTUATOR_CONTROL_TARGET(..) => "SET_ACTUATOR_CONTROL_TARGET",
            MavMessage::ACTUATOR_CONTROL_TARGET(..) => "ACTUATOR_CONTROL_TARGET",
            MavMessage::ALTITUDE(..) => "ALTITUDE",
            MavMessage::RESOURCE_REQUEST(..) => "RESOURCE_REQUEST",
            MavMessage::SCALED_PRESSURE3(..) => "SCALED_PRESSURE3",
            MavMessage::FOLLOW_TARGET(..) => "FOLLOW_TARGET",
            MavMessage::CONTROL_SYSTEM_STATE(..) => "CONTROL_SYSTEM_STATE",
            MavMessage::BATTERY_STATUS(..) => "BATTERY_STATUS",
            MavMessage::AUTOPILOT_VERSION(..) => "AUTOPILOT_VERSION",
            MavMessage::LANDING_TARGET(..) => "LANDING_TARGET",
            MavMessage::FENCE_STATUS(..) => "FENCE_STATUS",
            MavMessage::ESTIMATOR_STATUS(..) => "ESTIMATOR_STATUS",
            MavMessage::WIND_COV(..) => "WIND_COV",
            MavMessage::GPS_INPUT(..) => "GPS_INPUT",
            MavMessage::GPS_RTCM_DATA(..) => "GPS_RTCM_DATA",
            MavMessage::HIGH_LATENCY(..) => "HIGH_LATENCY",
            MavMessage::HIGH_LATENCY2(..) => "HIGH_LATENCY2",
            MavMessage::VIBRATION(..) => "VIBRATION",
            MavMessage::HOME_POSITION(..) => "HOME_POSITION",
            MavMessage::SET_HOME_POSITION(..) => "SET_HOME_POSITION",
            MavMessage::MESSAGE_INTERVAL(..) => "MESSAGE_INTERVAL",
            MavMessage::EXTENDED_SYS_STATE(..) => "EXTENDED_SYS_STATE",
            MavMessage::ADSB_VEHICLE(..) => "ADSB_VEHICLE",
            MavMessage::COLLISION(..) => "COLLISION",
            MavMessage::V2_EXTENSION(..) => "V2_EXTENSION",
            MavMessage::MEMORY_VECT(..) => "MEMORY_VECT",
            MavMessage::DEBUG_VECT(..) => "DEBUG_VECT",
            MavMessage::NAMED_VALUE_FLOAT(..) => "NAMED_VALUE_FLOAT",
            MavMessage::NAMED_VALUE_INT(..) => "NAMED_VALUE_INT",
            MavMessage::STATUSTEXT(..) => "STATUSTEXT",
            MavMessage::DEBUG(..) => "DEBUG",
            MavMessage::SETUP_SIGNING(..) => "SETUP_SIGNING",
            MavMessage::BUTTON_CHANGE(..) => "BUTTON_CHANGE",
            MavMessage::PLAY_TUNE(..) => "PLAY_TUNE",
            MavMessage::CAMERA_INFORMATION(..) => "CAMERA_INFORMATION",
            MavMessage::CAMERA_SETTINGS(..) => "CAMERA_SETTINGS",
            MavMessage::STORAGE_INFORMATION(..) => "STORAGE_INFORMATION",
            MavMessage::CAMERA_CAPTURE_STATUS(..) => "CAMERA_CAPTURE_STATUS",
            MavMessage::CAMERA_IMAGE_CAPTURED(..) => "CAMERA_IMAGE_CAPTURED",
            MavMessage::FLIGHT_INFORMATION(..) => "FLIGHT_INFORMATION",
            MavMessage::MOUNT_ORIENTATION(..) => "MOUNT_ORIENTATION",
            MavMessage::LOGGING_DATA(..) => "LOGGING_DATA",
            MavMessage::LOGGING_DATA_ACKED(..) => "LOGGING_DATA_ACKED",
            MavMessage::LOGGING_ACK(..) => "LOGGING_ACK",
            MavMessage::VIDEO_STREAM_INFORMATION(..) => "VIDEO_STREAM_INFORMATION",
            MavMessage::VIDEO_STREAM_STATUS(..) => "VIDEO_STREAM_STATUS",
            MavMessage::GIMBAL_MANAGER_INFORMATION(..) => "GIMBAL_MANAGER_INFORMATION",
            MavMessage::GIMBAL_MANAGER_STATUS(..) => "GIMBAL_MANAGER_STATUS",
            MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(..) => "GIMBAL_MANAGER_SET_ATTITUDE",
            MavMessage::GIMBAL_DEVICE_INFORMATION(..) => "GIMBAL_DEVICE_INFORMATION",
            MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(..) => "GIMBAL_DEVICE_SET_ATTITUDE",
            MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(..) => "GIMBAL_DEVICE_ATTITUDE_STATUS",
            MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(..) => {
                "AUTOPILOT_STATE_FOR_GIMBAL_DEVICE"
            }
            MavMessage::GIMBAL_MANAGER_SET_TILTPAN(..) => "GIMBAL_MANAGER_SET_TILTPAN",
            MavMessage::WIFI_CONFIG_AP(..) => "WIFI_CONFIG_AP",
            MavMessage::PROTOCOL_VERSION(..) => "PROTOCOL_VERSION",
            MavMessage::AIS_VESSEL(..) => "AIS_VESSEL",
            MavMessage::UAVCAN_NODE_STATUS(..) => "UAVCAN_NODE_STATUS",
            MavMessage::UAVCAN_NODE_INFO(..) => "UAVCAN_NODE_INFO",
            MavMessage::PARAM_EXT_REQUEST_READ(..) => "PARAM_EXT_REQUEST_READ",
            MavMessage::PARAM_EXT_REQUEST_LIST(..) => "PARAM_EXT_REQUEST_LIST",
            MavMessage::PARAM_EXT_VALUE(..) => "PARAM_EXT_VALUE",
            MavMessage::PARAM_EXT_SET(..) => "PARAM_EXT_SET",
            MavMessage::PARAM_EXT_ACK(..) => "PARAM_EXT_ACK",
            MavMessage::OBSTACLE_DISTANCE(..) => "OBSTACLE_DISTANCE",
            MavMessage::ODOMETRY(..) => "ODOMETRY",
            MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(..) => {
                "TRAJECTORY_REPRESENTATION_WAYPOINTS"
            }
            MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(..) => "TRAJECTORY_REPRESENTATION_BEZIER",
            MavMessage::CELLULAR_STATUS(..) => "CELLULAR_STATUS",
            MavMessage::ISBD_LINK_STATUS(..) => "ISBD_LINK_STATUS",
            MavMessage::CELLULAR_CONFIG(..) => "CELLULAR_CONFIG",
            MavMessage::RAW_RPM(..) => "RAW_RPM",
            MavMessage::UTM_GLOBAL_POSITION(..) => "UTM_GLOBAL_POSITION",
            MavMessage::DEBUG_FLOAT_ARRAY(..) => "DEBUG_FLOAT_ARRAY",
            MavMessage::ORBIT_EXECUTION_STATUS(..) => "ORBIT_EXECUTION_STATUS",
            MavMessage::SMART_BATTERY_INFO(..) => "SMART_BATTERY_INFO",
            MavMessage::SMART_BATTERY_STATUS(..) => "SMART_BATTERY_STATUS",
            MavMessage::GENERATOR_STATUS(..) => "GENERATOR_STATUS",
            MavMessage::ACTUATOR_OUTPUT_STATUS(..) => "ACTUATOR_OUTPUT_STATUS",
            MavMessage::TIME_ESTIMATE_TO_TARGET(..) => "TIME_ESTIMATE_TO_TARGET",
            MavMessage::TUNNEL(..) => "TUNNEL",
            MavMessage::ONBOARD_COMPUTER_STATUS(..) => "ONBOARD_COMPUTER_STATUS",
            MavMessage::COMPONENT_INFORMATION(..) => "COMPONENT_INFORMATION",
            MavMessage::PLAY_TUNE_V2(..) => "PLAY_TUNE_V2",
            MavMessage::SUPPORTED_TUNES(..) => "SUPPORTED_TUNES",
            MavMessage::WHEEL_DISTANCE(..) => "WHEEL_DISTANCE",
            MavMessage::OPEN_DRONE_ID_BASIC_ID(..) => "OPEN_DRONE_ID_BASIC_ID",
            MavMessage::OPEN_DRONE_ID_LOCATION(..) => "OPEN_DRONE_ID_LOCATION",
            MavMessage::OPEN_DRONE_ID_AUTHENTICATION(..) => "OPEN_DRONE_ID_AUTHENTICATION",
            MavMessage::OPEN_DRONE_ID_SELF_ID(..) => "OPEN_DRONE_ID_SELF_ID",
            MavMessage::OPEN_DRONE_ID_SYSTEM(..) => "OPEN_DRONE_ID_SYSTEM",
            MavMessage::OPEN_DRONE_ID_OPERATOR_ID(..) => "OPEN_DRONE_ID_OPERATOR_ID",
            MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(..) => "OPEN_DRONE_ID_MESSAGE_PACK",
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::HEARTBEAT(..) => 0,
            MavMessage::SYS_STATUS(..) => 1,
            MavMessage::SYSTEM_TIME(..) => 2,
            MavMessage::PING(..) => 4,
            MavMessage::CHANGE_OPERATOR_CONTROL(..) => 5,
            MavMessage::CHANGE_OPERATOR_CONTROL_ACK(..) => 6,
            MavMessage::AUTH_KEY(..) => 7,
            MavMessage::LINK_NODE_STATUS(..) => 8,
            MavMessage::SET_MODE(..) => 11,
            MavMessage::PARAM_REQUEST_READ(..) => 20,
            MavMessage::PARAM_REQUEST_LIST(..) => 21,
            MavMessage::PARAM_VALUE(..) => 22,
            MavMessage::PARAM_SET(..) => 23,
            MavMessage::GPS_RAW_INT(..) => 24,
            MavMessage::GPS_STATUS(..) => 25,
            MavMessage::SCALED_IMU(..) => 26,
            MavMessage::RAW_IMU(..) => 27,
            MavMessage::RAW_PRESSURE(..) => 28,
            MavMessage::SCALED_PRESSURE(..) => 29,
            MavMessage::ATTITUDE(..) => 30,
            MavMessage::ATTITUDE_QUATERNION(..) => 31,
            MavMessage::LOCAL_POSITION_NED(..) => 32,
            MavMessage::GLOBAL_POSITION_INT(..) => 33,
            MavMessage::RC_CHANNELS_SCALED(..) => 34,
            MavMessage::RC_CHANNELS_RAW(..) => 35,
            MavMessage::SERVO_OUTPUT_RAW(..) => 36,
            MavMessage::MISSION_REQUEST_PARTIAL_LIST(..) => 37,
            MavMessage::MISSION_WRITE_PARTIAL_LIST(..) => 38,
            MavMessage::MISSION_ITEM(..) => 39,
            MavMessage::MISSION_REQUEST(..) => 40,
            MavMessage::MISSION_SET_CURRENT(..) => 41,
            MavMessage::MISSION_CURRENT(..) => 42,
            MavMessage::MISSION_REQUEST_LIST(..) => 43,
            MavMessage::MISSION_COUNT(..) => 44,
            MavMessage::MISSION_CLEAR_ALL(..) => 45,
            MavMessage::MISSION_ITEM_REACHED(..) => 46,
            MavMessage::MISSION_ACK(..) => 47,
            MavMessage::SET_GPS_GLOBAL_ORIGIN(..) => 48,
            MavMessage::GPS_GLOBAL_ORIGIN(..) => 49,
            MavMessage::PARAM_MAP_RC(..) => 50,
            MavMessage::MISSION_REQUEST_INT(..) => 51,
            MavMessage::MISSION_CHANGED(..) => 52,
            MavMessage::SAFETY_SET_ALLOWED_AREA(..) => 54,
            MavMessage::SAFETY_ALLOWED_AREA(..) => 55,
            MavMessage::ATTITUDE_QUATERNION_COV(..) => 61,
            MavMessage::NAV_CONTROLLER_OUTPUT(..) => 62,
            MavMessage::GLOBAL_POSITION_INT_COV(..) => 63,
            MavMessage::LOCAL_POSITION_NED_COV(..) => 64,
            MavMessage::RC_CHANNELS(..) => 65,
            MavMessage::REQUEST_DATA_STREAM(..) => 66,
            MavMessage::DATA_STREAM(..) => 67,
            MavMessage::MANUAL_CONTROL(..) => 69,
            MavMessage::RC_CHANNELS_OVERRIDE(..) => 70,
            MavMessage::MISSION_ITEM_INT(..) => 73,
            MavMessage::VFR_HUD(..) => 74,
            MavMessage::COMMAND_INT(..) => 75,
            MavMessage::COMMAND_LONG(..) => 76,
            MavMessage::COMMAND_ACK(..) => 77,
            MavMessage::COMMAND_CANCEL(..) => 80,
            MavMessage::MANUAL_SETPOINT(..) => 81,
            MavMessage::SET_ATTITUDE_TARGET(..) => 82,
            MavMessage::ATTITUDE_TARGET(..) => 83,
            MavMessage::SET_POSITION_TARGET_LOCAL_NED(..) => 84,
            MavMessage::POSITION_TARGET_LOCAL_NED(..) => 85,
            MavMessage::SET_POSITION_TARGET_GLOBAL_INT(..) => 86,
            MavMessage::POSITION_TARGET_GLOBAL_INT(..) => 87,
            MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(..) => 89,
            MavMessage::HIL_STATE(..) => 90,
            MavMessage::HIL_CONTROLS(..) => 91,
            MavMessage::HIL_RC_INPUTS_RAW(..) => 92,
            MavMessage::HIL_ACTUATOR_CONTROLS(..) => 93,
            MavMessage::OPTICAL_FLOW(..) => 100,
            MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(..) => 101,
            MavMessage::VISION_POSITION_ESTIMATE(..) => 102,
            MavMessage::VISION_SPEED_ESTIMATE(..) => 103,
            MavMessage::VICON_POSITION_ESTIMATE(..) => 104,
            MavMessage::HIGHRES_IMU(..) => 105,
            MavMessage::OPTICAL_FLOW_RAD(..) => 106,
            MavMessage::HIL_SENSOR(..) => 107,
            MavMessage::SIM_STATE(..) => 108,
            MavMessage::RADIO_STATUS(..) => 109,
            MavMessage::FILE_TRANSFER_PROTOCOL(..) => 110,
            MavMessage::TIMESYNC(..) => 111,
            MavMessage::CAMERA_TRIGGER(..) => 112,
            MavMessage::HIL_GPS(..) => 113,
            MavMessage::HIL_OPTICAL_FLOW(..) => 114,
            MavMessage::HIL_STATE_QUATERNION(..) => 115,
            MavMessage::SCALED_IMU2(..) => 116,
            MavMessage::LOG_REQUEST_LIST(..) => 117,
            MavMessage::LOG_ENTRY(..) => 118,
            MavMessage::LOG_REQUEST_DATA(..) => 119,
            MavMessage::LOG_DATA(..) => 120,
            MavMessage::LOG_ERASE(..) => 121,
            MavMessage::LOG_REQUEST_END(..) => 122,
            MavMessage::GPS_INJECT_DATA(..) => 123,
            MavMessage::GPS2_RAW(..) => 124,
            MavMessage::POWER_STATUS(..) => 125,
            MavMessage::SERIAL_CONTROL(..) => 126,
            MavMessage::GPS_RTK(..) => 127,
            MavMessage::GPS2_RTK(..) => 128,
            MavMessage::SCALED_IMU3(..) => 129,
            MavMessage::DATA_TRANSMISSION_HANDSHAKE(..) => 130,
            MavMessage::ENCAPSULATED_DATA(..) => 131,
            MavMessage::DISTANCE_SENSOR(..) => 132,
            MavMessage::TERRAIN_REQUEST(..) => 133,
            MavMessage::TERRAIN_DATA(..) => 134,
            MavMessage::TERRAIN_CHECK(..) => 135,
            MavMessage::TERRAIN_REPORT(..) => 136,
            MavMessage::SCALED_PRESSURE2(..) => 137,
            MavMessage::ATT_POS_MOCAP(..) => 138,
            MavMessage::SET_ACTUATOR_CONTROL_TARGET(..) => 139,
            MavMessage::ACTUATOR_CONTROL_TARGET(..) => 140,
            MavMessage::ALTITUDE(..) => 141,
            MavMessage::RESOURCE_REQUEST(..) => 142,
            MavMessage::SCALED_PRESSURE3(..) => 143,
            MavMessage::FOLLOW_TARGET(..) => 144,
            MavMessage::CONTROL_SYSTEM_STATE(..) => 146,
            MavMessage::BATTERY_STATUS(..) => 147,
            MavMessage::AUTOPILOT_VERSION(..) => 148,
            MavMessage::LANDING_TARGET(..) => 149,
            MavMessage::FENCE_STATUS(..) => 162,
            MavMessage::ESTIMATOR_STATUS(..) => 230,
            MavMessage::WIND_COV(..) => 231,
            MavMessage::GPS_INPUT(..) => 232,
            MavMessage::GPS_RTCM_DATA(..) => 233,
            MavMessage::HIGH_LATENCY(..) => 234,
            MavMessage::HIGH_LATENCY2(..) => 235,
            MavMessage::VIBRATION(..) => 241,
            MavMessage::HOME_POSITION(..) => 242,
            MavMessage::SET_HOME_POSITION(..) => 243,
            MavMessage::MESSAGE_INTERVAL(..) => 244,
            MavMessage::EXTENDED_SYS_STATE(..) => 245,
            MavMessage::ADSB_VEHICLE(..) => 246,
            MavMessage::COLLISION(..) => 247,
            MavMessage::V2_EXTENSION(..) => 248,
            MavMessage::MEMORY_VECT(..) => 249,
            MavMessage::DEBUG_VECT(..) => 250,
            MavMessage::NAMED_VALUE_FLOAT(..) => 251,
            MavMessage::NAMED_VALUE_INT(..) => 252,
            MavMessage::STATUSTEXT(..) => 253,
            MavMessage::DEBUG(..) => 254,
            MavMessage::SETUP_SIGNING(..) => 256,
            MavMessage::BUTTON_CHANGE(..) => 257,
            MavMessage::PLAY_TUNE(..) => 258,
            MavMessage::CAMERA_INFORMATION(..) => 259,
            MavMessage::CAMERA_SETTINGS(..) => 260,
            MavMessage::STORAGE_INFORMATION(..) => 261,
            MavMessage::CAMERA_CAPTURE_STATUS(..) => 262,
            MavMessage::CAMERA_IMAGE_CAPTURED(..) => 263,
            MavMessage::FLIGHT_INFORMATION(..) => 264,
            MavMessage::MOUNT_ORIENTATION(..) => 265,
            MavMessage::LOGGING_DATA(..) => 266,
            MavMessage::LOGGING_DATA_ACKED(..) => 267,
            MavMessage::LOGGING_ACK(..) => 268,
            MavMessage::VIDEO_STREAM_INFORMATION(..) => 269,
            MavMessage::VIDEO_STREAM_STATUS(..) => 270,
            MavMessage::GIMBAL_MANAGER_INFORMATION(..) => 280,
            MavMessage::GIMBAL_MANAGER_STATUS(..) => 281,
            MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(..) => 282,
            MavMessage::GIMBAL_DEVICE_INFORMATION(..) => 283,
            MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(..) => 284,
            MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(..) => 285,
            MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(..) => 286,
            MavMessage::GIMBAL_MANAGER_SET_TILTPAN(..) => 287,
            MavMessage::WIFI_CONFIG_AP(..) => 299,
            MavMessage::PROTOCOL_VERSION(..) => 300,
            MavMessage::AIS_VESSEL(..) => 301,
            MavMessage::UAVCAN_NODE_STATUS(..) => 310,
            MavMessage::UAVCAN_NODE_INFO(..) => 311,
            MavMessage::PARAM_EXT_REQUEST_READ(..) => 320,
            MavMessage::PARAM_EXT_REQUEST_LIST(..) => 321,
            MavMessage::PARAM_EXT_VALUE(..) => 322,
            MavMessage::PARAM_EXT_SET(..) => 323,
            MavMessage::PARAM_EXT_ACK(..) => 324,
            MavMessage::OBSTACLE_DISTANCE(..) => 330,
            MavMessage::ODOMETRY(..) => 331,
            MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(..) => 332,
            MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(..) => 333,
            MavMessage::CELLULAR_STATUS(..) => 334,
            MavMessage::ISBD_LINK_STATUS(..) => 335,
            MavMessage::CELLULAR_CONFIG(..) => 336,
            MavMessage::RAW_RPM(..) => 339,
            MavMessage::UTM_GLOBAL_POSITION(..) => 340,
            MavMessage::DEBUG_FLOAT_ARRAY(..) => 350,
            MavMessage::ORBIT_EXECUTION_STATUS(..) => 360,
            MavMessage::SMART_BATTERY_INFO(..) => 370,
            MavMessage::SMART_BATTERY_STATUS(..) => 371,
            MavMessage::GENERATOR_STATUS(..) => 373,
            MavMessage::ACTUATOR_OUTPUT_STATUS(..) => 375,
            MavMessage::TIME_ESTIMATE_TO_TARGET(..) => 380,
            MavMessage::TUNNEL(..) => 385,
            MavMessage::ONBOARD_COMPUTER_STATUS(..) => 390,
            MavMessage::COMPONENT_INFORMATION(..) => 395,
            MavMessage::PLAY_TUNE_V2(..) => 400,
            MavMessage::SUPPORTED_TUNES(..) => 401,
            MavMessage::WHEEL_DISTANCE(..) => 9000,
            MavMessage::OPEN_DRONE_ID_BASIC_ID(..) => 12900,
            MavMessage::OPEN_DRONE_ID_LOCATION(..) => 12901,
            MavMessage::OPEN_DRONE_ID_AUTHENTICATION(..) => 12902,
            MavMessage::OPEN_DRONE_ID_SELF_ID(..) => 12903,
            MavMessage::OPEN_DRONE_ID_SYSTEM(..) => 12904,
            MavMessage::OPEN_DRONE_ID_OPERATOR_ID(..) => 12905,
            MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(..) => 12915,
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "HEARTBEAT" => Ok(0),
            "SYS_STATUS" => Ok(1),
            "SYSTEM_TIME" => Ok(2),
            "PING" => Ok(4),
            "CHANGE_OPERATOR_CONTROL" => Ok(5),
            "CHANGE_OPERATOR_CONTROL_ACK" => Ok(6),
            "AUTH_KEY" => Ok(7),
            "LINK_NODE_STATUS" => Ok(8),
            "SET_MODE" => Ok(11),
            "PARAM_REQUEST_READ" => Ok(20),
            "PARAM_REQUEST_LIST" => Ok(21),
            "PARAM_VALUE" => Ok(22),
            "PARAM_SET" => Ok(23),
            "GPS_RAW_INT" => Ok(24),
            "GPS_STATUS" => Ok(25),
            "SCALED_IMU" => Ok(26),
            "RAW_IMU" => Ok(27),
            "RAW_PRESSURE" => Ok(28),
            "SCALED_PRESSURE" => Ok(29),
            "ATTITUDE" => Ok(30),
            "ATTITUDE_QUATERNION" => Ok(31),
            "LOCAL_POSITION_NED" => Ok(32),
            "GLOBAL_POSITION_INT" => Ok(33),
            "RC_CHANNELS_SCALED" => Ok(34),
            "RC_CHANNELS_RAW" => Ok(35),
            "SERVO_OUTPUT_RAW" => Ok(36),
            "MISSION_REQUEST_PARTIAL_LIST" => Ok(37),
            "MISSION_WRITE_PARTIAL_LIST" => Ok(38),
            "MISSION_ITEM" => Ok(39),
            "MISSION_REQUEST" => Ok(40),
            "MISSION_SET_CURRENT" => Ok(41),
            "MISSION_CURRENT" => Ok(42),
            "MISSION_REQUEST_LIST" => Ok(43),
            "MISSION_COUNT" => Ok(44),
            "MISSION_CLEAR_ALL" => Ok(45),
            "MISSION_ITEM_REACHED" => Ok(46),
            "MISSION_ACK" => Ok(47),
            "SET_GPS_GLOBAL_ORIGIN" => Ok(48),
            "GPS_GLOBAL_ORIGIN" => Ok(49),
            "PARAM_MAP_RC" => Ok(50),
            "MISSION_REQUEST_INT" => Ok(51),
            "MISSION_CHANGED" => Ok(52),
            "SAFETY_SET_ALLOWED_AREA" => Ok(54),
            "SAFETY_ALLOWED_AREA" => Ok(55),
            "ATTITUDE_QUATERNION_COV" => Ok(61),
            "NAV_CONTROLLER_OUTPUT" => Ok(62),
            "GLOBAL_POSITION_INT_COV" => Ok(63),
            "LOCAL_POSITION_NED_COV" => Ok(64),
            "RC_CHANNELS" => Ok(65),
            "REQUEST_DATA_STREAM" => Ok(66),
            "DATA_STREAM" => Ok(67),
            "MANUAL_CONTROL" => Ok(69),
            "RC_CHANNELS_OVERRIDE" => Ok(70),
            "MISSION_ITEM_INT" => Ok(73),
            "VFR_HUD" => Ok(74),
            "COMMAND_INT" => Ok(75),
            "COMMAND_LONG" => Ok(76),
            "COMMAND_ACK" => Ok(77),
            "COMMAND_CANCEL" => Ok(80),
            "MANUAL_SETPOINT" => Ok(81),
            "SET_ATTITUDE_TARGET" => Ok(82),
            "ATTITUDE_TARGET" => Ok(83),
            "SET_POSITION_TARGET_LOCAL_NED" => Ok(84),
            "POSITION_TARGET_LOCAL_NED" => Ok(85),
            "SET_POSITION_TARGET_GLOBAL_INT" => Ok(86),
            "POSITION_TARGET_GLOBAL_INT" => Ok(87),
            "LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET" => Ok(89),
            "HIL_STATE" => Ok(90),
            "HIL_CONTROLS" => Ok(91),
            "HIL_RC_INPUTS_RAW" => Ok(92),
            "HIL_ACTUATOR_CONTROLS" => Ok(93),
            "OPTICAL_FLOW" => Ok(100),
            "GLOBAL_VISION_POSITION_ESTIMATE" => Ok(101),
            "VISION_POSITION_ESTIMATE" => Ok(102),
            "VISION_SPEED_ESTIMATE" => Ok(103),
            "VICON_POSITION_ESTIMATE" => Ok(104),
            "HIGHRES_IMU" => Ok(105),
            "OPTICAL_FLOW_RAD" => Ok(106),
            "HIL_SENSOR" => Ok(107),
            "SIM_STATE" => Ok(108),
            "RADIO_STATUS" => Ok(109),
            "FILE_TRANSFER_PROTOCOL" => Ok(110),
            "TIMESYNC" => Ok(111),
            "CAMERA_TRIGGER" => Ok(112),
            "HIL_GPS" => Ok(113),
            "HIL_OPTICAL_FLOW" => Ok(114),
            "HIL_STATE_QUATERNION" => Ok(115),
            "SCALED_IMU2" => Ok(116),
            "LOG_REQUEST_LIST" => Ok(117),
            "LOG_ENTRY" => Ok(118),
            "LOG_REQUEST_DATA" => Ok(119),
            "LOG_DATA" => Ok(120),
            "LOG_ERASE" => Ok(121),
            "LOG_REQUEST_END" => Ok(122),
            "GPS_INJECT_DATA" => Ok(123),
            "GPS2_RAW" => Ok(124),
            "POWER_STATUS" => Ok(125),
            "SERIAL_CONTROL" => Ok(126),
            "GPS_RTK" => Ok(127),
            "GPS2_RTK" => Ok(128),
            "SCALED_IMU3" => Ok(129),
            "DATA_TRANSMISSION_HANDSHAKE" => Ok(130),
            "ENCAPSULATED_DATA" => Ok(131),
            "DISTANCE_SENSOR" => Ok(132),
            "TERRAIN_REQUEST" => Ok(133),
            "TERRAIN_DATA" => Ok(134),
            "TERRAIN_CHECK" => Ok(135),
            "TERRAIN_REPORT" => Ok(136),
            "SCALED_PRESSURE2" => Ok(137),
            "ATT_POS_MOCAP" => Ok(138),
            "SET_ACTUATOR_CONTROL_TARGET" => Ok(139),
            "ACTUATOR_CONTROL_TARGET" => Ok(140),
            "ALTITUDE" => Ok(141),
            "RESOURCE_REQUEST" => Ok(142),
            "SCALED_PRESSURE3" => Ok(143),
            "FOLLOW_TARGET" => Ok(144),
            "CONTROL_SYSTEM_STATE" => Ok(146),
            "BATTERY_STATUS" => Ok(147),
            "AUTOPILOT_VERSION" => Ok(148),
            "LANDING_TARGET" => Ok(149),
            "FENCE_STATUS" => Ok(162),
            "ESTIMATOR_STATUS" => Ok(230),
            "WIND_COV" => Ok(231),
            "GPS_INPUT" => Ok(232),
            "GPS_RTCM_DATA" => Ok(233),
            "HIGH_LATENCY" => Ok(234),
            "HIGH_LATENCY2" => Ok(235),
            "VIBRATION" => Ok(241),
            "HOME_POSITION" => Ok(242),
            "SET_HOME_POSITION" => Ok(243),
            "MESSAGE_INTERVAL" => Ok(244),
            "EXTENDED_SYS_STATE" => Ok(245),
            "ADSB_VEHICLE" => Ok(246),
            "COLLISION" => Ok(247),
            "V2_EXTENSION" => Ok(248),
            "MEMORY_VECT" => Ok(249),
            "DEBUG_VECT" => Ok(250),
            "NAMED_VALUE_FLOAT" => Ok(251),
            "NAMED_VALUE_INT" => Ok(252),
            "STATUSTEXT" => Ok(253),
            "DEBUG" => Ok(254),
            "SETUP_SIGNING" => Ok(256),
            "BUTTON_CHANGE" => Ok(257),
            "PLAY_TUNE" => Ok(258),
            "CAMERA_INFORMATION" => Ok(259),
            "CAMERA_SETTINGS" => Ok(260),
            "STORAGE_INFORMATION" => Ok(261),
            "CAMERA_CAPTURE_STATUS" => Ok(262),
            "CAMERA_IMAGE_CAPTURED" => Ok(263),
            "FLIGHT_INFORMATION" => Ok(264),
            "MOUNT_ORIENTATION" => Ok(265),
            "LOGGING_DATA" => Ok(266),
            "LOGGING_DATA_ACKED" => Ok(267),
            "LOGGING_ACK" => Ok(268),
            "VIDEO_STREAM_INFORMATION" => Ok(269),
            "VIDEO_STREAM_STATUS" => Ok(270),
            "GIMBAL_MANAGER_INFORMATION" => Ok(280),
            "GIMBAL_MANAGER_STATUS" => Ok(281),
            "GIMBAL_MANAGER_SET_ATTITUDE" => Ok(282),
            "GIMBAL_DEVICE_INFORMATION" => Ok(283),
            "GIMBAL_DEVICE_SET_ATTITUDE" => Ok(284),
            "GIMBAL_DEVICE_ATTITUDE_STATUS" => Ok(285),
            "AUTOPILOT_STATE_FOR_GIMBAL_DEVICE" => Ok(286),
            "GIMBAL_MANAGER_SET_TILTPAN" => Ok(287),
            "WIFI_CONFIG_AP" => Ok(299),
            "PROTOCOL_VERSION" => Ok(300),
            "AIS_VESSEL" => Ok(301),
            "UAVCAN_NODE_STATUS" => Ok(310),
            "UAVCAN_NODE_INFO" => Ok(311),
            "PARAM_EXT_REQUEST_READ" => Ok(320),
            "PARAM_EXT_REQUEST_LIST" => Ok(321),
            "PARAM_EXT_VALUE" => Ok(322),
            "PARAM_EXT_SET" => Ok(323),
            "PARAM_EXT_ACK" => Ok(324),
            "OBSTACLE_DISTANCE" => Ok(330),
            "ODOMETRY" => Ok(331),
            "TRAJECTORY_REPRESENTATION_WAYPOINTS" => Ok(332),
            "TRAJECTORY_REPRESENTATION_BEZIER" => Ok(333),
            "CELLULAR_STATUS" => Ok(334),
            "ISBD_LINK_STATUS" => Ok(335),
            "CELLULAR_CONFIG" => Ok(336),
            "RAW_RPM" => Ok(339),
            "UTM_GLOBAL_POSITION" => Ok(340),
            "DEBUG_FLOAT_ARRAY" => Ok(350),
            "ORBIT_EXECUTION_STATUS" => Ok(360),
            "SMART_BATTERY_INFO" => Ok(370),
            "SMART_BATTERY_STATUS" => Ok(371),
            "GENERATOR_STATUS" => Ok(373),
            "ACTUATOR_OUTPUT_STATUS" => Ok(375),
            "TIME_ESTIMATE_TO_TARGET" => Ok(380),
            "TUNNEL" => Ok(385),
            "ONBOARD_COMPUTER_STATUS" => Ok(390),
            "COMPONENT_INFORMATION" => Ok(395),
            "PLAY_TUNE_V2" => Ok(400),
            "SUPPORTED_TUNES" => Ok(401),
            "WHEEL_DISTANCE" => Ok(9000),
            "OPEN_DRONE_ID_BASIC_ID" => Ok(12900),
            "OPEN_DRONE_ID_LOCATION" => Ok(12901),
            "OPEN_DRONE_ID_AUTHENTICATION" => Ok(12902),
            "OPEN_DRONE_ID_SELF_ID" => Ok(12903),
            "OPEN_DRONE_ID_SYSTEM" => Ok(12904),
            "OPEN_DRONE_ID_OPERATOR_ID" => Ok(12905),
            "OPEN_DRONE_ID_MESSAGE_PACK" => Ok(12915),
            _ => Err("Invalid message name."),
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            0 => Ok(MavMessage::HEARTBEAT(HEARTBEAT_DATA::default())),
            1 => Ok(MavMessage::SYS_STATUS(SYS_STATUS_DATA::default())),
            2 => Ok(MavMessage::SYSTEM_TIME(SYSTEM_TIME_DATA::default())),
            4 => Ok(MavMessage::PING(PING_DATA::default())),
            5 => Ok(MavMessage::CHANGE_OPERATOR_CONTROL(
                CHANGE_OPERATOR_CONTROL_DATA::default(),
            )),
            6 => Ok(MavMessage::CHANGE_OPERATOR_CONTROL_ACK(
                CHANGE_OPERATOR_CONTROL_ACK_DATA::default(),
            )),
            7 => Ok(MavMessage::AUTH_KEY(AUTH_KEY_DATA::default())),
            8 => Ok(MavMessage::LINK_NODE_STATUS(
                LINK_NODE_STATUS_DATA::default(),
            )),
            11 => Ok(MavMessage::SET_MODE(SET_MODE_DATA::default())),
            20 => Ok(MavMessage::PARAM_REQUEST_READ(
                PARAM_REQUEST_READ_DATA::default(),
            )),
            21 => Ok(MavMessage::PARAM_REQUEST_LIST(
                PARAM_REQUEST_LIST_DATA::default(),
            )),
            22 => Ok(MavMessage::PARAM_VALUE(PARAM_VALUE_DATA::default())),
            23 => Ok(MavMessage::PARAM_SET(PARAM_SET_DATA::default())),
            24 => Ok(MavMessage::GPS_RAW_INT(GPS_RAW_INT_DATA::default())),
            25 => Ok(MavMessage::GPS_STATUS(GPS_STATUS_DATA::default())),
            26 => Ok(MavMessage::SCALED_IMU(SCALED_IMU_DATA::default())),
            27 => Ok(MavMessage::RAW_IMU(RAW_IMU_DATA::default())),
            28 => Ok(MavMessage::RAW_PRESSURE(RAW_PRESSURE_DATA::default())),
            29 => Ok(MavMessage::SCALED_PRESSURE(SCALED_PRESSURE_DATA::default())),
            30 => Ok(MavMessage::ATTITUDE(ATTITUDE_DATA::default())),
            31 => Ok(MavMessage::ATTITUDE_QUATERNION(
                ATTITUDE_QUATERNION_DATA::default(),
            )),
            32 => Ok(MavMessage::LOCAL_POSITION_NED(
                LOCAL_POSITION_NED_DATA::default(),
            )),
            33 => Ok(MavMessage::GLOBAL_POSITION_INT(
                GLOBAL_POSITION_INT_DATA::default(),
            )),
            34 => Ok(MavMessage::RC_CHANNELS_SCALED(
                RC_CHANNELS_SCALED_DATA::default(),
            )),
            35 => Ok(MavMessage::RC_CHANNELS_RAW(RC_CHANNELS_RAW_DATA::default())),
            36 => Ok(MavMessage::SERVO_OUTPUT_RAW(
                SERVO_OUTPUT_RAW_DATA::default(),
            )),
            37 => Ok(MavMessage::MISSION_REQUEST_PARTIAL_LIST(
                MISSION_REQUEST_PARTIAL_LIST_DATA::default(),
            )),
            38 => Ok(MavMessage::MISSION_WRITE_PARTIAL_LIST(
                MISSION_WRITE_PARTIAL_LIST_DATA::default(),
            )),
            39 => Ok(MavMessage::MISSION_ITEM(MISSION_ITEM_DATA::default())),
            40 => Ok(MavMessage::MISSION_REQUEST(MISSION_REQUEST_DATA::default())),
            41 => Ok(MavMessage::MISSION_SET_CURRENT(
                MISSION_SET_CURRENT_DATA::default(),
            )),
            42 => Ok(MavMessage::MISSION_CURRENT(MISSION_CURRENT_DATA::default())),
            43 => Ok(MavMessage::MISSION_REQUEST_LIST(
                MISSION_REQUEST_LIST_DATA::default(),
            )),
            44 => Ok(MavMessage::MISSION_COUNT(MISSION_COUNT_DATA::default())),
            45 => Ok(MavMessage::MISSION_CLEAR_ALL(
                MISSION_CLEAR_ALL_DATA::default(),
            )),
            46 => Ok(MavMessage::MISSION_ITEM_REACHED(
                MISSION_ITEM_REACHED_DATA::default(),
            )),
            47 => Ok(MavMessage::MISSION_ACK(MISSION_ACK_DATA::default())),
            48 => Ok(MavMessage::SET_GPS_GLOBAL_ORIGIN(
                SET_GPS_GLOBAL_ORIGIN_DATA::default(),
            )),
            49 => Ok(MavMessage::GPS_GLOBAL_ORIGIN(
                GPS_GLOBAL_ORIGIN_DATA::default(),
            )),
            50 => Ok(MavMessage::PARAM_MAP_RC(PARAM_MAP_RC_DATA::default())),
            51 => Ok(MavMessage::MISSION_REQUEST_INT(
                MISSION_REQUEST_INT_DATA::default(),
            )),
            52 => Ok(MavMessage::MISSION_CHANGED(MISSION_CHANGED_DATA::default())),
            54 => Ok(MavMessage::SAFETY_SET_ALLOWED_AREA(
                SAFETY_SET_ALLOWED_AREA_DATA::default(),
            )),
            55 => Ok(MavMessage::SAFETY_ALLOWED_AREA(
                SAFETY_ALLOWED_AREA_DATA::default(),
            )),
            61 => Ok(MavMessage::ATTITUDE_QUATERNION_COV(
                ATTITUDE_QUATERNION_COV_DATA::default(),
            )),
            62 => Ok(MavMessage::NAV_CONTROLLER_OUTPUT(
                NAV_CONTROLLER_OUTPUT_DATA::default(),
            )),
            63 => Ok(MavMessage::GLOBAL_POSITION_INT_COV(
                GLOBAL_POSITION_INT_COV_DATA::default(),
            )),
            64 => Ok(MavMessage::LOCAL_POSITION_NED_COV(
                LOCAL_POSITION_NED_COV_DATA::default(),
            )),
            65 => Ok(MavMessage::RC_CHANNELS(RC_CHANNELS_DATA::default())),
            66 => Ok(MavMessage::REQUEST_DATA_STREAM(
                REQUEST_DATA_STREAM_DATA::default(),
            )),
            67 => Ok(MavMessage::DATA_STREAM(DATA_STREAM_DATA::default())),
            69 => Ok(MavMessage::MANUAL_CONTROL(MANUAL_CONTROL_DATA::default())),
            70 => Ok(MavMessage::RC_CHANNELS_OVERRIDE(
                RC_CHANNELS_OVERRIDE_DATA::default(),
            )),
            73 => Ok(MavMessage::MISSION_ITEM_INT(
                MISSION_ITEM_INT_DATA::default(),
            )),
            74 => Ok(MavMessage::VFR_HUD(VFR_HUD_DATA::default())),
            75 => Ok(MavMessage::COMMAND_INT(COMMAND_INT_DATA::default())),
            76 => Ok(MavMessage::COMMAND_LONG(COMMAND_LONG_DATA::default())),
            77 => Ok(MavMessage::COMMAND_ACK(COMMAND_ACK_DATA::default())),
            80 => Ok(MavMessage::COMMAND_CANCEL(COMMAND_CANCEL_DATA::default())),
            81 => Ok(MavMessage::MANUAL_SETPOINT(MANUAL_SETPOINT_DATA::default())),
            82 => Ok(MavMessage::SET_ATTITUDE_TARGET(
                SET_ATTITUDE_TARGET_DATA::default(),
            )),
            83 => Ok(MavMessage::ATTITUDE_TARGET(ATTITUDE_TARGET_DATA::default())),
            84 => Ok(MavMessage::SET_POSITION_TARGET_LOCAL_NED(
                SET_POSITION_TARGET_LOCAL_NED_DATA::default(),
            )),
            85 => Ok(MavMessage::POSITION_TARGET_LOCAL_NED(
                POSITION_TARGET_LOCAL_NED_DATA::default(),
            )),
            86 => Ok(MavMessage::SET_POSITION_TARGET_GLOBAL_INT(
                SET_POSITION_TARGET_GLOBAL_INT_DATA::default(),
            )),
            87 => Ok(MavMessage::POSITION_TARGET_GLOBAL_INT(
                POSITION_TARGET_GLOBAL_INT_DATA::default(),
            )),
            89 => Ok(MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(
                LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET_DATA::default(),
            )),
            90 => Ok(MavMessage::HIL_STATE(HIL_STATE_DATA::default())),
            91 => Ok(MavMessage::HIL_CONTROLS(HIL_CONTROLS_DATA::default())),
            92 => Ok(MavMessage::HIL_RC_INPUTS_RAW(
                HIL_RC_INPUTS_RAW_DATA::default(),
            )),
            93 => Ok(MavMessage::HIL_ACTUATOR_CONTROLS(
                HIL_ACTUATOR_CONTROLS_DATA::default(),
            )),
            100 => Ok(MavMessage::OPTICAL_FLOW(OPTICAL_FLOW_DATA::default())),
            101 => Ok(MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(
                GLOBAL_VISION_POSITION_ESTIMATE_DATA::default(),
            )),
            102 => Ok(MavMessage::VISION_POSITION_ESTIMATE(
                VISION_POSITION_ESTIMATE_DATA::default(),
            )),
            103 => Ok(MavMessage::VISION_SPEED_ESTIMATE(
                VISION_SPEED_ESTIMATE_DATA::default(),
            )),
            104 => Ok(MavMessage::VICON_POSITION_ESTIMATE(
                VICON_POSITION_ESTIMATE_DATA::default(),
            )),
            105 => Ok(MavMessage::HIGHRES_IMU(HIGHRES_IMU_DATA::default())),
            106 => Ok(MavMessage::OPTICAL_FLOW_RAD(
                OPTICAL_FLOW_RAD_DATA::default(),
            )),
            107 => Ok(MavMessage::HIL_SENSOR(HIL_SENSOR_DATA::default())),
            108 => Ok(MavMessage::SIM_STATE(SIM_STATE_DATA::default())),
            109 => Ok(MavMessage::RADIO_STATUS(RADIO_STATUS_DATA::default())),
            110 => Ok(MavMessage::FILE_TRANSFER_PROTOCOL(
                FILE_TRANSFER_PROTOCOL_DATA::default(),
            )),
            111 => Ok(MavMessage::TIMESYNC(TIMESYNC_DATA::default())),
            112 => Ok(MavMessage::CAMERA_TRIGGER(CAMERA_TRIGGER_DATA::default())),
            113 => Ok(MavMessage::HIL_GPS(HIL_GPS_DATA::default())),
            114 => Ok(MavMessage::HIL_OPTICAL_FLOW(
                HIL_OPTICAL_FLOW_DATA::default(),
            )),
            115 => Ok(MavMessage::HIL_STATE_QUATERNION(
                HIL_STATE_QUATERNION_DATA::default(),
            )),
            116 => Ok(MavMessage::SCALED_IMU2(SCALED_IMU2_DATA::default())),
            117 => Ok(MavMessage::LOG_REQUEST_LIST(
                LOG_REQUEST_LIST_DATA::default(),
            )),
            118 => Ok(MavMessage::LOG_ENTRY(LOG_ENTRY_DATA::default())),
            119 => Ok(MavMessage::LOG_REQUEST_DATA(
                LOG_REQUEST_DATA_DATA::default(),
            )),
            120 => Ok(MavMessage::LOG_DATA(LOG_DATA_DATA::default())),
            121 => Ok(MavMessage::LOG_ERASE(LOG_ERASE_DATA::default())),
            122 => Ok(MavMessage::LOG_REQUEST_END(LOG_REQUEST_END_DATA::default())),
            123 => Ok(MavMessage::GPS_INJECT_DATA(GPS_INJECT_DATA_DATA::default())),
            124 => Ok(MavMessage::GPS2_RAW(GPS2_RAW_DATA::default())),
            125 => Ok(MavMessage::POWER_STATUS(POWER_STATUS_DATA::default())),
            126 => Ok(MavMessage::SERIAL_CONTROL(SERIAL_CONTROL_DATA::default())),
            127 => Ok(MavMessage::GPS_RTK(GPS_RTK_DATA::default())),
            128 => Ok(MavMessage::GPS2_RTK(GPS2_RTK_DATA::default())),
            129 => Ok(MavMessage::SCALED_IMU3(SCALED_IMU3_DATA::default())),
            130 => Ok(MavMessage::DATA_TRANSMISSION_HANDSHAKE(
                DATA_TRANSMISSION_HANDSHAKE_DATA::default(),
            )),
            131 => Ok(MavMessage::ENCAPSULATED_DATA(
                ENCAPSULATED_DATA_DATA::default(),
            )),
            132 => Ok(MavMessage::DISTANCE_SENSOR(DISTANCE_SENSOR_DATA::default())),
            133 => Ok(MavMessage::TERRAIN_REQUEST(TERRAIN_REQUEST_DATA::default())),
            134 => Ok(MavMessage::TERRAIN_DATA(TERRAIN_DATA_DATA::default())),
            135 => Ok(MavMessage::TERRAIN_CHECK(TERRAIN_CHECK_DATA::default())),
            136 => Ok(MavMessage::TERRAIN_REPORT(TERRAIN_REPORT_DATA::default())),
            137 => Ok(MavMessage::SCALED_PRESSURE2(
                SCALED_PRESSURE2_DATA::default(),
            )),
            138 => Ok(MavMessage::ATT_POS_MOCAP(ATT_POS_MOCAP_DATA::default())),
            139 => Ok(MavMessage::SET_ACTUATOR_CONTROL_TARGET(
                SET_ACTUATOR_CONTROL_TARGET_DATA::default(),
            )),
            140 => Ok(MavMessage::ACTUATOR_CONTROL_TARGET(
                ACTUATOR_CONTROL_TARGET_DATA::default(),
            )),
            141 => Ok(MavMessage::ALTITUDE(ALTITUDE_DATA::default())),
            142 => Ok(MavMessage::RESOURCE_REQUEST(
                RESOURCE_REQUEST_DATA::default(),
            )),
            143 => Ok(MavMessage::SCALED_PRESSURE3(
                SCALED_PRESSURE3_DATA::default(),
            )),
            144 => Ok(MavMessage::FOLLOW_TARGET(FOLLOW_TARGET_DATA::default())),
            146 => Ok(MavMessage::CONTROL_SYSTEM_STATE(
                CONTROL_SYSTEM_STATE_DATA::default(),
            )),
            147 => Ok(MavMessage::BATTERY_STATUS(BATTERY_STATUS_DATA::default())),
            148 => Ok(MavMessage::AUTOPILOT_VERSION(
                AUTOPILOT_VERSION_DATA::default(),
            )),
            149 => Ok(MavMessage::LANDING_TARGET(LANDING_TARGET_DATA::default())),
            162 => Ok(MavMessage::FENCE_STATUS(FENCE_STATUS_DATA::default())),
            230 => Ok(MavMessage::ESTIMATOR_STATUS(
                ESTIMATOR_STATUS_DATA::default(),
            )),
            231 => Ok(MavMessage::WIND_COV(WIND_COV_DATA::default())),
            232 => Ok(MavMessage::GPS_INPUT(GPS_INPUT_DATA::default())),
            233 => Ok(MavMessage::GPS_RTCM_DATA(GPS_RTCM_DATA_DATA::default())),
            234 => Ok(MavMessage::HIGH_LATENCY(HIGH_LATENCY_DATA::default())),
            235 => Ok(MavMessage::HIGH_LATENCY2(HIGH_LATENCY2_DATA::default())),
            241 => Ok(MavMessage::VIBRATION(VIBRATION_DATA::default())),
            242 => Ok(MavMessage::HOME_POSITION(HOME_POSITION_DATA::default())),
            243 => Ok(MavMessage::SET_HOME_POSITION(
                SET_HOME_POSITION_DATA::default(),
            )),
            244 => Ok(MavMessage::MESSAGE_INTERVAL(
                MESSAGE_INTERVAL_DATA::default(),
            )),
            245 => Ok(MavMessage::EXTENDED_SYS_STATE(
                EXTENDED_SYS_STATE_DATA::default(),
            )),
            246 => Ok(MavMessage::ADSB_VEHICLE(ADSB_VEHICLE_DATA::default())),
            247 => Ok(MavMessage::COLLISION(COLLISION_DATA::default())),
            248 => Ok(MavMessage::V2_EXTENSION(V2_EXTENSION_DATA::default())),
            249 => Ok(MavMessage::MEMORY_VECT(MEMORY_VECT_DATA::default())),
            250 => Ok(MavMessage::DEBUG_VECT(DEBUG_VECT_DATA::default())),
            251 => Ok(MavMessage::NAMED_VALUE_FLOAT(
                NAMED_VALUE_FLOAT_DATA::default(),
            )),
            252 => Ok(MavMessage::NAMED_VALUE_INT(NAMED_VALUE_INT_DATA::default())),
            253 => Ok(MavMessage::STATUSTEXT(STATUSTEXT_DATA::default())),
            254 => Ok(MavMessage::DEBUG(DEBUG_DATA::default())),
            256 => Ok(MavMessage::SETUP_SIGNING(SETUP_SIGNING_DATA::default())),
            257 => Ok(MavMessage::BUTTON_CHANGE(BUTTON_CHANGE_DATA::default())),
            258 => Ok(MavMessage::PLAY_TUNE(PLAY_TUNE_DATA::default())),
            259 => Ok(MavMessage::CAMERA_INFORMATION(
                CAMERA_INFORMATION_DATA::default(),
            )),
            260 => Ok(MavMessage::CAMERA_SETTINGS(CAMERA_SETTINGS_DATA::default())),
            261 => Ok(MavMessage::STORAGE_INFORMATION(
                STORAGE_INFORMATION_DATA::default(),
            )),
            262 => Ok(MavMessage::CAMERA_CAPTURE_STATUS(
                CAMERA_CAPTURE_STATUS_DATA::default(),
            )),
            263 => Ok(MavMessage::CAMERA_IMAGE_CAPTURED(
                CAMERA_IMAGE_CAPTURED_DATA::default(),
            )),
            264 => Ok(MavMessage::FLIGHT_INFORMATION(
                FLIGHT_INFORMATION_DATA::default(),
            )),
            265 => Ok(MavMessage::MOUNT_ORIENTATION(
                MOUNT_ORIENTATION_DATA::default(),
            )),
            266 => Ok(MavMessage::LOGGING_DATA(LOGGING_DATA_DATA::default())),
            267 => Ok(MavMessage::LOGGING_DATA_ACKED(
                LOGGING_DATA_ACKED_DATA::default(),
            )),
            268 => Ok(MavMessage::LOGGING_ACK(LOGGING_ACK_DATA::default())),
            269 => Ok(MavMessage::VIDEO_STREAM_INFORMATION(
                VIDEO_STREAM_INFORMATION_DATA::default(),
            )),
            270 => Ok(MavMessage::VIDEO_STREAM_STATUS(
                VIDEO_STREAM_STATUS_DATA::default(),
            )),
            280 => Ok(MavMessage::GIMBAL_MANAGER_INFORMATION(
                GIMBAL_MANAGER_INFORMATION_DATA::default(),
            )),
            281 => Ok(MavMessage::GIMBAL_MANAGER_STATUS(
                GIMBAL_MANAGER_STATUS_DATA::default(),
            )),
            282 => Ok(MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(
                GIMBAL_MANAGER_SET_ATTITUDE_DATA::default(),
            )),
            283 => Ok(MavMessage::GIMBAL_DEVICE_INFORMATION(
                GIMBAL_DEVICE_INFORMATION_DATA::default(),
            )),
            284 => Ok(MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(
                GIMBAL_DEVICE_SET_ATTITUDE_DATA::default(),
            )),
            285 => Ok(MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(
                GIMBAL_DEVICE_ATTITUDE_STATUS_DATA::default(),
            )),
            286 => Ok(MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(
                AUTOPILOT_STATE_FOR_GIMBAL_DEVICE_DATA::default(),
            )),
            287 => Ok(MavMessage::GIMBAL_MANAGER_SET_TILTPAN(
                GIMBAL_MANAGER_SET_TILTPAN_DATA::default(),
            )),
            299 => Ok(MavMessage::WIFI_CONFIG_AP(WIFI_CONFIG_AP_DATA::default())),
            300 => Ok(MavMessage::PROTOCOL_VERSION(
                PROTOCOL_VERSION_DATA::default(),
            )),
            301 => Ok(MavMessage::AIS_VESSEL(AIS_VESSEL_DATA::default())),
            310 => Ok(MavMessage::UAVCAN_NODE_STATUS(
                UAVCAN_NODE_STATUS_DATA::default(),
            )),
            311 => Ok(MavMessage::UAVCAN_NODE_INFO(
                UAVCAN_NODE_INFO_DATA::default(),
            )),
            320 => Ok(MavMessage::PARAM_EXT_REQUEST_READ(
                PARAM_EXT_REQUEST_READ_DATA::default(),
            )),
            321 => Ok(MavMessage::PARAM_EXT_REQUEST_LIST(
                PARAM_EXT_REQUEST_LIST_DATA::default(),
            )),
            322 => Ok(MavMessage::PARAM_EXT_VALUE(PARAM_EXT_VALUE_DATA::default())),
            323 => Ok(MavMessage::PARAM_EXT_SET(PARAM_EXT_SET_DATA::default())),
            324 => Ok(MavMessage::PARAM_EXT_ACK(PARAM_EXT_ACK_DATA::default())),
            330 => Ok(MavMessage::OBSTACLE_DISTANCE(
                OBSTACLE_DISTANCE_DATA::default(),
            )),
            331 => Ok(MavMessage::ODOMETRY(ODOMETRY_DATA::default())),
            332 => Ok(MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(
                TRAJECTORY_REPRESENTATION_WAYPOINTS_DATA::default(),
            )),
            333 => Ok(MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(
                TRAJECTORY_REPRESENTATION_BEZIER_DATA::default(),
            )),
            334 => Ok(MavMessage::CELLULAR_STATUS(CELLULAR_STATUS_DATA::default())),
            335 => Ok(MavMessage::ISBD_LINK_STATUS(
                ISBD_LINK_STATUS_DATA::default(),
            )),
            336 => Ok(MavMessage::CELLULAR_CONFIG(CELLULAR_CONFIG_DATA::default())),
            339 => Ok(MavMessage::RAW_RPM(RAW_RPM_DATA::default())),
            340 => Ok(MavMessage::UTM_GLOBAL_POSITION(
                UTM_GLOBAL_POSITION_DATA::default(),
            )),
            350 => Ok(MavMessage::DEBUG_FLOAT_ARRAY(
                DEBUG_FLOAT_ARRAY_DATA::default(),
            )),
            360 => Ok(MavMessage::ORBIT_EXECUTION_STATUS(
                ORBIT_EXECUTION_STATUS_DATA::default(),
            )),
            370 => Ok(MavMessage::SMART_BATTERY_INFO(
                SMART_BATTERY_INFO_DATA::default(),
            )),
            371 => Ok(MavMessage::SMART_BATTERY_STATUS(
                SMART_BATTERY_STATUS_DATA::default(),
            )),
            373 => Ok(MavMessage::GENERATOR_STATUS(
                GENERATOR_STATUS_DATA::default(),
            )),
            375 => Ok(MavMessage::ACTUATOR_OUTPUT_STATUS(
                ACTUATOR_OUTPUT_STATUS_DATA::default(),
            )),
            380 => Ok(MavMessage::TIME_ESTIMATE_TO_TARGET(
                TIME_ESTIMATE_TO_TARGET_DATA::default(),
            )),
            385 => Ok(MavMessage::TUNNEL(TUNNEL_DATA::default())),
            390 => Ok(MavMessage::ONBOARD_COMPUTER_STATUS(
                ONBOARD_COMPUTER_STATUS_DATA::default(),
            )),
            395 => Ok(MavMessage::COMPONENT_INFORMATION(
                COMPONENT_INFORMATION_DATA::default(),
            )),
            400 => Ok(MavMessage::PLAY_TUNE_V2(PLAY_TUNE_V2_DATA::default())),
            401 => Ok(MavMessage::SUPPORTED_TUNES(SUPPORTED_TUNES_DATA::default())),
            9000 => Ok(MavMessage::WHEEL_DISTANCE(WHEEL_DISTANCE_DATA::default())),
            12900 => Ok(MavMessage::OPEN_DRONE_ID_BASIC_ID(
                OPEN_DRONE_ID_BASIC_ID_DATA::default(),
            )),
            12901 => Ok(MavMessage::OPEN_DRONE_ID_LOCATION(
                OPEN_DRONE_ID_LOCATION_DATA::default(),
            )),
            12902 => Ok(MavMessage::OPEN_DRONE_ID_AUTHENTICATION(
                OPEN_DRONE_ID_AUTHENTICATION_DATA::default(),
            )),
            12903 => Ok(MavMessage::OPEN_DRONE_ID_SELF_ID(
                OPEN_DRONE_ID_SELF_ID_DATA::default(),
            )),
            12904 => Ok(MavMessage::OPEN_DRONE_ID_SYSTEM(
                OPEN_DRONE_ID_SYSTEM_DATA::default(),
            )),
            12905 => Ok(MavMessage::OPEN_DRONE_ID_OPERATOR_ID(
                OPEN_DRONE_ID_OPERATOR_ID_DATA::default(),
            )),
            12915 => Ok(MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(
                OPEN_DRONE_ID_MESSAGE_PACK_DATA::default(),
            )),
            _ => {
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::HEARTBEAT(ref body) => body.ser(),
            &MavMessage::SYS_STATUS(ref body) => body.ser(),
            &MavMessage::SYSTEM_TIME(ref body) => body.ser(),
            &MavMessage::PING(ref body) => body.ser(),
            &MavMessage::CHANGE_OPERATOR_CONTROL(ref body) => body.ser(),
            &MavMessage::CHANGE_OPERATOR_CONTROL_ACK(ref body) => body.ser(),
            &MavMessage::AUTH_KEY(ref body) => body.ser(),
            &MavMessage::LINK_NODE_STATUS(ref body) => body.ser(),
            &MavMessage::SET_MODE(ref body) => body.ser(),
            &MavMessage::PARAM_REQUEST_READ(ref body) => body.ser(),
            &MavMessage::PARAM_REQUEST_LIST(ref body) => body.ser(),
            &MavMessage::PARAM_VALUE(ref body) => body.ser(),
            &MavMessage::PARAM_SET(ref body) => body.ser(),
            &MavMessage::GPS_RAW_INT(ref body) => body.ser(),
            &MavMessage::GPS_STATUS(ref body) => body.ser(),
            &MavMessage::SCALED_IMU(ref body) => body.ser(),
            &MavMessage::RAW_IMU(ref body) => body.ser(),
            &MavMessage::RAW_PRESSURE(ref body) => body.ser(),
            &MavMessage::SCALED_PRESSURE(ref body) => body.ser(),
            &MavMessage::ATTITUDE(ref body) => body.ser(),
            &MavMessage::ATTITUDE_QUATERNION(ref body) => body.ser(),
            &MavMessage::LOCAL_POSITION_NED(ref body) => body.ser(),
            &MavMessage::GLOBAL_POSITION_INT(ref body) => body.ser(),
            &MavMessage::RC_CHANNELS_SCALED(ref body) => body.ser(),
            &MavMessage::RC_CHANNELS_RAW(ref body) => body.ser(),
            &MavMessage::SERVO_OUTPUT_RAW(ref body) => body.ser(),
            &MavMessage::MISSION_REQUEST_PARTIAL_LIST(ref body) => body.ser(),
            &MavMessage::MISSION_WRITE_PARTIAL_LIST(ref body) => body.ser(),
            &MavMessage::MISSION_ITEM(ref body) => body.ser(),
            &MavMessage::MISSION_REQUEST(ref body) => body.ser(),
            &MavMessage::MISSION_SET_CURRENT(ref body) => body.ser(),
            &MavMessage::MISSION_CURRENT(ref body) => body.ser(),
            &MavMessage::MISSION_REQUEST_LIST(ref body) => body.ser(),
            &MavMessage::MISSION_COUNT(ref body) => body.ser(),
            &MavMessage::MISSION_CLEAR_ALL(ref body) => body.ser(),
            &MavMessage::MISSION_ITEM_REACHED(ref body) => body.ser(),
            &MavMessage::MISSION_ACK(ref body) => body.ser(),
            &MavMessage::SET_GPS_GLOBAL_ORIGIN(ref body) => body.ser(),
            &MavMessage::GPS_GLOBAL_ORIGIN(ref body) => body.ser(),
            &MavMessage::PARAM_MAP_RC(ref body) => body.ser(),
            &MavMessage::MISSION_REQUEST_INT(ref body) => body.ser(),
            &MavMessage::MISSION_CHANGED(ref body) => body.ser(),
            &MavMessage::SAFETY_SET_ALLOWED_AREA(ref body) => body.ser(),
            &MavMessage::SAFETY_ALLOWED_AREA(ref body) => body.ser(),
            &MavMessage::ATTITUDE_QUATERNION_COV(ref body) => body.ser(),
            &MavMessage::NAV_CONTROLLER_OUTPUT(ref body) => body.ser(),
            &MavMessage::GLOBAL_POSITION_INT_COV(ref body) => body.ser(),
            &MavMessage::LOCAL_POSITION_NED_COV(ref body) => body.ser(),
            &MavMessage::RC_CHANNELS(ref body) => body.ser(),
            &MavMessage::REQUEST_DATA_STREAM(ref body) => body.ser(),
            &MavMessage::DATA_STREAM(ref body) => body.ser(),
            &MavMessage::MANUAL_CONTROL(ref body) => body.ser(),
            &MavMessage::RC_CHANNELS_OVERRIDE(ref body) => body.ser(),
            &MavMessage::MISSION_ITEM_INT(ref body) => body.ser(),
            &MavMessage::VFR_HUD(ref body) => body.ser(),
            &MavMessage::COMMAND_INT(ref body) => body.ser(),
            &MavMessage::COMMAND_LONG(ref body) => body.ser(),
            &MavMessage::COMMAND_ACK(ref body) => body.ser(),
            &MavMessage::COMMAND_CANCEL(ref body) => body.ser(),
            &MavMessage::MANUAL_SETPOINT(ref body) => body.ser(),
            &MavMessage::SET_ATTITUDE_TARGET(ref body) => body.ser(),
            &MavMessage::ATTITUDE_TARGET(ref body) => body.ser(),
            &MavMessage::SET_POSITION_TARGET_LOCAL_NED(ref body) => body.ser(),
            &MavMessage::POSITION_TARGET_LOCAL_NED(ref body) => body.ser(),
            &MavMessage::SET_POSITION_TARGET_GLOBAL_INT(ref body) => body.ser(),
            &MavMessage::POSITION_TARGET_GLOBAL_INT(ref body) => body.ser(),
            &MavMessage::LOCAL_POSITION_NED_SYSTEM_GLOBAL_OFFSET(ref body) => body.ser(),
            &MavMessage::HIL_STATE(ref body) => body.ser(),
            &MavMessage::HIL_CONTROLS(ref body) => body.ser(),
            &MavMessage::HIL_RC_INPUTS_RAW(ref body) => body.ser(),
            &MavMessage::HIL_ACTUATOR_CONTROLS(ref body) => body.ser(),
            &MavMessage::OPTICAL_FLOW(ref body) => body.ser(),
            &MavMessage::GLOBAL_VISION_POSITION_ESTIMATE(ref body) => body.ser(),
            &MavMessage::VISION_POSITION_ESTIMATE(ref body) => body.ser(),
            &MavMessage::VISION_SPEED_ESTIMATE(ref body) => body.ser(),
            &MavMessage::VICON_POSITION_ESTIMATE(ref body) => body.ser(),
            &MavMessage::HIGHRES_IMU(ref body) => body.ser(),
            &MavMessage::OPTICAL_FLOW_RAD(ref body) => body.ser(),
            &MavMessage::HIL_SENSOR(ref body) => body.ser(),
            &MavMessage::SIM_STATE(ref body) => body.ser(),
            &MavMessage::RADIO_STATUS(ref body) => body.ser(),
            &MavMessage::FILE_TRANSFER_PROTOCOL(ref body) => body.ser(),
            &MavMessage::TIMESYNC(ref body) => body.ser(),
            &MavMessage::CAMERA_TRIGGER(ref body) => body.ser(),
            &MavMessage::HIL_GPS(ref body) => body.ser(),
            &MavMessage::HIL_OPTICAL_FLOW(ref body) => body.ser(),
            &MavMessage::HIL_STATE_QUATERNION(ref body) => body.ser(),
            &MavMessage::SCALED_IMU2(ref body) => body.ser(),
            &MavMessage::LOG_REQUEST_LIST(ref body) => body.ser(),
            &MavMessage::LOG_ENTRY(ref body) => body.ser(),
            &MavMessage::LOG_REQUEST_DATA(ref body) => body.ser(),
            &MavMessage::LOG_DATA(ref body) => body.ser(),
            &MavMessage::LOG_ERASE(ref body) => body.ser(),
            &MavMessage::LOG_REQUEST_END(ref body) => body.ser(),
            &MavMessage::GPS_INJECT_DATA(ref body) => body.ser(),
            &MavMessage::GPS2_RAW(ref body) => body.ser(),
            &MavMessage::POWER_STATUS(ref body) => body.ser(),
            &MavMessage::SERIAL_CONTROL(ref body) => body.ser(),
            &MavMessage::GPS_RTK(ref body) => body.ser(),
            &MavMessage::GPS2_RTK(ref body) => body.ser(),
            &MavMessage::SCALED_IMU3(ref body) => body.ser(),
            &MavMessage::DATA_TRANSMISSION_HANDSHAKE(ref body) => body.ser(),
            &MavMessage::ENCAPSULATED_DATA(ref body) => body.ser(),
            &MavMessage::DISTANCE_SENSOR(ref body) => body.ser(),
            &MavMessage::TERRAIN_REQUEST(ref body) => body.ser(),
            &MavMessage::TERRAIN_DATA(ref body) => body.ser(),
            &MavMessage::TERRAIN_CHECK(ref body) => body.ser(),
            &MavMessage::TERRAIN_REPORT(ref body) => body.ser(),
            &MavMessage::SCALED_PRESSURE2(ref body) => body.ser(),
            &MavMessage::ATT_POS_MOCAP(ref body) => body.ser(),
            &MavMessage::SET_ACTUATOR_CONTROL_TARGET(ref body) => body.ser(),
            &MavMessage::ACTUATOR_CONTROL_TARGET(ref body) => body.ser(),
            &MavMessage::ALTITUDE(ref body) => body.ser(),
            &MavMessage::RESOURCE_REQUEST(ref body) => body.ser(),
            &MavMessage::SCALED_PRESSURE3(ref body) => body.ser(),
            &MavMessage::FOLLOW_TARGET(ref body) => body.ser(),
            &MavMessage::CONTROL_SYSTEM_STATE(ref body) => body.ser(),
            &MavMessage::BATTERY_STATUS(ref body) => body.ser(),
            &MavMessage::AUTOPILOT_VERSION(ref body) => body.ser(),
            &MavMessage::LANDING_TARGET(ref body) => body.ser(),
            &MavMessage::FENCE_STATUS(ref body) => body.ser(),
            &MavMessage::ESTIMATOR_STATUS(ref body) => body.ser(),
            &MavMessage::WIND_COV(ref body) => body.ser(),
            &MavMessage::GPS_INPUT(ref body) => body.ser(),
            &MavMessage::GPS_RTCM_DATA(ref body) => body.ser(),
            &MavMessage::HIGH_LATENCY(ref body) => body.ser(),
            &MavMessage::HIGH_LATENCY2(ref body) => body.ser(),
            &MavMessage::VIBRATION(ref body) => body.ser(),
            &MavMessage::HOME_POSITION(ref body) => body.ser(),
            &MavMessage::SET_HOME_POSITION(ref body) => body.ser(),
            &MavMessage::MESSAGE_INTERVAL(ref body) => body.ser(),
            &MavMessage::EXTENDED_SYS_STATE(ref body) => body.ser(),
            &MavMessage::ADSB_VEHICLE(ref body) => body.ser(),
            &MavMessage::COLLISION(ref body) => body.ser(),
            &MavMessage::V2_EXTENSION(ref body) => body.ser(),
            &MavMessage::MEMORY_VECT(ref body) => body.ser(),
            &MavMessage::DEBUG_VECT(ref body) => body.ser(),
            &MavMessage::NAMED_VALUE_FLOAT(ref body) => body.ser(),
            &MavMessage::NAMED_VALUE_INT(ref body) => body.ser(),
            &MavMessage::STATUSTEXT(ref body) => body.ser(),
            &MavMessage::DEBUG(ref body) => body.ser(),
            &MavMessage::SETUP_SIGNING(ref body) => body.ser(),
            &MavMessage::BUTTON_CHANGE(ref body) => body.ser(),
            &MavMessage::PLAY_TUNE(ref body) => body.ser(),
            &MavMessage::CAMERA_INFORMATION(ref body) => body.ser(),
            &MavMessage::CAMERA_SETTINGS(ref body) => body.ser(),
            &MavMessage::STORAGE_INFORMATION(ref body) => body.ser(),
            &MavMessage::CAMERA_CAPTURE_STATUS(ref body) => body.ser(),
            &MavMessage::CAMERA_IMAGE_CAPTURED(ref body) => body.ser(),
            &MavMessage::FLIGHT_INFORMATION(ref body) => body.ser(),
            &MavMessage::MOUNT_ORIENTATION(ref body) => body.ser(),
            &MavMessage::LOGGING_DATA(ref body) => body.ser(),
            &MavMessage::LOGGING_DATA_ACKED(ref body) => body.ser(),
            &MavMessage::LOGGING_ACK(ref body) => body.ser(),
            &MavMessage::VIDEO_STREAM_INFORMATION(ref body) => body.ser(),
            &MavMessage::VIDEO_STREAM_STATUS(ref body) => body.ser(),
            &MavMessage::GIMBAL_MANAGER_INFORMATION(ref body) => body.ser(),
            &MavMessage::GIMBAL_MANAGER_STATUS(ref body) => body.ser(),
            &MavMessage::GIMBAL_MANAGER_SET_ATTITUDE(ref body) => body.ser(),
            &MavMessage::GIMBAL_DEVICE_INFORMATION(ref body) => body.ser(),
            &MavMessage::GIMBAL_DEVICE_SET_ATTITUDE(ref body) => body.ser(),
            &MavMessage::GIMBAL_DEVICE_ATTITUDE_STATUS(ref body) => body.ser(),
            &MavMessage::AUTOPILOT_STATE_FOR_GIMBAL_DEVICE(ref body) => body.ser(),
            &MavMessage::GIMBAL_MANAGER_SET_TILTPAN(ref body) => body.ser(),
            &MavMessage::WIFI_CONFIG_AP(ref body) => body.ser(),
            &MavMessage::PROTOCOL_VERSION(ref body) => body.ser(),
            &MavMessage::AIS_VESSEL(ref body) => body.ser(),
            &MavMessage::UAVCAN_NODE_STATUS(ref body) => body.ser(),
            &MavMessage::UAVCAN_NODE_INFO(ref body) => body.ser(),
            &MavMessage::PARAM_EXT_REQUEST_READ(ref body) => body.ser(),
            &MavMessage::PARAM_EXT_REQUEST_LIST(ref body) => body.ser(),
            &MavMessage::PARAM_EXT_VALUE(ref body) => body.ser(),
            &MavMessage::PARAM_EXT_SET(ref body) => body.ser(),
            &MavMessage::PARAM_EXT_ACK(ref body) => body.ser(),
            &MavMessage::OBSTACLE_DISTANCE(ref body) => body.ser(),
            &MavMessage::ODOMETRY(ref body) => body.ser(),
            &MavMessage::TRAJECTORY_REPRESENTATION_WAYPOINTS(ref body) => body.ser(),
            &MavMessage::TRAJECTORY_REPRESENTATION_BEZIER(ref body) => body.ser(),
            &MavMessage::CELLULAR_STATUS(ref body) => body.ser(),
            &MavMessage::ISBD_LINK_STATUS(ref body) => body.ser(),
            &MavMessage::CELLULAR_CONFIG(ref body) => body.ser(),
            &MavMessage::RAW_RPM(ref body) => body.ser(),
            &MavMessage::UTM_GLOBAL_POSITION(ref body) => body.ser(),
            &MavMessage::DEBUG_FLOAT_ARRAY(ref body) => body.ser(),
            &MavMessage::ORBIT_EXECUTION_STATUS(ref body) => body.ser(),
            &MavMessage::SMART_BATTERY_INFO(ref body) => body.ser(),
            &MavMessage::SMART_BATTERY_STATUS(ref body) => body.ser(),
            &MavMessage::GENERATOR_STATUS(ref body) => body.ser(),
            &MavMessage::ACTUATOR_OUTPUT_STATUS(ref body) => body.ser(),
            &MavMessage::TIME_ESTIMATE_TO_TARGET(ref body) => body.ser(),
            &MavMessage::TUNNEL(ref body) => body.ser(),
            &MavMessage::ONBOARD_COMPUTER_STATUS(ref body) => body.ser(),
            &MavMessage::COMPONENT_INFORMATION(ref body) => body.ser(),
            &MavMessage::PLAY_TUNE_V2(ref body) => body.ser(),
            &MavMessage::SUPPORTED_TUNES(ref body) => body.ser(),
            &MavMessage::WHEEL_DISTANCE(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_BASIC_ID(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_LOCATION(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_AUTHENTICATION(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_SELF_ID(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_SYSTEM(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_OPERATOR_ID(ref body) => body.ser(),
            &MavMessage::OPEN_DRONE_ID_MESSAGE_PACK(ref body) => body.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            0 => 50,
            1 => 124,
            2 => 137,
            4 => 237,
            5 => 217,
            6 => 104,
            7 => 119,
            8 => 117,
            11 => 89,
            20 => 214,
            21 => 159,
            22 => 220,
            23 => 168,
            24 => 24,
            25 => 23,
            26 => 170,
            27 => 144,
            28 => 67,
            29 => 115,
            30 => 39,
            31 => 246,
            32 => 185,
            33 => 104,
            34 => 237,
            35 => 244,
            36 => 222,
            37 => 212,
            38 => 9,
            39 => 254,
            40 => 230,
            41 => 28,
            42 => 28,
            43 => 132,
            44 => 221,
            45 => 232,
            46 => 11,
            47 => 153,
            48 => 41,
            49 => 39,
            50 => 78,
            51 => 196,
            52 => 132,
            54 => 15,
            55 => 3,
            61 => 167,
            62 => 183,
            63 => 119,
            64 => 191,
            65 => 118,
            66 => 148,
            67 => 21,
            69 => 243,
            70 => 124,
            73 => 38,
            74 => 20,
            75 => 158,
            76 => 152,
            77 => 143,
            80 => 14,
            81 => 106,
            82 => 49,
            83 => 22,
            84 => 143,
            85 => 140,
            86 => 5,
            87 => 150,
            89 => 231,
            90 => 183,
            91 => 63,
            92 => 54,
            93 => 47,
            100 => 175,
            101 => 102,
            102 => 158,
            103 => 208,
            104 => 56,
            105 => 93,
            106 => 138,
            107 => 108,
            108 => 32,
            109 => 185,
            110 => 84,
            111 => 34,
            112 => 174,
            113 => 124,
            114 => 237,
            115 => 4,
            116 => 76,
            117 => 128,
            118 => 56,
            119 => 116,
            120 => 134,
            121 => 237,
            122 => 203,
            123 => 250,
            124 => 87,
            125 => 203,
            126 => 220,
            127 => 25,
            128 => 226,
            129 => 46,
            130 => 29,
            131 => 223,
            132 => 85,
            133 => 6,
            134 => 229,
            135 => 203,
            136 => 1,
            137 => 195,
            138 => 109,
            139 => 168,
            140 => 181,
            141 => 47,
            142 => 72,
            143 => 131,
            144 => 127,
            146 => 103,
            147 => 154,
            148 => 178,
            149 => 200,
            162 => 189,
            230 => 163,
            231 => 105,
            232 => 151,
            233 => 35,
            234 => 150,
            235 => 179,
            241 => 90,
            242 => 104,
            243 => 85,
            244 => 95,
            245 => 130,
            246 => 184,
            247 => 81,
            248 => 8,
            249 => 204,
            250 => 49,
            251 => 170,
            252 => 44,
            253 => 83,
            254 => 46,
            256 => 71,
            257 => 131,
            258 => 187,
            259 => 92,
            260 => 146,
            261 => 179,
            262 => 12,
            263 => 133,
            264 => 49,
            265 => 26,
            266 => 193,
            267 => 35,
            268 => 14,
            269 => 109,
            270 => 59,
            280 => 166,
            281 => 0,
            282 => 123,
            283 => 247,
            284 => 99,
            285 => 137,
            286 => 210,
            287 => 74,
            299 => 19,
            300 => 217,
            301 => 243,
            310 => 28,
            311 => 95,
            320 => 243,
            321 => 88,
            322 => 243,
            323 => 78,
            324 => 132,
            330 => 23,
            331 => 91,
            332 => 236,
            333 => 231,
            334 => 72,
            335 => 225,
            336 => 178,
            339 => 199,
            340 => 99,
            350 => 232,
            360 => 11,
            370 => 98,
            371 => 161,
            373 => 192,
            375 => 251,
            380 => 232,
            385 => 147,
            390 => 156,
            395 => 163,
            400 => 110,
            401 => 183,
            9000 => 113,
            12900 => 114,
            12901 => 254,
            12902 => 49,
            12903 => 249,
            12904 => 203,
            12905 => 49,
            12915 => 62,
            _ => 0,
        }
    }
}
