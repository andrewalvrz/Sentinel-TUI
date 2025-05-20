// This file was automatically generated, do not edit
use crate::MavlinkVersion;
#[allow(unused_imports)]
use crate::{common::*, icarous::*, uavionix::*};
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
pub enum AccelcalVehiclePos {
    ACCELCAL_VEHICLE_POS_LEVEL = 1,
    ACCELCAL_VEHICLE_POS_LEFT = 2,
    ACCELCAL_VEHICLE_POS_RIGHT = 3,
    ACCELCAL_VEHICLE_POS_NOSEDOWN = 4,
    ACCELCAL_VEHICLE_POS_NOSEUP = 5,
    ACCELCAL_VEHICLE_POS_BACK = 6,
    ACCELCAL_VEHICLE_POS_SUCCESS = 16777215,
    ACCELCAL_VEHICLE_POS_FAILED = 16777216,
}
impl Default for AccelcalVehiclePos {
    fn default() -> Self {
        AccelcalVehiclePos::ACCELCAL_VEHICLE_POS_LEVEL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavCmd {
    MAV_CMD_DO_GRIPPER = 211,
    MAV_CMD_DO_AUTOTUNE_ENABLE = 212,
    MAV_CMD_DO_SET_RESUME_REPEAT_DIST = 215,
    MAV_CMD_NAV_ALTITUDE_WAIT = 83,
    MAV_CMD_POWER_OFF_INITIATED = 42000,
    MAV_CMD_SOLO_BTN_FLY_CLICK = 42001,
    MAV_CMD_SOLO_BTN_FLY_HOLD = 42002,
    MAV_CMD_SOLO_BTN_PAUSE_CLICK = 42003,
    MAV_CMD_FIXED_MAG_CAL = 42004,
    MAV_CMD_FIXED_MAG_CAL_FIELD = 42005,
    MAV_CMD_DO_START_MAG_CAL = 42424,
    MAV_CMD_DO_ACCEPT_MAG_CAL = 42425,
    MAV_CMD_DO_CANCEL_MAG_CAL = 42426,
    MAV_CMD_ACCELCAL_VEHICLE_POS = 42429,
    MAV_CMD_DO_SEND_BANNER = 42428,
    MAV_CMD_SET_FACTORY_TEST_MODE = 42427,
    MAV_CMD_GIMBAL_RESET = 42501,
    MAV_CMD_GIMBAL_AXIS_CALIBRATION_STATUS = 42502,
    MAV_CMD_GIMBAL_REQUEST_AXIS_CALIBRATION = 42503,
    MAV_CMD_GIMBAL_FULL_RESET = 42505,
    MAV_CMD_DO_WINCH = 42600,
    MAV_CMD_FLASH_BOOTLOADER = 42650,
    MAV_CMD_BATTERY_RESET = 42651,
    MAV_CMD_DEBUG_TRAP = 42700,
    MAV_CMD_SCRIPTING = 42701,
}
impl Default for MavCmd {
    fn default() -> Self {
        MavCmd::MAV_CMD_DO_GRIPPER
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum ScriptingCmd {
    SCRIPTING_CMD_REPL_START = 0,
    SCRIPTING_CMD_REPL_STOP = 1,
    SCRIPTING_CMD_STOP = 2,
    SCRIPTING_CMD_STOP_AND_RESTART = 3,
}
impl Default for ScriptingCmd {
    fn default() -> Self {
        ScriptingCmd::SCRIPTING_CMD_REPL_START
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum LimitsState {
    LIMITS_INIT = 0,
    LIMITS_DISABLED = 1,
    LIMITS_ENABLED = 2,
    LIMITS_TRIGGERED = 3,
    LIMITS_RECOVERING = 4,
    LIMITS_RECOVERED = 5,
}
impl Default for LimitsState {
    fn default() -> Self {
        LimitsState::LIMITS_INIT
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct LimitModule : u8 { const LIMIT_GPSLOCK = 1 ; const LIMIT_GEOFENCE = 2 ; const LIMIT_ALTITUDE = 4 ; } }
impl Default for LimitModule {
    fn default() -> Self {
        LimitModule::LIMIT_GPSLOCK
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct RallyFlags : u8 { const FAVORABLE_WIND = 1 ; const LAND_IMMEDIATELY = 2 ; } }
impl Default for RallyFlags {
    fn default() -> Self {
        RallyFlags::FAVORABLE_WIND
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GripperActions {
    GRIPPER_ACTION_RELEASE = 0,
    GRIPPER_ACTION_GRAB = 1,
}
impl Default for GripperActions {
    fn default() -> Self {
        GripperActions::GRIPPER_ACTION_RELEASE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum WinchActions {
    WINCH_RELAXED = 0,
    WINCH_RELATIVE_LENGTH_CONTROL = 1,
    WINCH_RATE_CONTROL = 2,
}
impl Default for WinchActions {
    fn default() -> Self {
        WinchActions::WINCH_RELAXED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CameraStatusTypes {
    CAMERA_STATUS_TYPE_HEARTBEAT = 0,
    CAMERA_STATUS_TYPE_TRIGGER = 1,
    CAMERA_STATUS_TYPE_DISCONNECT = 2,
    CAMERA_STATUS_TYPE_ERROR = 3,
    CAMERA_STATUS_TYPE_LOWBATT = 4,
    CAMERA_STATUS_TYPE_LOWSTORE = 5,
    CAMERA_STATUS_TYPE_LOWSTOREV = 6,
}
impl Default for CameraStatusTypes {
    fn default() -> Self {
        CameraStatusTypes::CAMERA_STATUS_TYPE_HEARTBEAT
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CameraFeedbackFlags {
    CAMERA_FEEDBACK_PHOTO = 0,
    CAMERA_FEEDBACK_VIDEO = 1,
    CAMERA_FEEDBACK_BADEXPOSURE = 2,
    CAMERA_FEEDBACK_CLOSEDLOOP = 3,
    CAMERA_FEEDBACK_OPENLOOP = 4,
}
impl Default for CameraFeedbackFlags {
    fn default() -> Self {
        CameraFeedbackFlags::CAMERA_FEEDBACK_PHOTO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavModeGimbal {
    MAV_MODE_GIMBAL_UNINITIALIZED = 0,
    MAV_MODE_GIMBAL_CALIBRATING_PITCH = 1,
    MAV_MODE_GIMBAL_CALIBRATING_ROLL = 2,
    MAV_MODE_GIMBAL_CALIBRATING_YAW = 3,
    MAV_MODE_GIMBAL_INITIALIZED = 4,
    MAV_MODE_GIMBAL_ACTIVE = 5,
    MAV_MODE_GIMBAL_RATE_CMD_TIMEOUT = 6,
}
impl Default for MavModeGimbal {
    fn default() -> Self {
        MavModeGimbal::MAV_MODE_GIMBAL_UNINITIALIZED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalAxis {
    GIMBAL_AXIS_YAW = 0,
    GIMBAL_AXIS_PITCH = 1,
    GIMBAL_AXIS_ROLL = 2,
}
impl Default for GimbalAxis {
    fn default() -> Self {
        GimbalAxis::GIMBAL_AXIS_YAW
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalAxisCalibrationStatus {
    GIMBAL_AXIS_CALIBRATION_STATUS_IN_PROGRESS = 0,
    GIMBAL_AXIS_CALIBRATION_STATUS_SUCCEEDED = 1,
    GIMBAL_AXIS_CALIBRATION_STATUS_FAILED = 2,
}
impl Default for GimbalAxisCalibrationStatus {
    fn default() -> Self {
        GimbalAxisCalibrationStatus::GIMBAL_AXIS_CALIBRATION_STATUS_IN_PROGRESS
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GimbalAxisCalibrationRequired {
    GIMBAL_AXIS_CALIBRATION_REQUIRED_UNKNOWN = 0,
    GIMBAL_AXIS_CALIBRATION_REQUIRED_TRUE = 1,
    GIMBAL_AXIS_CALIBRATION_REQUIRED_FALSE = 2,
}
impl Default for GimbalAxisCalibrationRequired {
    fn default() -> Self {
        GimbalAxisCalibrationRequired::GIMBAL_AXIS_CALIBRATION_REQUIRED_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproHeartbeatStatus {
    GOPRO_HEARTBEAT_STATUS_DISCONNECTED = 0,
    GOPRO_HEARTBEAT_STATUS_INCOMPATIBLE = 1,
    GOPRO_HEARTBEAT_STATUS_CONNECTED = 2,
    GOPRO_HEARTBEAT_STATUS_ERROR = 3,
}
impl Default for GoproHeartbeatStatus {
    fn default() -> Self {
        GoproHeartbeatStatus::GOPRO_HEARTBEAT_STATUS_DISCONNECTED
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct GoproHeartbeatFlags : u8 { const GOPRO_FLAG_RECORDING = 1 ; } }
impl Default for GoproHeartbeatFlags {
    fn default() -> Self {
        GoproHeartbeatFlags::GOPRO_FLAG_RECORDING
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproRequestStatus {
    GOPRO_REQUEST_SUCCESS = 0,
    GOPRO_REQUEST_FAILED = 1,
}
impl Default for GoproRequestStatus {
    fn default() -> Self {
        GoproRequestStatus::GOPRO_REQUEST_SUCCESS
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproCommand {
    GOPRO_COMMAND_POWER = 0,
    GOPRO_COMMAND_CAPTURE_MODE = 1,
    GOPRO_COMMAND_SHUTTER = 2,
    GOPRO_COMMAND_BATTERY = 3,
    GOPRO_COMMAND_MODEL = 4,
    GOPRO_COMMAND_VIDEO_SETTINGS = 5,
    GOPRO_COMMAND_LOW_LIGHT = 6,
    GOPRO_COMMAND_PHOTO_RESOLUTION = 7,
    GOPRO_COMMAND_PHOTO_BURST_RATE = 8,
    GOPRO_COMMAND_PROTUNE = 9,
    GOPRO_COMMAND_PROTUNE_WHITE_BALANCE = 10,
    GOPRO_COMMAND_PROTUNE_COLOUR = 11,
    GOPRO_COMMAND_PROTUNE_GAIN = 12,
    GOPRO_COMMAND_PROTUNE_SHARPNESS = 13,
    GOPRO_COMMAND_PROTUNE_EXPOSURE = 14,
    GOPRO_COMMAND_TIME = 15,
    GOPRO_COMMAND_CHARGING = 16,
}
impl Default for GoproCommand {
    fn default() -> Self {
        GoproCommand::GOPRO_COMMAND_POWER
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproCaptureMode {
    GOPRO_CAPTURE_MODE_VIDEO = 0,
    GOPRO_CAPTURE_MODE_PHOTO = 1,
    GOPRO_CAPTURE_MODE_BURST = 2,
    GOPRO_CAPTURE_MODE_TIME_LAPSE = 3,
    GOPRO_CAPTURE_MODE_MULTI_SHOT = 4,
    GOPRO_CAPTURE_MODE_PLAYBACK = 5,
    GOPRO_CAPTURE_MODE_SETUP = 6,
    GOPRO_CAPTURE_MODE_UNKNOWN = 255,
}
impl Default for GoproCaptureMode {
    fn default() -> Self {
        GoproCaptureMode::GOPRO_CAPTURE_MODE_VIDEO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproResolution {
    GOPRO_RESOLUTION_480p = 0,
    GOPRO_RESOLUTION_720p = 1,
    GOPRO_RESOLUTION_960p = 2,
    GOPRO_RESOLUTION_1080p = 3,
    GOPRO_RESOLUTION_1440p = 4,
    GOPRO_RESOLUTION_2_7k_17_9 = 5,
    GOPRO_RESOLUTION_2_7k_16_9 = 6,
    GOPRO_RESOLUTION_2_7k_4_3 = 7,
    GOPRO_RESOLUTION_4k_16_9 = 8,
    GOPRO_RESOLUTION_4k_17_9 = 9,
    GOPRO_RESOLUTION_720p_SUPERVIEW = 10,
    GOPRO_RESOLUTION_1080p_SUPERVIEW = 11,
    GOPRO_RESOLUTION_2_7k_SUPERVIEW = 12,
    GOPRO_RESOLUTION_4k_SUPERVIEW = 13,
}
impl Default for GoproResolution {
    fn default() -> Self {
        GoproResolution::GOPRO_RESOLUTION_480p
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproFrameRate {
    GOPRO_FRAME_RATE_12 = 0,
    GOPRO_FRAME_RATE_15 = 1,
    GOPRO_FRAME_RATE_24 = 2,
    GOPRO_FRAME_RATE_25 = 3,
    GOPRO_FRAME_RATE_30 = 4,
    GOPRO_FRAME_RATE_48 = 5,
    GOPRO_FRAME_RATE_50 = 6,
    GOPRO_FRAME_RATE_60 = 7,
    GOPRO_FRAME_RATE_80 = 8,
    GOPRO_FRAME_RATE_90 = 9,
    GOPRO_FRAME_RATE_100 = 10,
    GOPRO_FRAME_RATE_120 = 11,
    GOPRO_FRAME_RATE_240 = 12,
    GOPRO_FRAME_RATE_12_5 = 13,
}
impl Default for GoproFrameRate {
    fn default() -> Self {
        GoproFrameRate::GOPRO_FRAME_RATE_12
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproFieldOfView {
    GOPRO_FIELD_OF_VIEW_WIDE = 0,
    GOPRO_FIELD_OF_VIEW_MEDIUM = 1,
    GOPRO_FIELD_OF_VIEW_NARROW = 2,
}
impl Default for GoproFieldOfView {
    fn default() -> Self {
        GoproFieldOfView::GOPRO_FIELD_OF_VIEW_WIDE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproVideoSettingsFlags {
    GOPRO_VIDEO_SETTINGS_TV_MODE = 1,
}
impl Default for GoproVideoSettingsFlags {
    fn default() -> Self {
        GoproVideoSettingsFlags::GOPRO_VIDEO_SETTINGS_TV_MODE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproPhotoResolution {
    GOPRO_PHOTO_RESOLUTION_5MP_MEDIUM = 0,
    GOPRO_PHOTO_RESOLUTION_7MP_MEDIUM = 1,
    GOPRO_PHOTO_RESOLUTION_7MP_WIDE = 2,
    GOPRO_PHOTO_RESOLUTION_10MP_WIDE = 3,
    GOPRO_PHOTO_RESOLUTION_12MP_WIDE = 4,
}
impl Default for GoproPhotoResolution {
    fn default() -> Self {
        GoproPhotoResolution::GOPRO_PHOTO_RESOLUTION_5MP_MEDIUM
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproProtuneWhiteBalance {
    GOPRO_PROTUNE_WHITE_BALANCE_AUTO = 0,
    GOPRO_PROTUNE_WHITE_BALANCE_3000K = 1,
    GOPRO_PROTUNE_WHITE_BALANCE_5500K = 2,
    GOPRO_PROTUNE_WHITE_BALANCE_6500K = 3,
    GOPRO_PROTUNE_WHITE_BALANCE_RAW = 4,
}
impl Default for GoproProtuneWhiteBalance {
    fn default() -> Self {
        GoproProtuneWhiteBalance::GOPRO_PROTUNE_WHITE_BALANCE_AUTO
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproProtuneColour {
    GOPRO_PROTUNE_COLOUR_STANDARD = 0,
    GOPRO_PROTUNE_COLOUR_NEUTRAL = 1,
}
impl Default for GoproProtuneColour {
    fn default() -> Self {
        GoproProtuneColour::GOPRO_PROTUNE_COLOUR_STANDARD
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproProtuneGain {
    GOPRO_PROTUNE_GAIN_400 = 0,
    GOPRO_PROTUNE_GAIN_800 = 1,
    GOPRO_PROTUNE_GAIN_1600 = 2,
    GOPRO_PROTUNE_GAIN_3200 = 3,
    GOPRO_PROTUNE_GAIN_6400 = 4,
}
impl Default for GoproProtuneGain {
    fn default() -> Self {
        GoproProtuneGain::GOPRO_PROTUNE_GAIN_400
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproProtuneSharpness {
    GOPRO_PROTUNE_SHARPNESS_LOW = 0,
    GOPRO_PROTUNE_SHARPNESS_MEDIUM = 1,
    GOPRO_PROTUNE_SHARPNESS_HIGH = 2,
}
impl Default for GoproProtuneSharpness {
    fn default() -> Self {
        GoproProtuneSharpness::GOPRO_PROTUNE_SHARPNESS_LOW
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproProtuneExposure {
    GOPRO_PROTUNE_EXPOSURE_NEG_5_0 = 0,
    GOPRO_PROTUNE_EXPOSURE_NEG_4_5 = 1,
    GOPRO_PROTUNE_EXPOSURE_NEG_4_0 = 2,
    GOPRO_PROTUNE_EXPOSURE_NEG_3_5 = 3,
    GOPRO_PROTUNE_EXPOSURE_NEG_3_0 = 4,
    GOPRO_PROTUNE_EXPOSURE_NEG_2_5 = 5,
    GOPRO_PROTUNE_EXPOSURE_NEG_2_0 = 6,
    GOPRO_PROTUNE_EXPOSURE_NEG_1_5 = 7,
    GOPRO_PROTUNE_EXPOSURE_NEG_1_0 = 8,
    GOPRO_PROTUNE_EXPOSURE_NEG_0_5 = 9,
    GOPRO_PROTUNE_EXPOSURE_ZERO = 10,
    GOPRO_PROTUNE_EXPOSURE_POS_0_5 = 11,
    GOPRO_PROTUNE_EXPOSURE_POS_1_0 = 12,
    GOPRO_PROTUNE_EXPOSURE_POS_1_5 = 13,
    GOPRO_PROTUNE_EXPOSURE_POS_2_0 = 14,
    GOPRO_PROTUNE_EXPOSURE_POS_2_5 = 15,
    GOPRO_PROTUNE_EXPOSURE_POS_3_0 = 16,
    GOPRO_PROTUNE_EXPOSURE_POS_3_5 = 17,
    GOPRO_PROTUNE_EXPOSURE_POS_4_0 = 18,
    GOPRO_PROTUNE_EXPOSURE_POS_4_5 = 19,
    GOPRO_PROTUNE_EXPOSURE_POS_5_0 = 20,
}
impl Default for GoproProtuneExposure {
    fn default() -> Self {
        GoproProtuneExposure::GOPRO_PROTUNE_EXPOSURE_NEG_5_0
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproCharging {
    GOPRO_CHARGING_DISABLED = 0,
    GOPRO_CHARGING_ENABLED = 1,
}
impl Default for GoproCharging {
    fn default() -> Self {
        GoproCharging::GOPRO_CHARGING_DISABLED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproModel {
    GOPRO_MODEL_UNKNOWN = 0,
    GOPRO_MODEL_HERO_3_PLUS_SILVER = 1,
    GOPRO_MODEL_HERO_3_PLUS_BLACK = 2,
    GOPRO_MODEL_HERO_4_SILVER = 3,
    GOPRO_MODEL_HERO_4_BLACK = 4,
}
impl Default for GoproModel {
    fn default() -> Self {
        GoproModel::GOPRO_MODEL_UNKNOWN
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GoproBurstRate {
    GOPRO_BURST_RATE_3_IN_1_SECOND = 0,
    GOPRO_BURST_RATE_5_IN_1_SECOND = 1,
    GOPRO_BURST_RATE_10_IN_1_SECOND = 2,
    GOPRO_BURST_RATE_10_IN_2_SECOND = 3,
    GOPRO_BURST_RATE_10_IN_3_SECOND = 4,
    GOPRO_BURST_RATE_30_IN_1_SECOND = 5,
    GOPRO_BURST_RATE_30_IN_2_SECOND = 6,
    GOPRO_BURST_RATE_30_IN_3_SECOND = 7,
    GOPRO_BURST_RATE_30_IN_6_SECOND = 8,
}
impl Default for GoproBurstRate {
    fn default() -> Self {
        GoproBurstRate::GOPRO_BURST_RATE_3_IN_1_SECOND
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum LedControlPattern {
    LED_CONTROL_PATTERN_OFF = 0,
    LED_CONTROL_PATTERN_FIRMWAREUPDATE = 1,
    LED_CONTROL_PATTERN_CUSTOM = 255,
}
impl Default for LedControlPattern {
    fn default() -> Self {
        LedControlPattern::LED_CONTROL_PATTERN_OFF
    }
}
bitflags! { # [ cfg_attr ( feature = "serde" , derive ( Serialize , Deserialize ) ) ] pub struct EkfStatusFlags : u16 { const EKF_ATTITUDE = 1 ; const EKF_VELOCITY_HORIZ = 2 ; const EKF_VELOCITY_VERT = 4 ; const EKF_POS_HORIZ_REL = 8 ; const EKF_POS_HORIZ_ABS = 16 ; const EKF_POS_VERT_ABS = 32 ; const EKF_POS_VERT_AGL = 64 ; const EKF_CONST_POS_MODE = 128 ; const EKF_PRED_POS_HORIZ_REL = 256 ; const EKF_PRED_POS_HORIZ_ABS = 512 ; const EKF_UNINITIALIZED = 1024 ; } }
impl Default for EkfStatusFlags {
    fn default() -> Self {
        EkfStatusFlags::EKF_ATTITUDE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PidTuningAxis {
    PID_TUNING_ROLL = 1,
    PID_TUNING_PITCH = 2,
    PID_TUNING_YAW = 3,
    PID_TUNING_ACCZ = 4,
    PID_TUNING_STEER = 5,
    PID_TUNING_LANDING = 6,
}
impl Default for PidTuningAxis {
    fn default() -> Self {
        PidTuningAxis::PID_TUNING_ROLL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MagCalStatus {
    MAG_CAL_NOT_STARTED = 0,
    MAG_CAL_WAITING_TO_START = 1,
    MAG_CAL_RUNNING_STEP_ONE = 2,
    MAG_CAL_RUNNING_STEP_TWO = 3,
    MAG_CAL_SUCCESS = 4,
    MAG_CAL_FAILED = 5,
    MAG_CAL_BAD_ORIENTATION = 6,
    MAG_CAL_BAD_RADIUS = 7,
}
impl Default for MagCalStatus {
    fn default() -> Self {
        MagCalStatus::MAG_CAL_NOT_STARTED
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavRemoteLogDataBlockCommands {
    MAV_REMOTE_LOG_DATA_BLOCK_STOP = 2147483645,
    MAV_REMOTE_LOG_DATA_BLOCK_START = 2147483646,
}
impl Default for MavRemoteLogDataBlockCommands {
    fn default() -> Self {
        MavRemoteLogDataBlockCommands::MAV_REMOTE_LOG_DATA_BLOCK_STOP
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavRemoteLogDataBlockStatuses {
    MAV_REMOTE_LOG_DATA_BLOCK_NACK = 0,
    MAV_REMOTE_LOG_DATA_BLOCK_ACK = 1,
}
impl Default for MavRemoteLogDataBlockStatuses {
    fn default() -> Self {
        MavRemoteLogDataBlockStatuses::MAV_REMOTE_LOG_DATA_BLOCK_NACK
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum DeviceOpBustype {
    DEVICE_OP_BUSTYPE_I2C = 0,
    DEVICE_OP_BUSTYPE_SPI = 1,
}
impl Default for DeviceOpBustype {
    fn default() -> Self {
        DeviceOpBustype::DEVICE_OP_BUSTYPE_I2C
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum DeepstallStage {
    DEEPSTALL_STAGE_FLY_TO_LANDING = 0,
    DEEPSTALL_STAGE_ESTIMATE_WIND = 1,
    DEEPSTALL_STAGE_WAIT_FOR_BREAKOUT = 2,
    DEEPSTALL_STAGE_FLY_TO_ARC = 3,
    DEEPSTALL_STAGE_ARC = 4,
    DEEPSTALL_STAGE_APPROACH = 5,
    DEEPSTALL_STAGE_LAND = 6,
}
impl Default for DeepstallStage {
    fn default() -> Self {
        DeepstallStage::DEEPSTALL_STAGE_FLY_TO_LANDING
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PlaneMode {
    PLANE_MODE_MANUAL = 0,
    PLANE_MODE_CIRCLE = 1,
    PLANE_MODE_STABILIZE = 2,
    PLANE_MODE_TRAINING = 3,
    PLANE_MODE_ACRO = 4,
    PLANE_MODE_FLY_BY_WIRE_A = 5,
    PLANE_MODE_FLY_BY_WIRE_B = 6,
    PLANE_MODE_CRUISE = 7,
    PLANE_MODE_AUTOTUNE = 8,
    PLANE_MODE_AUTO = 10,
    PLANE_MODE_RTL = 11,
    PLANE_MODE_LOITER = 12,
    PLANE_MODE_TAKEOFF = 13,
    PLANE_MODE_AVOID_ADSB = 14,
    PLANE_MODE_GUIDED = 15,
    PLANE_MODE_INITIALIZING = 16,
    PLANE_MODE_QSTABILIZE = 17,
    PLANE_MODE_QHOVER = 18,
    PLANE_MODE_QLOITER = 19,
    PLANE_MODE_QLAND = 20,
    PLANE_MODE_QRTL = 21,
    PLANE_MODE_QAUTOTUNE = 22,
}
impl Default for PlaneMode {
    fn default() -> Self {
        PlaneMode::PLANE_MODE_MANUAL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum CopterMode {
    COPTER_MODE_STABILIZE = 0,
    COPTER_MODE_ACRO = 1,
    COPTER_MODE_ALT_HOLD = 2,
    COPTER_MODE_AUTO = 3,
    COPTER_MODE_GUIDED = 4,
    COPTER_MODE_LOITER = 5,
    COPTER_MODE_RTL = 6,
    COPTER_MODE_CIRCLE = 7,
    COPTER_MODE_LAND = 9,
    COPTER_MODE_DRIFT = 11,
    COPTER_MODE_SPORT = 13,
    COPTER_MODE_FLIP = 14,
    COPTER_MODE_AUTOTUNE = 15,
    COPTER_MODE_POSHOLD = 16,
    COPTER_MODE_BRAKE = 17,
    COPTER_MODE_THROW = 18,
    COPTER_MODE_AVOID_ADSB = 19,
    COPTER_MODE_GUIDED_NOGPS = 20,
    COPTER_MODE_SMART_RTL = 21,
}
impl Default for CopterMode {
    fn default() -> Self {
        CopterMode::COPTER_MODE_STABILIZE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum SubMode {
    SUB_MODE_STABILIZE = 0,
    SUB_MODE_ACRO = 1,
    SUB_MODE_ALT_HOLD = 2,
    SUB_MODE_AUTO = 3,
    SUB_MODE_GUIDED = 4,
    SUB_MODE_CIRCLE = 7,
    SUB_MODE_SURFACE = 9,
    SUB_MODE_POSHOLD = 16,
    SUB_MODE_MANUAL = 19,
}
impl Default for SubMode {
    fn default() -> Self {
        SubMode::SUB_MODE_STABILIZE
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum RoverMode {
    ROVER_MODE_MANUAL = 0,
    ROVER_MODE_ACRO = 1,
    ROVER_MODE_STEERING = 3,
    ROVER_MODE_HOLD = 4,
    ROVER_MODE_LOITER = 5,
    ROVER_MODE_AUTO = 10,
    ROVER_MODE_RTL = 11,
    ROVER_MODE_SMART_RTL = 12,
    ROVER_MODE_GUIDED = 15,
    ROVER_MODE_INITIALIZING = 16,
}
impl Default for RoverMode {
    fn default() -> Self {
        RoverMode::ROVER_MODE_MANUAL
    }
}
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum TrackerMode {
    TRACKER_MODE_MANUAL = 0,
    TRACKER_MODE_STOP = 1,
    TRACKER_MODE_SCAN = 2,
    TRACKER_MODE_SERVO_TEST = 3,
    TRACKER_MODE_AUTO = 10,
    TRACKER_MODE_INITIALIZING = 16,
}
impl Default for TrackerMode {
    fn default() -> Self {
        TrackerMode::TRACKER_MODE_MANUAL
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SENSOR_OFFSETS_DATA {
    pub mag_declination: f32,
    pub raw_press: i32,
    pub raw_temp: i32,
    pub gyro_cal_x: f32,
    pub gyro_cal_y: f32,
    pub gyro_cal_z: f32,
    pub accel_cal_x: f32,
    pub accel_cal_y: f32,
    pub accel_cal_z: f32,
    pub mag_ofs_x: i16,
    pub mag_ofs_y: i16,
    pub mag_ofs_z: i16,
}
impl SENSOR_OFFSETS_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SENSOR_OFFSETS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SENSOR_OFFSETS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mag_declination = buf.get_f32_le();
        _struct.raw_press = buf.get_i32_le();
        _struct.raw_temp = buf.get_i32_le();
        _struct.gyro_cal_x = buf.get_f32_le();
        _struct.gyro_cal_y = buf.get_f32_le();
        _struct.gyro_cal_z = buf.get_f32_le();
        _struct.accel_cal_x = buf.get_f32_le();
        _struct.accel_cal_y = buf.get_f32_le();
        _struct.accel_cal_z = buf.get_f32_le();
        _struct.mag_ofs_x = buf.get_i16_le();
        _struct.mag_ofs_y = buf.get_i16_le();
        _struct.mag_ofs_z = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.mag_declination);
        _tmp.put_i32_le(self.raw_press);
        _tmp.put_i32_le(self.raw_temp);
        _tmp.put_f32_le(self.gyro_cal_x);
        _tmp.put_f32_le(self.gyro_cal_y);
        _tmp.put_f32_le(self.gyro_cal_z);
        _tmp.put_f32_le(self.accel_cal_x);
        _tmp.put_f32_le(self.accel_cal_y);
        _tmp.put_f32_le(self.accel_cal_z);
        _tmp.put_i16_le(self.mag_ofs_x);
        _tmp.put_i16_le(self.mag_ofs_y);
        _tmp.put_i16_le(self.mag_ofs_z);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SET_MAG_OFFSETS_DATA {
    pub mag_ofs_x: i16,
    pub mag_ofs_y: i16,
    pub mag_ofs_z: i16,
    pub target_system: u8,
    pub target_component: u8,
}
impl SET_MAG_OFFSETS_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SET_MAG_OFFSETS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SET_MAG_OFFSETS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mag_ofs_x = buf.get_i16_le();
        _struct.mag_ofs_y = buf.get_i16_le();
        _struct.mag_ofs_z = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.mag_ofs_x);
        _tmp.put_i16_le(self.mag_ofs_y);
        _tmp.put_i16_le(self.mag_ofs_z);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MEMINFO_DATA {
    pub brkval: u16,
    pub freemem: u16,
}
impl MEMINFO_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MEMINFO_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MEMINFO_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.brkval = buf.get_u16_le();
        _struct.freemem = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.brkval);
        _tmp.put_u16_le(self.freemem);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AP_ADC_DATA {
    pub adc1: u16,
    pub adc2: u16,
    pub adc3: u16,
    pub adc4: u16,
    pub adc5: u16,
    pub adc6: u16,
}
impl AP_ADC_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AP_ADC_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AP_ADC_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.adc1 = buf.get_u16_le();
        _struct.adc2 = buf.get_u16_le();
        _struct.adc3 = buf.get_u16_le();
        _struct.adc4 = buf.get_u16_le();
        _struct.adc5 = buf.get_u16_le();
        _struct.adc6 = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.adc1);
        _tmp.put_u16_le(self.adc2);
        _tmp.put_u16_le(self.adc3);
        _tmp.put_u16_le(self.adc4);
        _tmp.put_u16_le(self.adc5);
        _tmp.put_u16_le(self.adc6);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DIGICAM_CONFIGURE_DATA {
    pub extra_value: f32,
    pub shutter_speed: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub mode: u8,
    pub aperture: u8,
    pub iso: u8,
    pub exposure_type: u8,
    pub command_id: u8,
    pub engine_cut_off: u8,
    pub extra_param: u8,
}
impl DIGICAM_CONFIGURE_DATA {
    pub const ENCODED_LEN: usize = 15usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DIGICAM_CONFIGURE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DIGICAM_CONFIGURE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.extra_value = buf.get_f32_le();
        _struct.shutter_speed = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.mode = buf.get_u8();
        _struct.aperture = buf.get_u8();
        _struct.iso = buf.get_u8();
        _struct.exposure_type = buf.get_u8();
        _struct.command_id = buf.get_u8();
        _struct.engine_cut_off = buf.get_u8();
        _struct.extra_param = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.extra_value);
        _tmp.put_u16_le(self.shutter_speed);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.mode);
        _tmp.put_u8(self.aperture);
        _tmp.put_u8(self.iso);
        _tmp.put_u8(self.exposure_type);
        _tmp.put_u8(self.command_id);
        _tmp.put_u8(self.engine_cut_off);
        _tmp.put_u8(self.extra_param);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DIGICAM_CONTROL_DATA {
    pub extra_value: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub session: u8,
    pub zoom_pos: u8,
    pub zoom_step: i8,
    pub focus_lock: u8,
    pub shot: u8,
    pub command_id: u8,
    pub extra_param: u8,
}
impl DIGICAM_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 13usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DIGICAM_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DIGICAM_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.extra_value = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.session = buf.get_u8();
        _struct.zoom_pos = buf.get_u8();
        _struct.zoom_step = buf.get_i8();
        _struct.focus_lock = buf.get_u8();
        _struct.shot = buf.get_u8();
        _struct.command_id = buf.get_u8();
        _struct.extra_param = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.extra_value);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.session);
        _tmp.put_u8(self.zoom_pos);
        _tmp.put_i8(self.zoom_step);
        _tmp.put_u8(self.focus_lock);
        _tmp.put_u8(self.shot);
        _tmp.put_u8(self.command_id);
        _tmp.put_u8(self.extra_param);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MOUNT_CONFIGURE_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub mount_mode: MavMountMode,
    pub stab_roll: u8,
    pub stab_pitch: u8,
    pub stab_yaw: u8,
}
impl MOUNT_CONFIGURE_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MOUNT_CONFIGURE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MOUNT_CONFIGURE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.mount_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavMountMode".to_string(),
            value: tmp as u32,
        })?;
        _struct.stab_roll = buf.get_u8();
        _struct.stab_pitch = buf.get_u8();
        _struct.stab_yaw = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.mount_mode as u8);
        _tmp.put_u8(self.stab_roll);
        _tmp.put_u8(self.stab_pitch);
        _tmp.put_u8(self.stab_yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MOUNT_CONTROL_DATA {
    pub input_a: i32,
    pub input_b: i32,
    pub input_c: i32,
    pub target_system: u8,
    pub target_component: u8,
    pub save_position: u8,
}
impl MOUNT_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 15usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MOUNT_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MOUNT_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.input_a = buf.get_i32_le();
        _struct.input_b = buf.get_i32_le();
        _struct.input_c = buf.get_i32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.save_position = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.input_a);
        _tmp.put_i32_le(self.input_b);
        _tmp.put_i32_le(self.input_c);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.save_position);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MOUNT_STATUS_DATA {
    pub pointing_a: i32,
    pub pointing_b: i32,
    pub pointing_c: i32,
    pub target_system: u8,
    pub target_component: u8,
}
impl MOUNT_STATUS_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MOUNT_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MOUNT_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.pointing_a = buf.get_i32_le();
        _struct.pointing_b = buf.get_i32_le();
        _struct.pointing_c = buf.get_i32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.pointing_a);
        _tmp.put_i32_le(self.pointing_b);
        _tmp.put_i32_le(self.pointing_c);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FENCE_POINT_DATA {
    pub lat: f32,
    pub lng: f32,
    pub target_system: u8,
    pub target_component: u8,
    pub idx: u8,
    pub count: u8,
}
impl FENCE_POINT_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FENCE_POINT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FENCE_POINT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.lat = buf.get_f32_le();
        _struct.lng = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.idx = buf.get_u8();
        _struct.count = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.lat);
        _tmp.put_f32_le(self.lng);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.idx);
        _tmp.put_u8(self.count);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FENCE_FETCH_POINT_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub idx: u8,
}
impl FENCE_FETCH_POINT_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < FENCE_FETCH_POINT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; FENCE_FETCH_POINT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.idx = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.idx);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AHRS_DATA {
    pub omegaIx: f32,
    pub omegaIy: f32,
    pub omegaIz: f32,
    pub accel_weight: f32,
    pub renorm_val: f32,
    pub error_rp: f32,
    pub error_yaw: f32,
}
impl AHRS_DATA {
    pub const ENCODED_LEN: usize = 28usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AHRS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AHRS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.omegaIx = buf.get_f32_le();
        _struct.omegaIy = buf.get_f32_le();
        _struct.omegaIz = buf.get_f32_le();
        _struct.accel_weight = buf.get_f32_le();
        _struct.renorm_val = buf.get_f32_le();
        _struct.error_rp = buf.get_f32_le();
        _struct.error_yaw = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.omegaIx);
        _tmp.put_f32_le(self.omegaIy);
        _tmp.put_f32_le(self.omegaIz);
        _tmp.put_f32_le(self.accel_weight);
        _tmp.put_f32_le(self.renorm_val);
        _tmp.put_f32_le(self.error_rp);
        _tmp.put_f32_le(self.error_yaw);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SIMSTATE_DATA {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub xacc: f32,
    pub yacc: f32,
    pub zacc: f32,
    pub xgyro: f32,
    pub ygyro: f32,
    pub zgyro: f32,
    pub lat: i32,
    pub lng: i32,
}
impl SIMSTATE_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < SIMSTATE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; SIMSTATE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.xacc = buf.get_f32_le();
        _struct.yacc = buf.get_f32_le();
        _struct.zacc = buf.get_f32_le();
        _struct.xgyro = buf.get_f32_le();
        _struct.ygyro = buf.get_f32_le();
        _struct.zgyro = buf.get_f32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lng = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.xacc);
        _tmp.put_f32_le(self.yacc);
        _tmp.put_f32_le(self.zacc);
        _tmp.put_f32_le(self.xgyro);
        _tmp.put_f32_le(self.ygyro);
        _tmp.put_f32_le(self.zgyro);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lng);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HWSTATUS_DATA {
    pub Vcc: u16,
    pub I2Cerr: u8,
}
impl HWSTATUS_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < HWSTATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; HWSTATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.Vcc = buf.get_u16_le();
        _struct.I2Cerr = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.Vcc);
        _tmp.put_u8(self.I2Cerr);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RADIO_DATA {
    pub rxerrors: u16,
    pub fixed: u16,
    pub rssi: u8,
    pub remrssi: u8,
    pub txbuf: u8,
    pub noise: u8,
    pub remnoise: u8,
}
impl RADIO_DATA {
    pub const ENCODED_LEN: usize = 9usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RADIO_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RADIO_DATA::ENCODED_LEN];
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
pub struct LIMITS_STATUS_DATA {
    pub last_trigger: u32,
    pub last_action: u32,
    pub last_recovery: u32,
    pub last_clear: u32,
    pub breach_count: u16,
    pub limits_state: LimitsState,
    pub mods_enabled: LimitModule,
    pub mods_required: LimitModule,
    pub mods_triggered: LimitModule,
}
impl LIMITS_STATUS_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LIMITS_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LIMITS_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.last_trigger = buf.get_u32_le();
        _struct.last_action = buf.get_u32_le();
        _struct.last_recovery = buf.get_u32_le();
        _struct.last_clear = buf.get_u32_le();
        _struct.breach_count = buf.get_u16_le();
        let tmp = buf.get_u8();
        _struct.limits_state = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "LimitsState".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.mods_enabled = LimitModule::from_bits(tmp & LimitModule::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "LimitModule".to_string(),
                value: tmp as u32,
            },
        )?;
        let tmp = buf.get_u8();
        _struct.mods_required = LimitModule::from_bits(tmp & LimitModule::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "LimitModule".to_string(),
                value: tmp as u32,
            },
        )?;
        let tmp = buf.get_u8();
        _struct.mods_triggered = LimitModule::from_bits(tmp & LimitModule::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "LimitModule".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.last_trigger);
        _tmp.put_u32_le(self.last_action);
        _tmp.put_u32_le(self.last_recovery);
        _tmp.put_u32_le(self.last_clear);
        _tmp.put_u16_le(self.breach_count);
        _tmp.put_u8(self.limits_state as u8);
        _tmp.put_u8(self.mods_enabled.bits());
        _tmp.put_u8(self.mods_required.bits());
        _tmp.put_u8(self.mods_triggered.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WIND_DATA {
    pub direction: f32,
    pub speed: f32,
    pub speed_z: f32,
}
impl WIND_DATA {
    pub const ENCODED_LEN: usize = 12usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < WIND_DATA::ENCODED_LEN {
            let mut payload_buf = [0; WIND_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.direction = buf.get_f32_le();
        _struct.speed = buf.get_f32_le();
        _struct.speed_z = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.direction);
        _tmp.put_f32_le(self.speed);
        _tmp.put_f32_le(self.speed_z);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA16_DATA {
    pub mavtype: u8,
    pub len: u8,
    pub data: [u8; 16],
}
impl DATA16_DATA {
    pub const ENCODED_LEN: usize = 18usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA16_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA16_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mavtype = buf.get_u8();
        _struct.len = buf.get_u8();
        for idx in 0..16usize {
            let val = buf.get_u8();
            _struct.data[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mavtype);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA32_DATA {
    pub mavtype: u8,
    pub len: u8,
    pub data: [u8; 32],
}
impl DATA32_DATA {
    pub const ENCODED_LEN: usize = 34usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA32_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA32_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mavtype = buf.get_u8();
        _struct.len = buf.get_u8();
        for idx in 0..32usize {
            let val = buf.get_u8();
            _struct.data[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mavtype);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA64_DATA {
    pub mavtype: u8,
    pub len: u8,
    pub data: Vec<u8>, /* 64 elements */
}
impl DATA64_DATA {
    pub const ENCODED_LEN: usize = 66usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA64_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA64_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mavtype = buf.get_u8();
        _struct.len = buf.get_u8();
        for _ in 0..64usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mavtype);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DATA96_DATA {
    pub mavtype: u8,
    pub len: u8,
    pub data: Vec<u8>, /* 96 elements */
}
impl DATA96_DATA {
    pub const ENCODED_LEN: usize = 98usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DATA96_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DATA96_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.mavtype = buf.get_u8();
        _struct.len = buf.get_u8();
        for _ in 0..96usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.mavtype);
        _tmp.put_u8(self.len);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RANGEFINDER_DATA {
    pub distance: f32,
    pub voltage: f32,
}
impl RANGEFINDER_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RANGEFINDER_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RANGEFINDER_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.distance = buf.get_f32_le();
        _struct.voltage = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.distance);
        _tmp.put_f32_le(self.voltage);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AIRSPEED_AUTOCAL_DATA {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
    pub diff_pressure: f32,
    pub EAS2TAS: f32,
    pub ratio: f32,
    pub state_x: f32,
    pub state_y: f32,
    pub state_z: f32,
    pub Pax: f32,
    pub Pby: f32,
    pub Pcz: f32,
}
impl AIRSPEED_AUTOCAL_DATA {
    pub const ENCODED_LEN: usize = 48usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AIRSPEED_AUTOCAL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AIRSPEED_AUTOCAL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.vx = buf.get_f32_le();
        _struct.vy = buf.get_f32_le();
        _struct.vz = buf.get_f32_le();
        _struct.diff_pressure = buf.get_f32_le();
        _struct.EAS2TAS = buf.get_f32_le();
        _struct.ratio = buf.get_f32_le();
        _struct.state_x = buf.get_f32_le();
        _struct.state_y = buf.get_f32_le();
        _struct.state_z = buf.get_f32_le();
        _struct.Pax = buf.get_f32_le();
        _struct.Pby = buf.get_f32_le();
        _struct.Pcz = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.vx);
        _tmp.put_f32_le(self.vy);
        _tmp.put_f32_le(self.vz);
        _tmp.put_f32_le(self.diff_pressure);
        _tmp.put_f32_le(self.EAS2TAS);
        _tmp.put_f32_le(self.ratio);
        _tmp.put_f32_le(self.state_x);
        _tmp.put_f32_le(self.state_y);
        _tmp.put_f32_le(self.state_z);
        _tmp.put_f32_le(self.Pax);
        _tmp.put_f32_le(self.Pby);
        _tmp.put_f32_le(self.Pcz);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RALLY_POINT_DATA {
    pub lat: i32,
    pub lng: i32,
    pub alt: i16,
    pub break_alt: i16,
    pub land_dir: u16,
    pub target_system: u8,
    pub target_component: u8,
    pub idx: u8,
    pub count: u8,
    pub flags: RallyFlags,
}
impl RALLY_POINT_DATA {
    pub const ENCODED_LEN: usize = 19usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RALLY_POINT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RALLY_POINT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.lat = buf.get_i32_le();
        _struct.lng = buf.get_i32_le();
        _struct.alt = buf.get_i16_le();
        _struct.break_alt = buf.get_i16_le();
        _struct.land_dir = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.idx = buf.get_u8();
        _struct.count = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.flags = RallyFlags::from_bits(tmp & RallyFlags::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "RallyFlags".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lng);
        _tmp.put_i16_le(self.alt);
        _tmp.put_i16_le(self.break_alt);
        _tmp.put_u16_le(self.land_dir);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.idx);
        _tmp.put_u8(self.count);
        _tmp.put_u8(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RALLY_FETCH_POINT_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub idx: u8,
}
impl RALLY_FETCH_POINT_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RALLY_FETCH_POINT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RALLY_FETCH_POINT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.idx = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.idx);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct COMPASSMOT_STATUS_DATA {
    pub current: f32,
    pub CompensationX: f32,
    pub CompensationY: f32,
    pub CompensationZ: f32,
    pub throttle: u16,
    pub interference: u16,
}
impl COMPASSMOT_STATUS_DATA {
    pub const ENCODED_LEN: usize = 20usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < COMPASSMOT_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; COMPASSMOT_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.current = buf.get_f32_le();
        _struct.CompensationX = buf.get_f32_le();
        _struct.CompensationY = buf.get_f32_le();
        _struct.CompensationZ = buf.get_f32_le();
        _struct.throttle = buf.get_u16_le();
        _struct.interference = buf.get_u16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.current);
        _tmp.put_f32_le(self.CompensationX);
        _tmp.put_f32_le(self.CompensationY);
        _tmp.put_f32_le(self.CompensationZ);
        _tmp.put_u16_le(self.throttle);
        _tmp.put_u16_le(self.interference);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AHRS2_DATA {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub altitude: f32,
    pub lat: i32,
    pub lng: i32,
}
impl AHRS2_DATA {
    pub const ENCODED_LEN: usize = 24usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AHRS2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AHRS2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.altitude = buf.get_f32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lng = buf.get_i32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.altitude);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lng);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_STATUS_DATA {
    pub time_usec: u64,
    pub p1: f32,
    pub p2: f32,
    pub p3: f32,
    pub p4: f32,
    pub img_idx: u16,
    pub target_system: u8,
    pub cam_idx: u8,
    pub event_id: CameraStatusTypes,
}
impl CAMERA_STATUS_DATA {
    pub const ENCODED_LEN: usize = 29usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.p1 = buf.get_f32_le();
        _struct.p2 = buf.get_f32_le();
        _struct.p3 = buf.get_f32_le();
        _struct.p4 = buf.get_f32_le();
        _struct.img_idx = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.cam_idx = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.event_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CameraStatusTypes".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.p1);
        _tmp.put_f32_le(self.p2);
        _tmp.put_f32_le(self.p3);
        _tmp.put_f32_le(self.p4);
        _tmp.put_u16_le(self.img_idx);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.cam_idx);
        _tmp.put_u8(self.event_id as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CAMERA_FEEDBACK_DATA {
    pub time_usec: u64,
    pub lat: i32,
    pub lng: i32,
    pub alt_msl: f32,
    pub alt_rel: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub foc_len: f32,
    pub img_idx: u16,
    pub target_system: u8,
    pub cam_idx: u8,
    pub flags: CameraFeedbackFlags,
}
impl CAMERA_FEEDBACK_DATA {
    pub const ENCODED_LEN: usize = 45usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < CAMERA_FEEDBACK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; CAMERA_FEEDBACK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.lat = buf.get_i32_le();
        _struct.lng = buf.get_i32_le();
        _struct.alt_msl = buf.get_f32_le();
        _struct.alt_rel = buf.get_f32_le();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.foc_len = buf.get_f32_le();
        _struct.img_idx = buf.get_u16_le();
        _struct.target_system = buf.get_u8();
        _struct.cam_idx = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.flags = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "CameraFeedbackFlags".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lng);
        _tmp.put_f32_le(self.alt_msl);
        _tmp.put_f32_le(self.alt_rel);
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.foc_len);
        _tmp.put_u16_le(self.img_idx);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.cam_idx);
        _tmp.put_u8(self.flags as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BATTERY2_DATA {
    pub voltage: u16,
    pub current_battery: i16,
}
impl BATTERY2_DATA {
    pub const ENCODED_LEN: usize = 4usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < BATTERY2_DATA::ENCODED_LEN {
            let mut payload_buf = [0; BATTERY2_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.voltage = buf.get_u16_le();
        _struct.current_battery = buf.get_i16_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u16_le(self.voltage);
        _tmp.put_i16_le(self.current_battery);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AHRS3_DATA {
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub altitude: f32,
    pub lat: i32,
    pub lng: i32,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
    pub v4: f32,
}
impl AHRS3_DATA {
    pub const ENCODED_LEN: usize = 40usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AHRS3_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AHRS3_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.roll = buf.get_f32_le();
        _struct.pitch = buf.get_f32_le();
        _struct.yaw = buf.get_f32_le();
        _struct.altitude = buf.get_f32_le();
        _struct.lat = buf.get_i32_le();
        _struct.lng = buf.get_i32_le();
        _struct.v1 = buf.get_f32_le();
        _struct.v2 = buf.get_f32_le();
        _struct.v3 = buf.get_f32_le();
        _struct.v4 = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.roll);
        _tmp.put_f32_le(self.pitch);
        _tmp.put_f32_le(self.yaw);
        _tmp.put_f32_le(self.altitude);
        _tmp.put_i32_le(self.lat);
        _tmp.put_i32_le(self.lng);
        _tmp.put_f32_le(self.v1);
        _tmp.put_f32_le(self.v2);
        _tmp.put_f32_le(self.v3);
        _tmp.put_f32_le(self.v4);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AUTOPILOT_VERSION_REQUEST_DATA {
    pub target_system: u8,
    pub target_component: u8,
}
impl AUTOPILOT_VERSION_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AUTOPILOT_VERSION_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AUTOPILOT_VERSION_REQUEST_DATA::ENCODED_LEN];
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
pub struct REMOTE_LOG_DATA_BLOCK_DATA {
    pub seqno: MavRemoteLogDataBlockCommands,
    pub target_system: u8,
    pub target_component: u8,
    pub data: Vec<u8>, /* 200 elements */
}
impl REMOTE_LOG_DATA_BLOCK_DATA {
    pub const ENCODED_LEN: usize = 206usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < REMOTE_LOG_DATA_BLOCK_DATA::ENCODED_LEN {
            let mut payload_buf = [0; REMOTE_LOG_DATA_BLOCK_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u32_le();
        _struct.seqno = FromPrimitive::from_u32(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavRemoteLogDataBlockCommands".to_string(),
            value: tmp as u32,
        })?;
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        for _ in 0..200usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.seqno as u32);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct REMOTE_LOG_BLOCK_STATUS_DATA {
    pub seqno: u32,
    pub target_system: u8,
    pub target_component: u8,
    pub status: MavRemoteLogDataBlockStatuses,
}
impl REMOTE_LOG_BLOCK_STATUS_DATA {
    pub const ENCODED_LEN: usize = 7usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < REMOTE_LOG_BLOCK_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; REMOTE_LOG_BLOCK_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.seqno = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MavRemoteLogDataBlockStatuses".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.seqno);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LED_CONTROL_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub instance: u8,
    pub pattern: u8,
    pub custom_len: u8,
    pub custom_bytes: [u8; 24],
}
impl LED_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 29usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < LED_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; LED_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        _struct.instance = buf.get_u8();
        _struct.pattern = buf.get_u8();
        _struct.custom_len = buf.get_u8();
        for idx in 0..24usize {
            let val = buf.get_u8();
            _struct.custom_bytes[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.instance);
        _tmp.put_u8(self.pattern);
        _tmp.put_u8(self.custom_len);
        for val in &self.custom_bytes {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MAG_CAL_PROGRESS_DATA {
    pub direction_x: f32,
    pub direction_y: f32,
    pub direction_z: f32,
    pub compass_id: u8,
    pub cal_mask: u8,
    pub cal_status: MagCalStatus,
    pub attempt: u8,
    pub completion_pct: u8,
    pub completion_mask: [u8; 10],
}
impl MAG_CAL_PROGRESS_DATA {
    pub const ENCODED_LEN: usize = 27usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MAG_CAL_PROGRESS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MAG_CAL_PROGRESS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.direction_x = buf.get_f32_le();
        _struct.direction_y = buf.get_f32_le();
        _struct.direction_z = buf.get_f32_le();
        _struct.compass_id = buf.get_u8();
        _struct.cal_mask = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.cal_status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MagCalStatus".to_string(),
            value: tmp as u32,
        })?;
        _struct.attempt = buf.get_u8();
        _struct.completion_pct = buf.get_u8();
        for idx in 0..10usize {
            let val = buf.get_u8();
            _struct.completion_mask[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.direction_x);
        _tmp.put_f32_le(self.direction_y);
        _tmp.put_f32_le(self.direction_z);
        _tmp.put_u8(self.compass_id);
        _tmp.put_u8(self.cal_mask);
        _tmp.put_u8(self.cal_status as u8);
        _tmp.put_u8(self.attempt);
        _tmp.put_u8(self.completion_pct);
        for val in &self.completion_mask {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MAG_CAL_REPORT_DATA {
    pub fitness: f32,
    pub ofs_x: f32,
    pub ofs_y: f32,
    pub ofs_z: f32,
    pub diag_x: f32,
    pub diag_y: f32,
    pub diag_z: f32,
    pub offdiag_x: f32,
    pub offdiag_y: f32,
    pub offdiag_z: f32,
    pub compass_id: u8,
    pub cal_mask: u8,
    pub cal_status: MagCalStatus,
    pub autosaved: u8,
}
impl MAG_CAL_REPORT_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < MAG_CAL_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; MAG_CAL_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.fitness = buf.get_f32_le();
        _struct.ofs_x = buf.get_f32_le();
        _struct.ofs_y = buf.get_f32_le();
        _struct.ofs_z = buf.get_f32_le();
        _struct.diag_x = buf.get_f32_le();
        _struct.diag_y = buf.get_f32_le();
        _struct.diag_z = buf.get_f32_le();
        _struct.offdiag_x = buf.get_f32_le();
        _struct.offdiag_y = buf.get_f32_le();
        _struct.offdiag_z = buf.get_f32_le();
        _struct.compass_id = buf.get_u8();
        _struct.cal_mask = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.cal_status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "MagCalStatus".to_string(),
            value: tmp as u32,
        })?;
        _struct.autosaved = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.fitness);
        _tmp.put_f32_le(self.ofs_x);
        _tmp.put_f32_le(self.ofs_y);
        _tmp.put_f32_le(self.ofs_z);
        _tmp.put_f32_le(self.diag_x);
        _tmp.put_f32_le(self.diag_y);
        _tmp.put_f32_le(self.diag_z);
        _tmp.put_f32_le(self.offdiag_x);
        _tmp.put_f32_le(self.offdiag_y);
        _tmp.put_f32_le(self.offdiag_z);
        _tmp.put_u8(self.compass_id);
        _tmp.put_u8(self.cal_mask);
        _tmp.put_u8(self.cal_status as u8);
        _tmp.put_u8(self.autosaved);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EKF_STATUS_REPORT_DATA {
    pub velocity_variance: f32,
    pub pos_horiz_variance: f32,
    pub pos_vert_variance: f32,
    pub compass_variance: f32,
    pub terrain_alt_variance: f32,
    pub flags: EkfStatusFlags,
}
impl EKF_STATUS_REPORT_DATA {
    pub const ENCODED_LEN: usize = 22usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < EKF_STATUS_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; EKF_STATUS_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.velocity_variance = buf.get_f32_le();
        _struct.pos_horiz_variance = buf.get_f32_le();
        _struct.pos_vert_variance = buf.get_f32_le();
        _struct.compass_variance = buf.get_f32_le();
        _struct.terrain_alt_variance = buf.get_f32_le();
        let tmp = buf.get_u16_le();
        _struct.flags = EkfStatusFlags::from_bits(tmp & EkfStatusFlags::all().bits()).ok_or(
            ParserError::InvalidFlag {
                flag_type: "EkfStatusFlags".to_string(),
                value: tmp as u32,
            },
        )?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.velocity_variance);
        _tmp.put_f32_le(self.pos_horiz_variance);
        _tmp.put_f32_le(self.pos_vert_variance);
        _tmp.put_f32_le(self.compass_variance);
        _tmp.put_f32_le(self.terrain_alt_variance);
        _tmp.put_u16_le(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PID_TUNING_DATA {
    pub desired: f32,
    pub achieved: f32,
    pub FF: f32,
    pub P: f32,
    pub I: f32,
    pub D: f32,
    pub axis: PidTuningAxis,
}
impl PID_TUNING_DATA {
    pub const ENCODED_LEN: usize = 25usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < PID_TUNING_DATA::ENCODED_LEN {
            let mut payload_buf = [0; PID_TUNING_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.desired = buf.get_f32_le();
        _struct.achieved = buf.get_f32_le();
        _struct.FF = buf.get_f32_le();
        _struct.P = buf.get_f32_le();
        _struct.I = buf.get_f32_le();
        _struct.D = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.axis = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "PidTuningAxis".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.desired);
        _tmp.put_f32_le(self.achieved);
        _tmp.put_f32_le(self.FF);
        _tmp.put_f32_le(self.P);
        _tmp.put_f32_le(self.I);
        _tmp.put_f32_le(self.D);
        _tmp.put_u8(self.axis as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEEPSTALL_DATA {
    pub landing_lat: i32,
    pub landing_lon: i32,
    pub path_lat: i32,
    pub path_lon: i32,
    pub arc_entry_lat: i32,
    pub arc_entry_lon: i32,
    pub altitude: f32,
    pub expected_travel_distance: f32,
    pub cross_track_error: f32,
    pub stage: DeepstallStage,
}
impl DEEPSTALL_DATA {
    pub const ENCODED_LEN: usize = 37usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEEPSTALL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEEPSTALL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.landing_lat = buf.get_i32_le();
        _struct.landing_lon = buf.get_i32_le();
        _struct.path_lat = buf.get_i32_le();
        _struct.path_lon = buf.get_i32_le();
        _struct.arc_entry_lat = buf.get_i32_le();
        _struct.arc_entry_lon = buf.get_i32_le();
        _struct.altitude = buf.get_f32_le();
        _struct.expected_travel_distance = buf.get_f32_le();
        _struct.cross_track_error = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.stage = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "DeepstallStage".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i32_le(self.landing_lat);
        _tmp.put_i32_le(self.landing_lon);
        _tmp.put_i32_le(self.path_lat);
        _tmp.put_i32_le(self.path_lon);
        _tmp.put_i32_le(self.arc_entry_lat);
        _tmp.put_i32_le(self.arc_entry_lon);
        _tmp.put_f32_le(self.altitude);
        _tmp.put_f32_le(self.expected_travel_distance);
        _tmp.put_f32_le(self.cross_track_error);
        _tmp.put_u8(self.stage as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_REPORT_DATA {
    pub delta_time: f32,
    pub delta_angle_x: f32,
    pub delta_angle_y: f32,
    pub delta_angle_z: f32,
    pub delta_velocity_x: f32,
    pub delta_velocity_y: f32,
    pub delta_velocity_z: f32,
    pub joint_roll: f32,
    pub joint_el: f32,
    pub joint_az: f32,
    pub target_system: u8,
    pub target_component: u8,
}
impl GIMBAL_REPORT_DATA {
    pub const ENCODED_LEN: usize = 42usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.delta_time = buf.get_f32_le();
        _struct.delta_angle_x = buf.get_f32_le();
        _struct.delta_angle_y = buf.get_f32_le();
        _struct.delta_angle_z = buf.get_f32_le();
        _struct.delta_velocity_x = buf.get_f32_le();
        _struct.delta_velocity_y = buf.get_f32_le();
        _struct.delta_velocity_z = buf.get_f32_le();
        _struct.joint_roll = buf.get_f32_le();
        _struct.joint_el = buf.get_f32_le();
        _struct.joint_az = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.delta_time);
        _tmp.put_f32_le(self.delta_angle_x);
        _tmp.put_f32_le(self.delta_angle_y);
        _tmp.put_f32_le(self.delta_angle_z);
        _tmp.put_f32_le(self.delta_velocity_x);
        _tmp.put_f32_le(self.delta_velocity_y);
        _tmp.put_f32_le(self.delta_velocity_z);
        _tmp.put_f32_le(self.joint_roll);
        _tmp.put_f32_le(self.joint_el);
        _tmp.put_f32_le(self.joint_az);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_CONTROL_DATA {
    pub demanded_rate_x: f32,
    pub demanded_rate_y: f32,
    pub demanded_rate_z: f32,
    pub target_system: u8,
    pub target_component: u8,
}
impl GIMBAL_CONTROL_DATA {
    pub const ENCODED_LEN: usize = 14usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_CONTROL_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_CONTROL_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.demanded_rate_x = buf.get_f32_le();
        _struct.demanded_rate_y = buf.get_f32_le();
        _struct.demanded_rate_z = buf.get_f32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.demanded_rate_x);
        _tmp.put_f32_le(self.demanded_rate_y);
        _tmp.put_f32_le(self.demanded_rate_z);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GIMBAL_TORQUE_CMD_REPORT_DATA {
    pub rl_torque_cmd: i16,
    pub el_torque_cmd: i16,
    pub az_torque_cmd: i16,
    pub target_system: u8,
    pub target_component: u8,
}
impl GIMBAL_TORQUE_CMD_REPORT_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GIMBAL_TORQUE_CMD_REPORT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GIMBAL_TORQUE_CMD_REPORT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.rl_torque_cmd = buf.get_i16_le();
        _struct.el_torque_cmd = buf.get_i16_le();
        _struct.az_torque_cmd = buf.get_i16_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_i16_le(self.rl_torque_cmd);
        _tmp.put_i16_le(self.el_torque_cmd);
        _tmp.put_i16_le(self.az_torque_cmd);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GOPRO_HEARTBEAT_DATA {
    pub status: GoproHeartbeatStatus,
    pub capture_mode: GoproCaptureMode,
    pub flags: GoproHeartbeatFlags,
}
impl GOPRO_HEARTBEAT_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GOPRO_HEARTBEAT_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GOPRO_HEARTBEAT_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproHeartbeatStatus".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.capture_mode = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproCaptureMode".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.flags = GoproHeartbeatFlags::from_bits(tmp & GoproHeartbeatFlags::all().bits())
            .ok_or(ParserError::InvalidFlag {
                flag_type: "GoproHeartbeatFlags".to_string(),
                value: tmp as u32,
            })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.status as u8);
        _tmp.put_u8(self.capture_mode as u8);
        _tmp.put_u8(self.flags.bits());
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GOPRO_GET_REQUEST_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub cmd_id: GoproCommand,
}
impl GOPRO_GET_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 3usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GOPRO_GET_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GOPRO_GET_REQUEST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproCommand".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.cmd_id as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GOPRO_GET_RESPONSE_DATA {
    pub cmd_id: GoproCommand,
    pub status: GoproRequestStatus,
    pub value: [u8; 4],
}
impl GOPRO_GET_RESPONSE_DATA {
    pub const ENCODED_LEN: usize = 6usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GOPRO_GET_RESPONSE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GOPRO_GET_RESPONSE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproCommand".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproRequestStatus".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.value[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.cmd_id as u8);
        _tmp.put_u8(self.status as u8);
        for val in &self.value {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GOPRO_SET_REQUEST_DATA {
    pub target_system: u8,
    pub target_component: u8,
    pub cmd_id: GoproCommand,
    pub value: [u8; 4],
}
impl GOPRO_SET_REQUEST_DATA {
    pub const ENCODED_LEN: usize = 7usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GOPRO_SET_REQUEST_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GOPRO_SET_REQUEST_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproCommand".to_string(),
            value: tmp as u32,
        })?;
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.value[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.cmd_id as u8);
        for val in &self.value {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GOPRO_SET_RESPONSE_DATA {
    pub cmd_id: GoproCommand,
    pub status: GoproRequestStatus,
}
impl GOPRO_SET_RESPONSE_DATA {
    pub const ENCODED_LEN: usize = 2usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < GOPRO_SET_RESPONSE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; GOPRO_SET_RESPONSE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        let tmp = buf.get_u8();
        _struct.cmd_id = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproCommand".to_string(),
            value: tmp as u32,
        })?;
        let tmp = buf.get_u8();
        _struct.status = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "GoproRequestStatus".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u8(self.cmd_id as u8);
        _tmp.put_u8(self.status as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EFI_STATUS_DATA {
    pub ecu_index: f32,
    pub rpm: f32,
    pub fuel_consumed: f32,
    pub fuel_flow: f32,
    pub engine_load: f32,
    pub throttle_position: f32,
    pub spark_dwell_time: f32,
    pub barometric_pressure: f32,
    pub intake_manifold_pressure: f32,
    pub intake_manifold_temperature: f32,
    pub cylinder_head_temperature: f32,
    pub ignition_timing: f32,
    pub injection_time: f32,
    pub exhaust_gas_temperature: f32,
    pub throttle_out: f32,
    pub pt_compensation: f32,
    pub health: u8,
}
impl EFI_STATUS_DATA {
    pub const ENCODED_LEN: usize = 65usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < EFI_STATUS_DATA::ENCODED_LEN {
            let mut payload_buf = [0; EFI_STATUS_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.ecu_index = buf.get_f32_le();
        _struct.rpm = buf.get_f32_le();
        _struct.fuel_consumed = buf.get_f32_le();
        _struct.fuel_flow = buf.get_f32_le();
        _struct.engine_load = buf.get_f32_le();
        _struct.throttle_position = buf.get_f32_le();
        _struct.spark_dwell_time = buf.get_f32_le();
        _struct.barometric_pressure = buf.get_f32_le();
        _struct.intake_manifold_pressure = buf.get_f32_le();
        _struct.intake_manifold_temperature = buf.get_f32_le();
        _struct.cylinder_head_temperature = buf.get_f32_le();
        _struct.ignition_timing = buf.get_f32_le();
        _struct.injection_time = buf.get_f32_le();
        _struct.exhaust_gas_temperature = buf.get_f32_le();
        _struct.throttle_out = buf.get_f32_le();
        _struct.pt_compensation = buf.get_f32_le();
        _struct.health = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.ecu_index);
        _tmp.put_f32_le(self.rpm);
        _tmp.put_f32_le(self.fuel_consumed);
        _tmp.put_f32_le(self.fuel_flow);
        _tmp.put_f32_le(self.engine_load);
        _tmp.put_f32_le(self.throttle_position);
        _tmp.put_f32_le(self.spark_dwell_time);
        _tmp.put_f32_le(self.barometric_pressure);
        _tmp.put_f32_le(self.intake_manifold_pressure);
        _tmp.put_f32_le(self.intake_manifold_temperature);
        _tmp.put_f32_le(self.cylinder_head_temperature);
        _tmp.put_f32_le(self.ignition_timing);
        _tmp.put_f32_le(self.injection_time);
        _tmp.put_f32_le(self.exhaust_gas_temperature);
        _tmp.put_f32_le(self.throttle_out);
        _tmp.put_f32_le(self.pt_compensation);
        _tmp.put_u8(self.health);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RPM_DATA {
    pub rpm1: f32,
    pub rpm2: f32,
}
impl RPM_DATA {
    pub const ENCODED_LEN: usize = 8usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < RPM_DATA::ENCODED_LEN {
            let mut payload_buf = [0; RPM_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.rpm1 = buf.get_f32_le();
        _struct.rpm2 = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.rpm1);
        _tmp.put_f32_le(self.rpm2);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEVICE_OP_READ_DATA {
    pub request_id: u32,
    pub target_system: u8,
    pub target_component: u8,
    pub bustype: DeviceOpBustype,
    pub bus: u8,
    pub address: u8,
    pub busname: Vec<char>, /* 40 elements */
    pub regstart: u8,
    pub count: u8,
}
impl DEVICE_OP_READ_DATA {
    pub const ENCODED_LEN: usize = 51usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEVICE_OP_READ_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEVICE_OP_READ_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.request_id = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.bustype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "DeviceOpBustype".to_string(),
            value: tmp as u32,
        })?;
        _struct.bus = buf.get_u8();
        _struct.address = buf.get_u8();
        for _ in 0..40usize {
            let val = buf.get_u8() as char;
            _struct.busname.push(val);
        }
        _struct.regstart = buf.get_u8();
        _struct.count = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.bustype as u8);
        _tmp.put_u8(self.bus);
        _tmp.put_u8(self.address);
        for val in &self.busname {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.regstart);
        _tmp.put_u8(self.count);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEVICE_OP_READ_REPLY_DATA {
    pub request_id: u32,
    pub result: u8,
    pub regstart: u8,
    pub count: u8,
    pub data: Vec<u8>, /* 128 elements */
}
impl DEVICE_OP_READ_REPLY_DATA {
    pub const ENCODED_LEN: usize = 135usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEVICE_OP_READ_REPLY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEVICE_OP_READ_REPLY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.request_id = buf.get_u32_le();
        _struct.result = buf.get_u8();
        _struct.regstart = buf.get_u8();
        _struct.count = buf.get_u8();
        for _ in 0..128usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id);
        _tmp.put_u8(self.result);
        _tmp.put_u8(self.regstart);
        _tmp.put_u8(self.count);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEVICE_OP_WRITE_DATA {
    pub request_id: u32,
    pub target_system: u8,
    pub target_component: u8,
    pub bustype: DeviceOpBustype,
    pub bus: u8,
    pub address: u8,
    pub busname: Vec<char>, /* 40 elements */
    pub regstart: u8,
    pub count: u8,
    pub data: Vec<u8>, /* 128 elements */
}
impl DEVICE_OP_WRITE_DATA {
    pub const ENCODED_LEN: usize = 179usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEVICE_OP_WRITE_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEVICE_OP_WRITE_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.request_id = buf.get_u32_le();
        _struct.target_system = buf.get_u8();
        _struct.target_component = buf.get_u8();
        let tmp = buf.get_u8();
        _struct.bustype = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "DeviceOpBustype".to_string(),
            value: tmp as u32,
        })?;
        _struct.bus = buf.get_u8();
        _struct.address = buf.get_u8();
        for _ in 0..40usize {
            let val = buf.get_u8() as char;
            _struct.busname.push(val);
        }
        _struct.regstart = buf.get_u8();
        _struct.count = buf.get_u8();
        for _ in 0..128usize {
            let val = buf.get_u8();
            _struct.data.push(val);
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id);
        _tmp.put_u8(self.target_system);
        _tmp.put_u8(self.target_component);
        _tmp.put_u8(self.bustype as u8);
        _tmp.put_u8(self.bus);
        _tmp.put_u8(self.address);
        for val in &self.busname {
            _tmp.put_u8(*val as u8);
        }
        _tmp.put_u8(self.regstart);
        _tmp.put_u8(self.count);
        for val in &self.data {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DEVICE_OP_WRITE_REPLY_DATA {
    pub request_id: u32,
    pub result: u8,
}
impl DEVICE_OP_WRITE_REPLY_DATA {
    pub const ENCODED_LEN: usize = 5usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < DEVICE_OP_WRITE_REPLY_DATA::ENCODED_LEN {
            let mut payload_buf = [0; DEVICE_OP_WRITE_REPLY_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.request_id = buf.get_u32_le();
        _struct.result = buf.get_u8();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u32_le(self.request_id);
        _tmp.put_u8(self.result);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ADAP_TUNING_DATA {
    pub desired: f32,
    pub achieved: f32,
    pub error: f32,
    pub theta: f32,
    pub omega: f32,
    pub sigma: f32,
    pub theta_dot: f32,
    pub omega_dot: f32,
    pub sigma_dot: f32,
    pub f: f32,
    pub f_dot: f32,
    pub u: f32,
    pub axis: PidTuningAxis,
}
impl ADAP_TUNING_DATA {
    pub const ENCODED_LEN: usize = 49usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ADAP_TUNING_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ADAP_TUNING_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.desired = buf.get_f32_le();
        _struct.achieved = buf.get_f32_le();
        _struct.error = buf.get_f32_le();
        _struct.theta = buf.get_f32_le();
        _struct.omega = buf.get_f32_le();
        _struct.sigma = buf.get_f32_le();
        _struct.theta_dot = buf.get_f32_le();
        _struct.omega_dot = buf.get_f32_le();
        _struct.sigma_dot = buf.get_f32_le();
        _struct.f = buf.get_f32_le();
        _struct.f_dot = buf.get_f32_le();
        _struct.u = buf.get_f32_le();
        let tmp = buf.get_u8();
        _struct.axis = FromPrimitive::from_u8(tmp).ok_or(ParserError::InvalidEnum {
            enum_type: "PidTuningAxis".to_string(),
            value: tmp as u32,
        })?;
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_f32_le(self.desired);
        _tmp.put_f32_le(self.achieved);
        _tmp.put_f32_le(self.error);
        _tmp.put_f32_le(self.theta);
        _tmp.put_f32_le(self.omega);
        _tmp.put_f32_le(self.sigma);
        _tmp.put_f32_le(self.theta_dot);
        _tmp.put_f32_le(self.omega_dot);
        _tmp.put_f32_le(self.sigma_dot);
        _tmp.put_f32_le(self.f);
        _tmp.put_f32_le(self.f_dot);
        _tmp.put_f32_le(self.u);
        _tmp.put_u8(self.axis as u8);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VISION_POSITION_DELTA_DATA {
    pub time_usec: u64,
    pub time_delta_usec: u64,
    pub angle_delta: [f32; 3],
    pub position_delta: [f32; 3],
    pub confidence: f32,
}
impl VISION_POSITION_DELTA_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < VISION_POSITION_DELTA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; VISION_POSITION_DELTA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.time_delta_usec = buf.get_u64_le();
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.angle_delta[idx] = val;
        }
        for idx in 0..3usize {
            let val = buf.get_f32_le();
            _struct.position_delta[idx] = val;
        }
        _struct.confidence = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_u64_le(self.time_delta_usec);
        for val in &self.angle_delta {
            _tmp.put_f32_le(*val);
        }
        for val in &self.position_delta {
            _tmp.put_f32_le(*val);
        }
        _tmp.put_f32_le(self.confidence);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AOA_SSA_DATA {
    pub time_usec: u64,
    pub AOA: f32,
    pub SSA: f32,
}
impl AOA_SSA_DATA {
    pub const ENCODED_LEN: usize = 16usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < AOA_SSA_DATA::ENCODED_LEN {
            let mut payload_buf = [0; AOA_SSA_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        _struct.time_usec = buf.get_u64_le();
        _struct.AOA = buf.get_f32_le();
        _struct.SSA = buf.get_f32_le();
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        _tmp.put_u64_le(self.time_usec);
        _tmp.put_f32_le(self.AOA);
        _tmp.put_f32_le(self.SSA);
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ESC_TELEMETRY_1_TO_4_DATA {
    pub voltage: [u16; 4],
    pub current: [u16; 4],
    pub totalcurrent: [u16; 4],
    pub rpm: [u16; 4],
    pub count: [u16; 4],
    pub temperature: [u8; 4],
}
impl ESC_TELEMETRY_1_TO_4_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ESC_TELEMETRY_1_TO_4_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ESC_TELEMETRY_1_TO_4_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.voltage[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.current[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.totalcurrent[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.rpm[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.count[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.temperature[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ESC_TELEMETRY_5_TO_8_DATA {
    pub voltage: [u16; 4],
    pub current: [u16; 4],
    pub totalcurrent: [u16; 4],
    pub rpm: [u16; 4],
    pub count: [u16; 4],
    pub temperature: [u8; 4],
}
impl ESC_TELEMETRY_5_TO_8_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ESC_TELEMETRY_5_TO_8_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ESC_TELEMETRY_5_TO_8_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.voltage[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.current[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.totalcurrent[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.rpm[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.count[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.temperature[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ESC_TELEMETRY_9_TO_12_DATA {
    pub voltage: [u16; 4],
    pub current: [u16; 4],
    pub totalcurrent: [u16; 4],
    pub rpm: [u16; 4],
    pub count: [u16; 4],
    pub temperature: [u8; 4],
}
impl ESC_TELEMETRY_9_TO_12_DATA {
    pub const ENCODED_LEN: usize = 44usize;
    pub fn deser(version: MavlinkVersion, _input: &[u8]) -> Result<Self, ParserError> {
        let avail_len = _input.len();
        let mut buf = BytesMut::from(_input);
        if avail_len < ESC_TELEMETRY_9_TO_12_DATA::ENCODED_LEN {
            let mut payload_buf = [0; ESC_TELEMETRY_9_TO_12_DATA::ENCODED_LEN];
            payload_buf[0..avail_len].copy_from_slice(_input);
            buf = BytesMut::from(&payload_buf[..]);
        }
        let mut _struct = Self::default();
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.voltage[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.current[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.totalcurrent[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.rpm[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u16_le();
            _struct.count[idx] = val;
        }
        for idx in 0..4usize {
            let val = buf.get_u8();
            _struct.temperature[idx] = val;
        }
        Ok(_struct)
    }
    pub fn ser(&self) -> Vec<u8> {
        let mut _tmp = Vec::new();
        for val in &self.voltage {
            _tmp.put_u16_le(*val);
        }
        for val in &self.current {
            _tmp.put_u16_le(*val);
        }
        for val in &self.totalcurrent {
            _tmp.put_u16_le(*val);
        }
        for val in &self.rpm {
            _tmp.put_u16_le(*val);
        }
        for val in &self.count {
            _tmp.put_u16_le(*val);
        }
        for val in &self.temperature {
            _tmp.put_u8(*val);
        }
        _tmp
    }
}
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum MavMessage {
    SENSOR_OFFSETS(SENSOR_OFFSETS_DATA),
    SET_MAG_OFFSETS(SET_MAG_OFFSETS_DATA),
    MEMINFO(MEMINFO_DATA),
    AP_ADC(AP_ADC_DATA),
    DIGICAM_CONFIGURE(DIGICAM_CONFIGURE_DATA),
    DIGICAM_CONTROL(DIGICAM_CONTROL_DATA),
    MOUNT_CONFIGURE(MOUNT_CONFIGURE_DATA),
    MOUNT_CONTROL(MOUNT_CONTROL_DATA),
    MOUNT_STATUS(MOUNT_STATUS_DATA),
    FENCE_POINT(FENCE_POINT_DATA),
    FENCE_FETCH_POINT(FENCE_FETCH_POINT_DATA),
    AHRS(AHRS_DATA),
    SIMSTATE(SIMSTATE_DATA),
    HWSTATUS(HWSTATUS_DATA),
    RADIO(RADIO_DATA),
    LIMITS_STATUS(LIMITS_STATUS_DATA),
    WIND(WIND_DATA),
    DATA16(DATA16_DATA),
    DATA32(DATA32_DATA),
    DATA64(DATA64_DATA),
    DATA96(DATA96_DATA),
    RANGEFINDER(RANGEFINDER_DATA),
    AIRSPEED_AUTOCAL(AIRSPEED_AUTOCAL_DATA),
    RALLY_POINT(RALLY_POINT_DATA),
    RALLY_FETCH_POINT(RALLY_FETCH_POINT_DATA),
    COMPASSMOT_STATUS(COMPASSMOT_STATUS_DATA),
    AHRS2(AHRS2_DATA),
    CAMERA_STATUS(CAMERA_STATUS_DATA),
    CAMERA_FEEDBACK(CAMERA_FEEDBACK_DATA),
    BATTERY2(BATTERY2_DATA),
    AHRS3(AHRS3_DATA),
    AUTOPILOT_VERSION_REQUEST(AUTOPILOT_VERSION_REQUEST_DATA),
    REMOTE_LOG_DATA_BLOCK(REMOTE_LOG_DATA_BLOCK_DATA),
    REMOTE_LOG_BLOCK_STATUS(REMOTE_LOG_BLOCK_STATUS_DATA),
    LED_CONTROL(LED_CONTROL_DATA),
    MAG_CAL_PROGRESS(MAG_CAL_PROGRESS_DATA),
    MAG_CAL_REPORT(MAG_CAL_REPORT_DATA),
    EKF_STATUS_REPORT(EKF_STATUS_REPORT_DATA),
    PID_TUNING(PID_TUNING_DATA),
    DEEPSTALL(DEEPSTALL_DATA),
    GIMBAL_REPORT(GIMBAL_REPORT_DATA),
    GIMBAL_CONTROL(GIMBAL_CONTROL_DATA),
    GIMBAL_TORQUE_CMD_REPORT(GIMBAL_TORQUE_CMD_REPORT_DATA),
    GOPRO_HEARTBEAT(GOPRO_HEARTBEAT_DATA),
    GOPRO_GET_REQUEST(GOPRO_GET_REQUEST_DATA),
    GOPRO_GET_RESPONSE(GOPRO_GET_RESPONSE_DATA),
    GOPRO_SET_REQUEST(GOPRO_SET_REQUEST_DATA),
    GOPRO_SET_RESPONSE(GOPRO_SET_RESPONSE_DATA),
    EFI_STATUS(EFI_STATUS_DATA),
    RPM(RPM_DATA),
    DEVICE_OP_READ(DEVICE_OP_READ_DATA),
    DEVICE_OP_READ_REPLY(DEVICE_OP_READ_REPLY_DATA),
    DEVICE_OP_WRITE(DEVICE_OP_WRITE_DATA),
    DEVICE_OP_WRITE_REPLY(DEVICE_OP_WRITE_REPLY_DATA),
    ADAP_TUNING(ADAP_TUNING_DATA),
    VISION_POSITION_DELTA(VISION_POSITION_DELTA_DATA),
    AOA_SSA(AOA_SSA_DATA),
    ESC_TELEMETRY_1_TO_4(ESC_TELEMETRY_1_TO_4_DATA),
    ESC_TELEMETRY_5_TO_8(ESC_TELEMETRY_5_TO_8_DATA),
    ESC_TELEMETRY_9_TO_12(ESC_TELEMETRY_9_TO_12_DATA),
    common(crate::common::MavMessage),
    uavionix(crate::uavionix::MavMessage),
    icarous(crate::icarous::MavMessage),
}
impl From<crate::common::MavMessage> for MavMessage {
    fn from(message: crate::common::MavMessage) -> Self {
        MavMessage::common(message)
    }
}
impl From<crate::uavionix::MavMessage> for MavMessage {
    fn from(message: crate::uavionix::MavMessage) -> Self {
        MavMessage::uavionix(message)
    }
}
impl From<crate::icarous::MavMessage> for MavMessage {
    fn from(message: crate::icarous::MavMessage) -> Self {
        MavMessage::icarous(message)
    }
}
impl Message for MavMessage {
    fn parse(version: MavlinkVersion, id: u32, payload: &[u8]) -> Result<MavMessage, ParserError> {
        match id {
            150 => {
                SENSOR_OFFSETS_DATA::deser(version, payload).map(|s| MavMessage::SENSOR_OFFSETS(s))
            }
            151 => SET_MAG_OFFSETS_DATA::deser(version, payload)
                .map(|s| MavMessage::SET_MAG_OFFSETS(s)),
            152 => MEMINFO_DATA::deser(version, payload).map(|s| MavMessage::MEMINFO(s)),
            153 => AP_ADC_DATA::deser(version, payload).map(|s| MavMessage::AP_ADC(s)),
            154 => DIGICAM_CONFIGURE_DATA::deser(version, payload)
                .map(|s| MavMessage::DIGICAM_CONFIGURE(s)),
            155 => DIGICAM_CONTROL_DATA::deser(version, payload)
                .map(|s| MavMessage::DIGICAM_CONTROL(s)),
            156 => MOUNT_CONFIGURE_DATA::deser(version, payload)
                .map(|s| MavMessage::MOUNT_CONFIGURE(s)),
            157 => {
                MOUNT_CONTROL_DATA::deser(version, payload).map(|s| MavMessage::MOUNT_CONTROL(s))
            }
            158 => MOUNT_STATUS_DATA::deser(version, payload).map(|s| MavMessage::MOUNT_STATUS(s)),
            160 => FENCE_POINT_DATA::deser(version, payload).map(|s| MavMessage::FENCE_POINT(s)),
            161 => FENCE_FETCH_POINT_DATA::deser(version, payload)
                .map(|s| MavMessage::FENCE_FETCH_POINT(s)),
            163 => AHRS_DATA::deser(version, payload).map(|s| MavMessage::AHRS(s)),
            164 => SIMSTATE_DATA::deser(version, payload).map(|s| MavMessage::SIMSTATE(s)),
            165 => HWSTATUS_DATA::deser(version, payload).map(|s| MavMessage::HWSTATUS(s)),
            166 => RADIO_DATA::deser(version, payload).map(|s| MavMessage::RADIO(s)),
            167 => {
                LIMITS_STATUS_DATA::deser(version, payload).map(|s| MavMessage::LIMITS_STATUS(s))
            }
            168 => WIND_DATA::deser(version, payload).map(|s| MavMessage::WIND(s)),
            169 => DATA16_DATA::deser(version, payload).map(|s| MavMessage::DATA16(s)),
            170 => DATA32_DATA::deser(version, payload).map(|s| MavMessage::DATA32(s)),
            171 => DATA64_DATA::deser(version, payload).map(|s| MavMessage::DATA64(s)),
            172 => DATA96_DATA::deser(version, payload).map(|s| MavMessage::DATA96(s)),
            173 => RANGEFINDER_DATA::deser(version, payload).map(|s| MavMessage::RANGEFINDER(s)),
            174 => AIRSPEED_AUTOCAL_DATA::deser(version, payload)
                .map(|s| MavMessage::AIRSPEED_AUTOCAL(s)),
            175 => RALLY_POINT_DATA::deser(version, payload).map(|s| MavMessage::RALLY_POINT(s)),
            176 => RALLY_FETCH_POINT_DATA::deser(version, payload)
                .map(|s| MavMessage::RALLY_FETCH_POINT(s)),
            177 => COMPASSMOT_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::COMPASSMOT_STATUS(s)),
            178 => AHRS2_DATA::deser(version, payload).map(|s| MavMessage::AHRS2(s)),
            179 => {
                CAMERA_STATUS_DATA::deser(version, payload).map(|s| MavMessage::CAMERA_STATUS(s))
            }
            180 => CAMERA_FEEDBACK_DATA::deser(version, payload)
                .map(|s| MavMessage::CAMERA_FEEDBACK(s)),
            181 => BATTERY2_DATA::deser(version, payload).map(|s| MavMessage::BATTERY2(s)),
            182 => AHRS3_DATA::deser(version, payload).map(|s| MavMessage::AHRS3(s)),
            183 => AUTOPILOT_VERSION_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::AUTOPILOT_VERSION_REQUEST(s)),
            184 => REMOTE_LOG_DATA_BLOCK_DATA::deser(version, payload)
                .map(|s| MavMessage::REMOTE_LOG_DATA_BLOCK(s)),
            185 => REMOTE_LOG_BLOCK_STATUS_DATA::deser(version, payload)
                .map(|s| MavMessage::REMOTE_LOG_BLOCK_STATUS(s)),
            186 => LED_CONTROL_DATA::deser(version, payload).map(|s| MavMessage::LED_CONTROL(s)),
            191 => MAG_CAL_PROGRESS_DATA::deser(version, payload)
                .map(|s| MavMessage::MAG_CAL_PROGRESS(s)),
            192 => {
                MAG_CAL_REPORT_DATA::deser(version, payload).map(|s| MavMessage::MAG_CAL_REPORT(s))
            }
            193 => EKF_STATUS_REPORT_DATA::deser(version, payload)
                .map(|s| MavMessage::EKF_STATUS_REPORT(s)),
            194 => PID_TUNING_DATA::deser(version, payload).map(|s| MavMessage::PID_TUNING(s)),
            195 => DEEPSTALL_DATA::deser(version, payload).map(|s| MavMessage::DEEPSTALL(s)),
            200 => {
                GIMBAL_REPORT_DATA::deser(version, payload).map(|s| MavMessage::GIMBAL_REPORT(s))
            }
            201 => {
                GIMBAL_CONTROL_DATA::deser(version, payload).map(|s| MavMessage::GIMBAL_CONTROL(s))
            }
            214 => GIMBAL_TORQUE_CMD_REPORT_DATA::deser(version, payload)
                .map(|s| MavMessage::GIMBAL_TORQUE_CMD_REPORT(s)),
            215 => GOPRO_HEARTBEAT_DATA::deser(version, payload)
                .map(|s| MavMessage::GOPRO_HEARTBEAT(s)),
            216 => GOPRO_GET_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::GOPRO_GET_REQUEST(s)),
            217 => GOPRO_GET_RESPONSE_DATA::deser(version, payload)
                .map(|s| MavMessage::GOPRO_GET_RESPONSE(s)),
            218 => GOPRO_SET_REQUEST_DATA::deser(version, payload)
                .map(|s| MavMessage::GOPRO_SET_REQUEST(s)),
            219 => GOPRO_SET_RESPONSE_DATA::deser(version, payload)
                .map(|s| MavMessage::GOPRO_SET_RESPONSE(s)),
            225 => EFI_STATUS_DATA::deser(version, payload).map(|s| MavMessage::EFI_STATUS(s)),
            226 => RPM_DATA::deser(version, payload).map(|s| MavMessage::RPM(s)),
            11000 => {
                DEVICE_OP_READ_DATA::deser(version, payload).map(|s| MavMessage::DEVICE_OP_READ(s))
            }
            11001 => DEVICE_OP_READ_REPLY_DATA::deser(version, payload)
                .map(|s| MavMessage::DEVICE_OP_READ_REPLY(s)),
            11002 => DEVICE_OP_WRITE_DATA::deser(version, payload)
                .map(|s| MavMessage::DEVICE_OP_WRITE(s)),
            11003 => DEVICE_OP_WRITE_REPLY_DATA::deser(version, payload)
                .map(|s| MavMessage::DEVICE_OP_WRITE_REPLY(s)),
            11010 => ADAP_TUNING_DATA::deser(version, payload).map(|s| MavMessage::ADAP_TUNING(s)),
            11011 => VISION_POSITION_DELTA_DATA::deser(version, payload)
                .map(|s| MavMessage::VISION_POSITION_DELTA(s)),
            11020 => AOA_SSA_DATA::deser(version, payload).map(|s| MavMessage::AOA_SSA(s)),
            11030 => ESC_TELEMETRY_1_TO_4_DATA::deser(version, payload)
                .map(|s| MavMessage::ESC_TELEMETRY_1_TO_4(s)),
            11031 => ESC_TELEMETRY_5_TO_8_DATA::deser(version, payload)
                .map(|s| MavMessage::ESC_TELEMETRY_5_TO_8(s)),
            11032 => ESC_TELEMETRY_9_TO_12_DATA::deser(version, payload)
                .map(|s| MavMessage::ESC_TELEMETRY_9_TO_12(s)),
            _ => {
                if let Ok(msg) = crate::common::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::common(msg));
                }
                if let Ok(msg) = crate::uavionix::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::uavionix(msg));
                }
                if let Ok(msg) = crate::icarous::MavMessage::parse(version, id, payload) {
                    return Ok(MavMessage::icarous(msg));
                }
                Err(ParserError::UnknownMessage { id })
            }
        }
    }
    fn message_name(&self) -> &'static str {
        match self {
            MavMessage::SENSOR_OFFSETS(..) => "SENSOR_OFFSETS",
            MavMessage::SET_MAG_OFFSETS(..) => "SET_MAG_OFFSETS",
            MavMessage::MEMINFO(..) => "MEMINFO",
            MavMessage::AP_ADC(..) => "AP_ADC",
            MavMessage::DIGICAM_CONFIGURE(..) => "DIGICAM_CONFIGURE",
            MavMessage::DIGICAM_CONTROL(..) => "DIGICAM_CONTROL",
            MavMessage::MOUNT_CONFIGURE(..) => "MOUNT_CONFIGURE",
            MavMessage::MOUNT_CONTROL(..) => "MOUNT_CONTROL",
            MavMessage::MOUNT_STATUS(..) => "MOUNT_STATUS",
            MavMessage::FENCE_POINT(..) => "FENCE_POINT",
            MavMessage::FENCE_FETCH_POINT(..) => "FENCE_FETCH_POINT",
            MavMessage::AHRS(..) => "AHRS",
            MavMessage::SIMSTATE(..) => "SIMSTATE",
            MavMessage::HWSTATUS(..) => "HWSTATUS",
            MavMessage::RADIO(..) => "RADIO",
            MavMessage::LIMITS_STATUS(..) => "LIMITS_STATUS",
            MavMessage::WIND(..) => "WIND",
            MavMessage::DATA16(..) => "DATA16",
            MavMessage::DATA32(..) => "DATA32",
            MavMessage::DATA64(..) => "DATA64",
            MavMessage::DATA96(..) => "DATA96",
            MavMessage::RANGEFINDER(..) => "RANGEFINDER",
            MavMessage::AIRSPEED_AUTOCAL(..) => "AIRSPEED_AUTOCAL",
            MavMessage::RALLY_POINT(..) => "RALLY_POINT",
            MavMessage::RALLY_FETCH_POINT(..) => "RALLY_FETCH_POINT",
            MavMessage::COMPASSMOT_STATUS(..) => "COMPASSMOT_STATUS",
            MavMessage::AHRS2(..) => "AHRS2",
            MavMessage::CAMERA_STATUS(..) => "CAMERA_STATUS",
            MavMessage::CAMERA_FEEDBACK(..) => "CAMERA_FEEDBACK",
            MavMessage::BATTERY2(..) => "BATTERY2",
            MavMessage::AHRS3(..) => "AHRS3",
            MavMessage::AUTOPILOT_VERSION_REQUEST(..) => "AUTOPILOT_VERSION_REQUEST",
            MavMessage::REMOTE_LOG_DATA_BLOCK(..) => "REMOTE_LOG_DATA_BLOCK",
            MavMessage::REMOTE_LOG_BLOCK_STATUS(..) => "REMOTE_LOG_BLOCK_STATUS",
            MavMessage::LED_CONTROL(..) => "LED_CONTROL",
            MavMessage::MAG_CAL_PROGRESS(..) => "MAG_CAL_PROGRESS",
            MavMessage::MAG_CAL_REPORT(..) => "MAG_CAL_REPORT",
            MavMessage::EKF_STATUS_REPORT(..) => "EKF_STATUS_REPORT",
            MavMessage::PID_TUNING(..) => "PID_TUNING",
            MavMessage::DEEPSTALL(..) => "DEEPSTALL",
            MavMessage::GIMBAL_REPORT(..) => "GIMBAL_REPORT",
            MavMessage::GIMBAL_CONTROL(..) => "GIMBAL_CONTROL",
            MavMessage::GIMBAL_TORQUE_CMD_REPORT(..) => "GIMBAL_TORQUE_CMD_REPORT",
            MavMessage::GOPRO_HEARTBEAT(..) => "GOPRO_HEARTBEAT",
            MavMessage::GOPRO_GET_REQUEST(..) => "GOPRO_GET_REQUEST",
            MavMessage::GOPRO_GET_RESPONSE(..) => "GOPRO_GET_RESPONSE",
            MavMessage::GOPRO_SET_REQUEST(..) => "GOPRO_SET_REQUEST",
            MavMessage::GOPRO_SET_RESPONSE(..) => "GOPRO_SET_RESPONSE",
            MavMessage::EFI_STATUS(..) => "EFI_STATUS",
            MavMessage::RPM(..) => "RPM",
            MavMessage::DEVICE_OP_READ(..) => "DEVICE_OP_READ",
            MavMessage::DEVICE_OP_READ_REPLY(..) => "DEVICE_OP_READ_REPLY",
            MavMessage::DEVICE_OP_WRITE(..) => "DEVICE_OP_WRITE",
            MavMessage::DEVICE_OP_WRITE_REPLY(..) => "DEVICE_OP_WRITE_REPLY",
            MavMessage::ADAP_TUNING(..) => "ADAP_TUNING",
            MavMessage::VISION_POSITION_DELTA(..) => "VISION_POSITION_DELTA",
            MavMessage::AOA_SSA(..) => "AOA_SSA",
            MavMessage::ESC_TELEMETRY_1_TO_4(..) => "ESC_TELEMETRY_1_TO_4",
            MavMessage::ESC_TELEMETRY_5_TO_8(..) => "ESC_TELEMETRY_5_TO_8",
            MavMessage::ESC_TELEMETRY_9_TO_12(..) => "ESC_TELEMETRY_9_TO_12",
            MavMessage::common(msg) => msg.message_name(),
            MavMessage::uavionix(msg) => msg.message_name(),
            MavMessage::icarous(msg) => msg.message_name(),
        }
    }
    fn message_id(&self) -> u32 {
        match self {
            MavMessage::SENSOR_OFFSETS(..) => 150,
            MavMessage::SET_MAG_OFFSETS(..) => 151,
            MavMessage::MEMINFO(..) => 152,
            MavMessage::AP_ADC(..) => 153,
            MavMessage::DIGICAM_CONFIGURE(..) => 154,
            MavMessage::DIGICAM_CONTROL(..) => 155,
            MavMessage::MOUNT_CONFIGURE(..) => 156,
            MavMessage::MOUNT_CONTROL(..) => 157,
            MavMessage::MOUNT_STATUS(..) => 158,
            MavMessage::FENCE_POINT(..) => 160,
            MavMessage::FENCE_FETCH_POINT(..) => 161,
            MavMessage::AHRS(..) => 163,
            MavMessage::SIMSTATE(..) => 164,
            MavMessage::HWSTATUS(..) => 165,
            MavMessage::RADIO(..) => 166,
            MavMessage::LIMITS_STATUS(..) => 167,
            MavMessage::WIND(..) => 168,
            MavMessage::DATA16(..) => 169,
            MavMessage::DATA32(..) => 170,
            MavMessage::DATA64(..) => 171,
            MavMessage::DATA96(..) => 172,
            MavMessage::RANGEFINDER(..) => 173,
            MavMessage::AIRSPEED_AUTOCAL(..) => 174,
            MavMessage::RALLY_POINT(..) => 175,
            MavMessage::RALLY_FETCH_POINT(..) => 176,
            MavMessage::COMPASSMOT_STATUS(..) => 177,
            MavMessage::AHRS2(..) => 178,
            MavMessage::CAMERA_STATUS(..) => 179,
            MavMessage::CAMERA_FEEDBACK(..) => 180,
            MavMessage::BATTERY2(..) => 181,
            MavMessage::AHRS3(..) => 182,
            MavMessage::AUTOPILOT_VERSION_REQUEST(..) => 183,
            MavMessage::REMOTE_LOG_DATA_BLOCK(..) => 184,
            MavMessage::REMOTE_LOG_BLOCK_STATUS(..) => 185,
            MavMessage::LED_CONTROL(..) => 186,
            MavMessage::MAG_CAL_PROGRESS(..) => 191,
            MavMessage::MAG_CAL_REPORT(..) => 192,
            MavMessage::EKF_STATUS_REPORT(..) => 193,
            MavMessage::PID_TUNING(..) => 194,
            MavMessage::DEEPSTALL(..) => 195,
            MavMessage::GIMBAL_REPORT(..) => 200,
            MavMessage::GIMBAL_CONTROL(..) => 201,
            MavMessage::GIMBAL_TORQUE_CMD_REPORT(..) => 214,
            MavMessage::GOPRO_HEARTBEAT(..) => 215,
            MavMessage::GOPRO_GET_REQUEST(..) => 216,
            MavMessage::GOPRO_GET_RESPONSE(..) => 217,
            MavMessage::GOPRO_SET_REQUEST(..) => 218,
            MavMessage::GOPRO_SET_RESPONSE(..) => 219,
            MavMessage::EFI_STATUS(..) => 225,
            MavMessage::RPM(..) => 226,
            MavMessage::DEVICE_OP_READ(..) => 11000,
            MavMessage::DEVICE_OP_READ_REPLY(..) => 11001,
            MavMessage::DEVICE_OP_WRITE(..) => 11002,
            MavMessage::DEVICE_OP_WRITE_REPLY(..) => 11003,
            MavMessage::ADAP_TUNING(..) => 11010,
            MavMessage::VISION_POSITION_DELTA(..) => 11011,
            MavMessage::AOA_SSA(..) => 11020,
            MavMessage::ESC_TELEMETRY_1_TO_4(..) => 11030,
            MavMessage::ESC_TELEMETRY_5_TO_8(..) => 11031,
            MavMessage::ESC_TELEMETRY_9_TO_12(..) => 11032,
            MavMessage::common(msg) => msg.message_id(),
            MavMessage::uavionix(msg) => msg.message_id(),
            MavMessage::icarous(msg) => msg.message_id(),
        }
    }
    fn message_id_from_name(name: &str) -> Result<u32, &'static str> {
        match name {
            "SENSOR_OFFSETS" => Ok(150),
            "SET_MAG_OFFSETS" => Ok(151),
            "MEMINFO" => Ok(152),
            "AP_ADC" => Ok(153),
            "DIGICAM_CONFIGURE" => Ok(154),
            "DIGICAM_CONTROL" => Ok(155),
            "MOUNT_CONFIGURE" => Ok(156),
            "MOUNT_CONTROL" => Ok(157),
            "MOUNT_STATUS" => Ok(158),
            "FENCE_POINT" => Ok(160),
            "FENCE_FETCH_POINT" => Ok(161),
            "AHRS" => Ok(163),
            "SIMSTATE" => Ok(164),
            "HWSTATUS" => Ok(165),
            "RADIO" => Ok(166),
            "LIMITS_STATUS" => Ok(167),
            "WIND" => Ok(168),
            "DATA16" => Ok(169),
            "DATA32" => Ok(170),
            "DATA64" => Ok(171),
            "DATA96" => Ok(172),
            "RANGEFINDER" => Ok(173),
            "AIRSPEED_AUTOCAL" => Ok(174),
            "RALLY_POINT" => Ok(175),
            "RALLY_FETCH_POINT" => Ok(176),
            "COMPASSMOT_STATUS" => Ok(177),
            "AHRS2" => Ok(178),
            "CAMERA_STATUS" => Ok(179),
            "CAMERA_FEEDBACK" => Ok(180),
            "BATTERY2" => Ok(181),
            "AHRS3" => Ok(182),
            "AUTOPILOT_VERSION_REQUEST" => Ok(183),
            "REMOTE_LOG_DATA_BLOCK" => Ok(184),
            "REMOTE_LOG_BLOCK_STATUS" => Ok(185),
            "LED_CONTROL" => Ok(186),
            "MAG_CAL_PROGRESS" => Ok(191),
            "MAG_CAL_REPORT" => Ok(192),
            "EKF_STATUS_REPORT" => Ok(193),
            "PID_TUNING" => Ok(194),
            "DEEPSTALL" => Ok(195),
            "GIMBAL_REPORT" => Ok(200),
            "GIMBAL_CONTROL" => Ok(201),
            "GIMBAL_TORQUE_CMD_REPORT" => Ok(214),
            "GOPRO_HEARTBEAT" => Ok(215),
            "GOPRO_GET_REQUEST" => Ok(216),
            "GOPRO_GET_RESPONSE" => Ok(217),
            "GOPRO_SET_REQUEST" => Ok(218),
            "GOPRO_SET_RESPONSE" => Ok(219),
            "EFI_STATUS" => Ok(225),
            "RPM" => Ok(226),
            "DEVICE_OP_READ" => Ok(11000),
            "DEVICE_OP_READ_REPLY" => Ok(11001),
            "DEVICE_OP_WRITE" => Ok(11002),
            "DEVICE_OP_WRITE_REPLY" => Ok(11003),
            "ADAP_TUNING" => Ok(11010),
            "VISION_POSITION_DELTA" => Ok(11011),
            "AOA_SSA" => Ok(11020),
            "ESC_TELEMETRY_1_TO_4" => Ok(11030),
            "ESC_TELEMETRY_5_TO_8" => Ok(11031),
            "ESC_TELEMETRY_9_TO_12" => Ok(11032),
            _ => {
                match crate::common::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                match crate::uavionix::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                match crate::icarous::MavMessage::message_id_from_name(name) {
                    Ok(name) => return Ok(name),
                    Err(..) => {}
                }
                Err("Invalid message name.")
            }
        }
    }
    fn default_message_from_id(id: u32) -> Result<MavMessage, &'static str> {
        match id {
            150 => Ok(MavMessage::SENSOR_OFFSETS(SENSOR_OFFSETS_DATA::default())),
            151 => Ok(MavMessage::SET_MAG_OFFSETS(SET_MAG_OFFSETS_DATA::default())),
            152 => Ok(MavMessage::MEMINFO(MEMINFO_DATA::default())),
            153 => Ok(MavMessage::AP_ADC(AP_ADC_DATA::default())),
            154 => Ok(MavMessage::DIGICAM_CONFIGURE(
                DIGICAM_CONFIGURE_DATA::default(),
            )),
            155 => Ok(MavMessage::DIGICAM_CONTROL(DIGICAM_CONTROL_DATA::default())),
            156 => Ok(MavMessage::MOUNT_CONFIGURE(MOUNT_CONFIGURE_DATA::default())),
            157 => Ok(MavMessage::MOUNT_CONTROL(MOUNT_CONTROL_DATA::default())),
            158 => Ok(MavMessage::MOUNT_STATUS(MOUNT_STATUS_DATA::default())),
            160 => Ok(MavMessage::FENCE_POINT(FENCE_POINT_DATA::default())),
            161 => Ok(MavMessage::FENCE_FETCH_POINT(
                FENCE_FETCH_POINT_DATA::default(),
            )),
            163 => Ok(MavMessage::AHRS(AHRS_DATA::default())),
            164 => Ok(MavMessage::SIMSTATE(SIMSTATE_DATA::default())),
            165 => Ok(MavMessage::HWSTATUS(HWSTATUS_DATA::default())),
            166 => Ok(MavMessage::RADIO(RADIO_DATA::default())),
            167 => Ok(MavMessage::LIMITS_STATUS(LIMITS_STATUS_DATA::default())),
            168 => Ok(MavMessage::WIND(WIND_DATA::default())),
            169 => Ok(MavMessage::DATA16(DATA16_DATA::default())),
            170 => Ok(MavMessage::DATA32(DATA32_DATA::default())),
            171 => Ok(MavMessage::DATA64(DATA64_DATA::default())),
            172 => Ok(MavMessage::DATA96(DATA96_DATA::default())),
            173 => Ok(MavMessage::RANGEFINDER(RANGEFINDER_DATA::default())),
            174 => Ok(MavMessage::AIRSPEED_AUTOCAL(
                AIRSPEED_AUTOCAL_DATA::default(),
            )),
            175 => Ok(MavMessage::RALLY_POINT(RALLY_POINT_DATA::default())),
            176 => Ok(MavMessage::RALLY_FETCH_POINT(
                RALLY_FETCH_POINT_DATA::default(),
            )),
            177 => Ok(MavMessage::COMPASSMOT_STATUS(
                COMPASSMOT_STATUS_DATA::default(),
            )),
            178 => Ok(MavMessage::AHRS2(AHRS2_DATA::default())),
            179 => Ok(MavMessage::CAMERA_STATUS(CAMERA_STATUS_DATA::default())),
            180 => Ok(MavMessage::CAMERA_FEEDBACK(CAMERA_FEEDBACK_DATA::default())),
            181 => Ok(MavMessage::BATTERY2(BATTERY2_DATA::default())),
            182 => Ok(MavMessage::AHRS3(AHRS3_DATA::default())),
            183 => Ok(MavMessage::AUTOPILOT_VERSION_REQUEST(
                AUTOPILOT_VERSION_REQUEST_DATA::default(),
            )),
            184 => Ok(MavMessage::REMOTE_LOG_DATA_BLOCK(
                REMOTE_LOG_DATA_BLOCK_DATA::default(),
            )),
            185 => Ok(MavMessage::REMOTE_LOG_BLOCK_STATUS(
                REMOTE_LOG_BLOCK_STATUS_DATA::default(),
            )),
            186 => Ok(MavMessage::LED_CONTROL(LED_CONTROL_DATA::default())),
            191 => Ok(MavMessage::MAG_CAL_PROGRESS(
                MAG_CAL_PROGRESS_DATA::default(),
            )),
            192 => Ok(MavMessage::MAG_CAL_REPORT(MAG_CAL_REPORT_DATA::default())),
            193 => Ok(MavMessage::EKF_STATUS_REPORT(
                EKF_STATUS_REPORT_DATA::default(),
            )),
            194 => Ok(MavMessage::PID_TUNING(PID_TUNING_DATA::default())),
            195 => Ok(MavMessage::DEEPSTALL(DEEPSTALL_DATA::default())),
            200 => Ok(MavMessage::GIMBAL_REPORT(GIMBAL_REPORT_DATA::default())),
            201 => Ok(MavMessage::GIMBAL_CONTROL(GIMBAL_CONTROL_DATA::default())),
            214 => Ok(MavMessage::GIMBAL_TORQUE_CMD_REPORT(
                GIMBAL_TORQUE_CMD_REPORT_DATA::default(),
            )),
            215 => Ok(MavMessage::GOPRO_HEARTBEAT(GOPRO_HEARTBEAT_DATA::default())),
            216 => Ok(MavMessage::GOPRO_GET_REQUEST(
                GOPRO_GET_REQUEST_DATA::default(),
            )),
            217 => Ok(MavMessage::GOPRO_GET_RESPONSE(
                GOPRO_GET_RESPONSE_DATA::default(),
            )),
            218 => Ok(MavMessage::GOPRO_SET_REQUEST(
                GOPRO_SET_REQUEST_DATA::default(),
            )),
            219 => Ok(MavMessage::GOPRO_SET_RESPONSE(
                GOPRO_SET_RESPONSE_DATA::default(),
            )),
            225 => Ok(MavMessage::EFI_STATUS(EFI_STATUS_DATA::default())),
            226 => Ok(MavMessage::RPM(RPM_DATA::default())),
            11000 => Ok(MavMessage::DEVICE_OP_READ(DEVICE_OP_READ_DATA::default())),
            11001 => Ok(MavMessage::DEVICE_OP_READ_REPLY(
                DEVICE_OP_READ_REPLY_DATA::default(),
            )),
            11002 => Ok(MavMessage::DEVICE_OP_WRITE(DEVICE_OP_WRITE_DATA::default())),
            11003 => Ok(MavMessage::DEVICE_OP_WRITE_REPLY(
                DEVICE_OP_WRITE_REPLY_DATA::default(),
            )),
            11010 => Ok(MavMessage::ADAP_TUNING(ADAP_TUNING_DATA::default())),
            11011 => Ok(MavMessage::VISION_POSITION_DELTA(
                VISION_POSITION_DELTA_DATA::default(),
            )),
            11020 => Ok(MavMessage::AOA_SSA(AOA_SSA_DATA::default())),
            11030 => Ok(MavMessage::ESC_TELEMETRY_1_TO_4(
                ESC_TELEMETRY_1_TO_4_DATA::default(),
            )),
            11031 => Ok(MavMessage::ESC_TELEMETRY_5_TO_8(
                ESC_TELEMETRY_5_TO_8_DATA::default(),
            )),
            11032 => Ok(MavMessage::ESC_TELEMETRY_9_TO_12(
                ESC_TELEMETRY_9_TO_12_DATA::default(),
            )),
            _ => {
                if let Ok(msg) = crate::common::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::common(msg));
                }
                if let Ok(msg) = crate::uavionix::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::uavionix(msg));
                }
                if let Ok(msg) = crate::icarous::MavMessage::default_message_from_id(id) {
                    return Ok(MavMessage::icarous(msg));
                }
                return Err("Invalid message id.");
            }
        }
    }
    fn ser(&self) -> Vec<u8> {
        match self {
            &MavMessage::SENSOR_OFFSETS(ref body) => body.ser(),
            &MavMessage::SET_MAG_OFFSETS(ref body) => body.ser(),
            &MavMessage::MEMINFO(ref body) => body.ser(),
            &MavMessage::AP_ADC(ref body) => body.ser(),
            &MavMessage::DIGICAM_CONFIGURE(ref body) => body.ser(),
            &MavMessage::DIGICAM_CONTROL(ref body) => body.ser(),
            &MavMessage::MOUNT_CONFIGURE(ref body) => body.ser(),
            &MavMessage::MOUNT_CONTROL(ref body) => body.ser(),
            &MavMessage::MOUNT_STATUS(ref body) => body.ser(),
            &MavMessage::FENCE_POINT(ref body) => body.ser(),
            &MavMessage::FENCE_FETCH_POINT(ref body) => body.ser(),
            &MavMessage::AHRS(ref body) => body.ser(),
            &MavMessage::SIMSTATE(ref body) => body.ser(),
            &MavMessage::HWSTATUS(ref body) => body.ser(),
            &MavMessage::RADIO(ref body) => body.ser(),
            &MavMessage::LIMITS_STATUS(ref body) => body.ser(),
            &MavMessage::WIND(ref body) => body.ser(),
            &MavMessage::DATA16(ref body) => body.ser(),
            &MavMessage::DATA32(ref body) => body.ser(),
            &MavMessage::DATA64(ref body) => body.ser(),
            &MavMessage::DATA96(ref body) => body.ser(),
            &MavMessage::RANGEFINDER(ref body) => body.ser(),
            &MavMessage::AIRSPEED_AUTOCAL(ref body) => body.ser(),
            &MavMessage::RALLY_POINT(ref body) => body.ser(),
            &MavMessage::RALLY_FETCH_POINT(ref body) => body.ser(),
            &MavMessage::COMPASSMOT_STATUS(ref body) => body.ser(),
            &MavMessage::AHRS2(ref body) => body.ser(),
            &MavMessage::CAMERA_STATUS(ref body) => body.ser(),
            &MavMessage::CAMERA_FEEDBACK(ref body) => body.ser(),
            &MavMessage::BATTERY2(ref body) => body.ser(),
            &MavMessage::AHRS3(ref body) => body.ser(),
            &MavMessage::AUTOPILOT_VERSION_REQUEST(ref body) => body.ser(),
            &MavMessage::REMOTE_LOG_DATA_BLOCK(ref body) => body.ser(),
            &MavMessage::REMOTE_LOG_BLOCK_STATUS(ref body) => body.ser(),
            &MavMessage::LED_CONTROL(ref body) => body.ser(),
            &MavMessage::MAG_CAL_PROGRESS(ref body) => body.ser(),
            &MavMessage::MAG_CAL_REPORT(ref body) => body.ser(),
            &MavMessage::EKF_STATUS_REPORT(ref body) => body.ser(),
            &MavMessage::PID_TUNING(ref body) => body.ser(),
            &MavMessage::DEEPSTALL(ref body) => body.ser(),
            &MavMessage::GIMBAL_REPORT(ref body) => body.ser(),
            &MavMessage::GIMBAL_CONTROL(ref body) => body.ser(),
            &MavMessage::GIMBAL_TORQUE_CMD_REPORT(ref body) => body.ser(),
            &MavMessage::GOPRO_HEARTBEAT(ref body) => body.ser(),
            &MavMessage::GOPRO_GET_REQUEST(ref body) => body.ser(),
            &MavMessage::GOPRO_GET_RESPONSE(ref body) => body.ser(),
            &MavMessage::GOPRO_SET_REQUEST(ref body) => body.ser(),
            &MavMessage::GOPRO_SET_RESPONSE(ref body) => body.ser(),
            &MavMessage::EFI_STATUS(ref body) => body.ser(),
            &MavMessage::RPM(ref body) => body.ser(),
            &MavMessage::DEVICE_OP_READ(ref body) => body.ser(),
            &MavMessage::DEVICE_OP_READ_REPLY(ref body) => body.ser(),
            &MavMessage::DEVICE_OP_WRITE(ref body) => body.ser(),
            &MavMessage::DEVICE_OP_WRITE_REPLY(ref body) => body.ser(),
            &MavMessage::ADAP_TUNING(ref body) => body.ser(),
            &MavMessage::VISION_POSITION_DELTA(ref body) => body.ser(),
            &MavMessage::AOA_SSA(ref body) => body.ser(),
            &MavMessage::ESC_TELEMETRY_1_TO_4(ref body) => body.ser(),
            &MavMessage::ESC_TELEMETRY_5_TO_8(ref body) => body.ser(),
            &MavMessage::ESC_TELEMETRY_9_TO_12(ref body) => body.ser(),
            &MavMessage::common(ref msg) => msg.ser(),
            &MavMessage::uavionix(ref msg) => msg.ser(),
            &MavMessage::icarous(ref msg) => msg.ser(),
        }
    }
    fn extra_crc(id: u32) -> u8 {
        match id {
            150 => 134,
            151 => 219,
            152 => 208,
            153 => 188,
            154 => 84,
            155 => 22,
            156 => 19,
            157 => 21,
            158 => 134,
            160 => 78,
            161 => 68,
            163 => 127,
            164 => 154,
            165 => 21,
            166 => 21,
            167 => 144,
            168 => 1,
            169 => 234,
            170 => 73,
            171 => 181,
            172 => 22,
            173 => 83,
            174 => 167,
            175 => 138,
            176 => 234,
            177 => 240,
            178 => 47,
            179 => 189,
            180 => 52,
            181 => 174,
            182 => 229,
            183 => 85,
            184 => 159,
            185 => 186,
            186 => 72,
            191 => 92,
            192 => 36,
            193 => 71,
            194 => 98,
            195 => 120,
            200 => 134,
            201 => 205,
            214 => 69,
            215 => 101,
            216 => 50,
            217 => 202,
            218 => 17,
            219 => 162,
            225 => 208,
            226 => 207,
            11000 => 134,
            11001 => 15,
            11002 => 234,
            11003 => 64,
            11010 => 46,
            11011 => 106,
            11020 => 205,
            11030 => 144,
            11031 => 133,
            11032 => 85,
            _ => {
                match crate::common::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                match crate::uavionix::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                match crate::icarous::MavMessage::extra_crc(id) {
                    0 => {}
                    any => return any,
                }
                0
            }
        }
    }
}
