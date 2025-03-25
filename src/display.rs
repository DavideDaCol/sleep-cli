use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    DefaultTerminal, Frame,
};
use color_eyre::Result;

use crate::{log::LogController, mathutils::Averages};

pub struct TerminalDisplay {
    pub app_log: LogController,
    pub app_calc: Averages,
    pub valid_state: bool,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
}

impl TerminalDisplay{
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.draw(frame))?;
            
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => {
                            self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                            self.vertical_scroll_state =
                                self.vertical_scroll_state.position(self.vertical_scroll);
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                            self.vertical_scroll_state =
                                self.vertical_scroll_state.position(self.vertical_scroll);
                        }
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(75)
            ]).split(frame.area());
                
        let paragraph = Paragraph::new(self.app_log.hours_slept.clone())
            .gray()
            .block(Block::new().borders(Borders::ALL))
            .scroll((self.vertical_scroll as u16, 0));
        frame.render_widget(paragraph, layout[0]);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            layout[0],
            &mut self.vertical_scroll_state,
        );
        frame.render_widget(
            Paragraph::new("graph")
            .block(Block::new().borders(Borders::ALL)),
        layout[1]);
    }
}