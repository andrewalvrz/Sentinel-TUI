mod app;
mod telemetry;
mod ui;

use app::AppState;
use telemetry::{start_simulated_telemetry};
use ui::draw_ui;
use app::TelemetryFrame;

use std::{
    io,
    time::{Duration, Instant},
    sync::mpsc,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
    event::{poll, read, Event, KeyCode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel::<TelemetryFrame>();
    start_simulated_telemetry(tx);

    let mut app = AppState::new(true);
    let start = Instant::now();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => app.next_tab(),
                    _ => {}
                }
            }
        }

        if let Ok(frame) = rx.try_recv() {
            let t = start.elapsed().as_secs_f64();
            app.data_log.push((t, frame.altitude));
            if app.data_log.len() > 100 {
                app.data_log.remove(0);
            }

            app.gps.lat = frame.gps_lat;
            app.gps.lon = frame.gps_lon;
            app.gps.alt = frame.gps_alt;

            app.imu.pitch = frame.pitch;
            app.imu.roll = frame.roll;
            app.imu.yaw = frame.yaw;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

