use std::io;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Paragraph, Widget},
};

use crate::game::Game;

pub struct App {
    exit: bool,
    game: Game,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: Default::default(),
            game: Game::new(3),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.is_press() => {
                self.exit = true;
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("hanabi will be here").render(area, buf);
    }
}
