use ratatui::{
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Color},
    widgets::{Block, Borders, Chart, Dataset, Axis, Paragraph},
    symbols,
    Frame,
};

use ratatui::text::{Span, Line};

use crate::app::{AppState, Tab};


pub fn draw_ui(f: &mut Frame, app: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let title = match app.current_tab {
        Tab::Overview => "Overview",
        Tab::IMU => "IMU Data",
        Tab::GPS => "GPS Coordinates",
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    match app.current_tab {
        Tab::Overview => draw_chart(f, chunks[1], app),
        Tab::GPS => draw_gps(f, chunks[1], app),
        Tab::IMU => draw_imu(f, chunks[1], app),
    }
}

fn draw_chart(f: &mut Frame, area: Rect, app: &AppState) {
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

fn draw_gps(f: &mut Frame, area: Rect, app: &AppState) {
    let gps = &app.gps;

    let text = vec![
    Line::from(vec![Span::raw(format!("Latitude:  {:.5}", gps.lat))]),
    Line::from(vec![Span::raw(format!("Longitude: {:.5}", gps.lon))]),
    Line::from(vec![Span::raw(format!("Altitude:  {:.1} m", gps.alt))]),
];

    

    let para = Paragraph::new(text)
        .block(Block::default().title("GPS Data").borders(Borders::ALL));

    f.render_widget(para, area);
}

fn draw_imu(f: &mut Frame, area: Rect, app: &AppState) {
    let imu = &app.imu;

    let text = vec![
    Line::from(vec![Span::raw(format!("Pitch: {:.2}°", imu.pitch))]),
    Line::from(vec![Span::raw(format!("Roll:  {:.2}°", imu.roll))]),
    Line::from(vec![Span::raw(format!("Yaw:   {:.2}°", imu.yaw))]),
];


    let para = Paragraph::new(text)
        .block(Block::default().title("IMU Data").borders(Borders::ALL));

    f.render_widget(para, area);
}
