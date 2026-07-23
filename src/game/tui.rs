use std::io;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::Style,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget},
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
        let cols = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .split(area);

        let players_layout = Layout::vertical([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Length(5),
        ])
        .flex(Flex::Center)
        .spacing(1)
        .split(cols[0]);

        for (idx, (player, &row)) in self
            .game
            .players
            .iter()
            .zip(players_layout.iter())
            .enumerate()
        {
            let is_current_player = idx == self.game.current_player;
            Paragraph::new(player.to_string())
                .centered()
                .block(
                    Block::bordered()
                        .border_set(if is_current_player {
                            symbols::border::DOUBLE
                        } else {
                            symbols::border::PLAIN
                        })
                        .padding(Padding::symmetric(1, 1))
                        .title_top(format!("player {}", idx + 1))
                        .title_style(if is_current_player {
                            Style::new().green().bold()
                        } else {
                            Style::new()
                        }),
                )
                .render(row, buf);
        }

        Paragraph::new("col2").render(cols[1], buf);
        Paragraph::new("col3").render(cols[2], buf);
    }
}
