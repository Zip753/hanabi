use rand::seq::SliceRandom;

mod display;

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

#[derive(Default, Copy, Clone)]
pub struct Card {
    color: usize,
    value: CardValue,
}

#[derive(Copy, Clone, Default)]
struct CardValue(u8);

impl CardValue {
    pub fn as_idx(&self) -> usize {
        (self.0 - 1) as usize
    }
}

pub struct Player {
    hand: [CardWithInformation; HAND],
}

#[derive(Default)]
pub struct CardWithInformation {
    card: Card,
    information: Vec<Information>,
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
