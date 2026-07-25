use std::io;

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Offset, Size},
    style::Style,
    symbols,
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
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
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .spacing(1)
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

        let tokens_layout = Layout::vertical([
            Constraint::Length(6),
            Constraint::Length(4),
            Constraint::Length(5),
        ])
        .flex(Flex::Center)
        .spacing(1)
        .split(cols[1]);

        let mut hints: Vec<Line> = vec![];
        for i in 0..4 {
            let first_char = if i < self.game.hints { "O" } else { "X" };
            let second_char = if i + 4 < self.game.hints { "O" } else { "X" };
            hints.push(format!("{} {}", first_char, second_char).into());
        }
        Paragraph::new(vec![
            Line::from("O O"),
            Line::from("O O"),
            Line::from("O O"),
            Line::from("O O"),
        ])
        .centered()
        .style(Style::new().blue())
        .block(Block::bordered().border_type(BorderType::Rounded))
        .render(tokens_layout[0], buf);

        let card_rect = tokens_layout[1];
        let card_rect = card_rect.resize(Size::new(card_rect.width - 1, card_rect.height - 1));
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(card_rect, buf);
        Paragraph::new(self.game.draw.len().to_string())
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(card_rect + Offset::new(1, 1), buf);

        let mut errors: Vec<Line> = vec![];
        for i in 0..3 {
            if i < self.game.errors {
                errors.push("O".into());
            } else {
                errors.push("X".into());
            }
        }
        Paragraph::new(errors)
            .centered()
            .style(Style::new().red())
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(tokens_layout[2], buf);

        Paragraph::new("col3").render(cols[2], buf);
    }
}
