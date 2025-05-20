use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, Chart, Dataset, Axis},
    symbols,
    Frame,
    backend::Backend,
};
use crate::app::{AppState, Tab};



pub fn draw_ui(f: &mut Frame, app: &AppState)
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let title = match app.current_tab {
        Tab::Overview => "Overview",
        Tab::IMU => "IMU",
        Tab::GPS => "GPS",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    match app.current_tab {
        Tab::Overview => draw_chart(f, chunks[1], app),
        _ => {} // Placeholder for now
    }
}

 fn draw_chart(f: &mut Frame, area: Rect, app: &AppState)
{
    let data = app.data_log.iter().cloned().collect::<Vec<_>>();

    let dataset = Dataset::default()
        .name("Altitude")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&data);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("Altitude vs Time").borders(Borders::ALL))
        .x_axis(Axis::default().title("Time").bounds([0.0, 60.0]))
        .y_axis(Axis::default().title("Altitude").bounds([0.0, 200.0]));

    f.render_widget(chart, area);
}
