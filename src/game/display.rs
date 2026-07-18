use std::fmt::Display;

use super::*;

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, player) in self.players.iter().enumerate() {
            writeln!(
                f,
                "{}Player {}: {}",
                if idx == self.current_player {
                    "-> "
                } else {
                    ""
                },
                idx + 1,
                player
            )?;
        }
        writeln!(f)?;

        write!(f, "Table: ")?;
        let mut iter = self.table.iter().enumerate().peekable();
        while let Some((color, &color_value)) = iter.next() {
            write!(f, "{}{}", COLOR_CODE[color], color_value)?;
            if iter.peek().is_some() {
                write!(f, " ")?;
            }
        }
        writeln!(f)?;
        writeln!(f)?;

        let mut discard_amount = [[0; MAX_VALUE as usize]; COLORS];
        for card in &self.discard {
            discard_amount[card.color][card.value.as_idx()] += 1;
        }

        writeln!(f, "Discard pile")?;
        for color in 0..COLORS {
            for value in 1..=MAX_VALUE {
                let value = CardValue(value);
                let card = Card { color, value };
                write!(
                    f,
                    "{}: {}/{}",
                    card,
                    discard_amount[color][value.as_idx()],
                    NUM_VALUES[value.as_idx()]
                )?;
                if value.0 != MAX_VALUE {
                    write!(f, ", ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        writeln!(f, "Remaining hints: {}", self.hints)?;
        write!(f, "Remaining errors: {}", self.errors)?;
        Ok(())
    }
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let color_name: Cow<'_, str> = match self.color {
        //     0 => "red".into(),
        //     1 => "green".into(),
        //     2 => "yellow".into(),
        //     3 => "blue".into(),
        //     4 => "white".into(),
        //     n => format!("<invalid ({})>", n).into(),
        // };
        let color_name = COLOR_CODE.get(self.color).unwrap_or(&"?");
        write!(f, "{}{}", color_name, self.value)
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.hand.iter().map(|h| h.card.to_string()).collect();
        write!(f, "{}", cards.join(", "))
    }
}

impl Display for CardWithInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: information
        write!(f, "{}", self.card)
    }
}
