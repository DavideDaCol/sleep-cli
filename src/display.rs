use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, Marker},
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    DefaultTerminal, Frame,
};
use color_eyre::Result;

pub fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75)
        ]).split(frame.area());

    frame.render_widget(
        Paragraph::new("log")
        .block(Block::new().borders(Borders::ALL)),
    layout[0]);
    frame.render_widget(
        Paragraph::new("graph")
        .block(Block::new().borders(Borders::ALL)),
    layout[1]);
}