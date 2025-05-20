use std::{sync::mpsc::Sender, thread, time::Duration};
use rand::Rng;

pub fn start_simulated_telemetry(tx: Sender<f64>) {
    thread::spawn(move || {
        let mut alt = 100.0;
        loop {
            alt += rand::thread_rng().gen_range(-0.5..0.5);
            tx.send(alt).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
}
