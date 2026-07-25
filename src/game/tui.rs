use std::{borrow::Cow, io};

use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout, Offset, Size},
    style::Style,
    symbols::{self, border},
    text::Line,
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::game::{COLORS, Game, HAND};

const ROUNDED_DASHED: border::Set<'static> = border::Set {
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_left: "╎",
    vertical_right: "╎",
    horizontal_top: "╌",
    horizontal_bottom: "╌",
};

fn border_color(color: usize) -> Style {
    match color {
        0 => Style::new().red(),
        1 => Style::new().green(),
        2 => Style::new().yellow(),
        3 => Style::new().blue(),
        4 => Style::new().white(),
        _ => unreachable!(),
    }
}

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

        let players_layout = Layout::vertical(vec![Constraint::Length(7); self.game.players.len()])
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
            Block::bordered()
                .border_set(if is_current_player {
                    symbols::border::DOUBLE
                } else {
                    symbols::border::PLAIN
                })
                .padding(Padding::uniform(1))
                .title_top(format!("player {}", idx + 1))
                .title_style(if is_current_player {
                    Style::new().green().bold()
                } else {
                    Style::new()
                })
                .render(row, buf);

            let inner_row =
                row.resize(Size::new(row.width - 2, row.height - 2)) + Offset::new(1, 1);

            let player_layout = Layout::horizontal(vec![Constraint::Length(5); HAND])
                .flex(Flex::Center)
                .spacing(3)
                .split(inner_row);

            for (card_with_info, &area) in player.hand.iter().zip(player_layout.iter()) {
                let card = card_with_info.card;
                Paragraph::new(card.value.to_string())
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .padding(Padding::uniform(1))
                            .style(border_color(card.color)),
                    )
                    .render(area, buf);
            }
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

        let table_discard_layout =
            Layout::vertical([Constraint::Length(11), Constraint::Length(9)])
                .flex(Flex::Center)
                .spacing(1)
                .split(cols[2]);

        let table_layout = table_discard_layout[0];
        Block::bordered()
            .border_type(BorderType::QuadrantOutside)
            .padding(Padding::symmetric(1, 1))
            .render(table_layout, buf);
        let inner_table_layout = table_layout
            .resize(Size::new(table_layout.width - 2, table_layout.height - 2))
            + Offset::new(1, 1);
        let table_card_layout = Layout::horizontal(vec![Constraint::Length(5); COLORS])
            .flex(Flex::Center)
            .spacing(3)
            .split(inner_table_layout);
        for (color, rect) in table_card_layout.iter().enumerate() {
            // let value = self.game.table[color];
            let value = color;

            if value == 0 {
                Block::bordered()
                    .border_set(ROUNDED_DASHED)
                    .style(border_color(color))
                    .padding(Padding::uniform(1))
                    .render(rect.resize(Size::new(5, 5)), buf);
                continue;
            }

            for i in 0..value {
                let text: Cow<'_, str> = if i == value - 1 {
                    value.to_string().into()
                } else {
                    "".into()
                };
                let rendered_height = if i == value - 1 { 5 } else { 1 };
                Paragraph::new(text)
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .padding(Padding::uniform(1))
                            .style(border_color(color)),
                    )
                    .render(
                        rect.resize(Size::new(5, rendered_height)) + Offset::new(0, i as i32),
                        buf,
                    );
            }
        }
        Paragraph::new(self.game.get_discard_matrix().to_string())
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::HeavyTripleDashed)
                    .padding(Padding::symmetric(1, 1)),
            )
            .render(table_discard_layout[1], buf);
    }
}
