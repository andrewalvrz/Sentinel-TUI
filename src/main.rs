mod app;
mod telemetry;
mod ui;

use crate::app::AppState;
use crate::telemetry::start_simulated_telemetry;
use crate::ui::draw_ui;

use std::{io, time::{Duration, Instant}, sync::mpsc};
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{poll, read, Event, KeyCode}};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel();
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

        if let Ok(alt) = rx.try_recv() {
            let t = start.elapsed().as_secs_f64();
            app.data_log.push((t, alt));
            if app.data_log.len() > 100 {
                app.data_log.remove(0);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
