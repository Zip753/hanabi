use std::{borrow::Cow, fmt::Display};

use rand::seq::SliceRandom;

const COLORS: usize = 5; // colors: red, green, yellow, blue, white
const VALUES: usize = 5; // values: 1, 2, 3, 4, 5
const HAND: usize = 5; // how many cards are on hand at any time

// How much of each value of a given color there is in the whole deck
const NUM_VALUES: [usize; VALUES] = [
    3, 2, 2, 2, 1
];

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
            write!(f, "Player {}: {}\n", idx + 1, player)?
        }
        Ok(())
    }
}

#[derive(Default, Copy, Clone)]
pub struct Card {
    color: usize,
    value: usize,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_name: Cow<'_, str> = match self.color {
            0 => "red".into(),
            1 => "green".into(),
            2 => "yellow".into(),
            3 => "blue".into(),
            4 => "white".into(),
            n => format!("<invalid ({})>", n).into(),
        };
        write!(f, "{} {}", color_name, self.value + 1)
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
            for value in 0..VALUES {
                for _ in 0..NUM_VALUES[value] {
                    all_cards.push(Card {
                        color,
                        value,
                    })
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

    pub fn show_state(&self) -> () {
        println!("Players: {}", self.players.len());
        for (idx, player) in self.players.iter().enumerate() {
               // println!("Player {}: [{}] (info coming later)", idx + 1, player.hand.map(|&h| h.card).join());
        }
    }
}
