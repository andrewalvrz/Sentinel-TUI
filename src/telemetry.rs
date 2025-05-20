use rand::Rng;
use std::{sync::mpsc::Sender, thread, time::Duration};
use crate::app::TelemetryFrame;

pub fn start_simulated_telemetry(tx: Sender<TelemetryFrame>) {
    thread::spawn(move || {
        let mut alt = 100.0;
        let mut lat = 26.3070;
        let mut lon = -98.1726;
        let mut pitch = 0.0;
        let mut roll = 0.0;
        let mut yaw = 0.0;

        loop {
            let mut rng = rand::thread_rng();

            alt += rng.gen_range(-0.5..0.5);
            lat += rng.gen_range(-0.0002..0.0002);
            lon += rng.gen_range(-0.0002..0.0002);
            pitch += rng.gen_range(-1.0..1.0);
            roll += rng.gen_range(-1.0..1.0);
            yaw += rng.gen_range(-1.0..1.0);

            let frame = TelemetryFrame {
                altitude: alt,
                gps_lat: lat,
                gps_lon: lon,
                gps_alt: alt,
                pitch,
                roll,
                yaw,
            };

            tx.send(frame).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
}
