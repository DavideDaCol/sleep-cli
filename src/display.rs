use ratatui::crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
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
    frame.render_widget("hello world", frame.area());
}