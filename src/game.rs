use std::fmt::Display;

use rand::seq::SliceRandom;

const COLORS: usize = 5; // colors: red, green, yellow, blue, white
const MAX_VALUE: u8 = 5; // values: 1, 2, 3, 4, 5
const HAND: usize = 5; // how many cards are on hand at any time

const COLOR_CODE: [&'static str; COLORS] = ["R", "G", "Y", "B", "W"];

// How much of each value of a given color there is in the whole deck
const NUM_VALUES: [u8; MAX_VALUE as usize] = [3, 2, 2, 2, 1];

pub struct Game {
    table: [usize; COLORS],
    discard: Vec<Card>,
    draw: Vec<Card>,
    players: Vec<Player>,
    hints: usize,
    errors: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, player) in self.players.iter().enumerate() {
            writeln!(f, "Player {}: {}", idx + 1, player)?;
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
        writeln!(f, "Remaining errors: {}", self.errors)?;
        Ok(())
    }
}

#[derive(Default, Copy, Clone)]
pub struct Card {
    color: usize,
    value: CardValue,
}

#[derive(Copy, Clone, Default)]
struct CardValue(u8);

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CardValue {
    pub fn as_idx(&self) -> usize {
        (self.0 - 1) as usize
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

pub struct Player {
    hand: [CardWithInformation; HAND],
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards: Vec<String> = self.hand.iter().map(|h| h.card.to_string()).collect();
        write!(f, "{}", cards.join(", "))
    }
}

#[derive(Default)]
pub struct CardWithInformation {
    card: Card,
    information: Vec<Information>,
}

impl Display for CardWithInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: information
        write!(f, "{}", self.card)
    }
}

pub struct Information {
    kind: InformationKind,
    matches: bool,
}

pub enum InformationKind {
    Value,
    Color,
}

impl Game {
    pub fn new(num_players: usize) -> Self {
        let mut all_cards: Vec<Card> = vec![];
        for color in 0..COLORS {
            for value in 1..=MAX_VALUE {
                let value = CardValue(value);
                for _ in 0..NUM_VALUES[value.as_idx()] {
                    all_cards.push(Card { color, value })
                }
            }
        }
        let mut rng = rand::rng();
        all_cards.shuffle(&mut rng);

        let mut players: Vec<Player> = vec![];
        for player_idx in 0..num_players {
            let mut player = Player {
                hand: std::array::from_fn(|_| CardWithInformation::default()),
            };
            for pos in 0..HAND {
                player.hand[pos].card = all_cards[player_idx * HAND + pos];
            }
            players.push(player);
        }

        Game {
            hints: 8,
            errors: 3,
            discard: vec![],
            table: [0; COLORS],
            players,
            draw: all_cards.split_off(num_players * HAND),
        }
    }
}
