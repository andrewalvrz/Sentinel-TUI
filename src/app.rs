pub enum Tab {
    Overview,
    IMU,
    GPS,
}

pub struct AppState {
    pub current_tab: Tab,
    pub data_log: Vec<(f64, f64)>,
    pub simulated: bool,
}

impl AppState {
    pub fn new(simulated: bool) -> Self {
        Self {
            current_tab: Tab::Overview,
            data_log: Vec::new(),
            simulated,
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
