#[derive(Debug, Clone)]
pub enum Tab {
    Overview,
    IMU,
    GPS,
}

#[derive(Debug, Clone)]
pub struct GpsData {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

#[derive(Debug, Clone)]
pub struct ImuData {
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
}

#[derive(Debug, Clone)]
pub struct TelemetryFrame {
    pub altitude: f64,
    pub gps_lat: f64,
    pub gps_lon: f64,
    pub gps_alt: f64,
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
}

pub struct AppState {
    pub current_tab: Tab,
    pub data_log: Vec<(f64, f64)>,
    pub simulated: bool,
    pub gps: GpsData,
    pub imu: ImuData,
}

impl AppState {
    pub fn new(simulated: bool) -> Self {
        Self {
            current_tab: Tab::Overview,
            data_log: Vec::new(),
            simulated,
            gps: GpsData {
                lat: 26.307,
                lon: -98.1726,
                alt: 100.0,
            },
            imu: ImuData {
                pitch: 0.0,
                roll: 0.0,
                yaw: 0.0,
            },
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Overview => Tab::IMU,
            Tab::IMU => Tab::GPS,
            Tab::GPS => Tab::Overview,
        };
    }
}
