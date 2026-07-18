use super::*;

// current player makes move
// player moves to the next one
//
// so one move can result in:
// - game ending
// - invalid move - this should be ideally disallowed by the UI (check if each move is valid for
// later)
// - success
// - in proper rules, if the deck becomes empty, we should trigger the last round
//   - it's either this or implementing less than full hand of cards (tracking which card is
//   actually there)
//
// this means that drawing should also be an Option<Card>
//
// we also rn only have one user interacting with the game on behalf of all players
// - you don't need to think about syncing players
//   - notifying about a turn
//   - checking if they can make a move/it's their turn
// - game knows which player is making a move - it's current one by default
//   - you only need "move" API, and you can fetch the current player from state for display

impl Game {
    // - discard(card in hand)
    //   - add discarded card to the discard pile
    //   - check if it was at the limit
    //      - if yes, game fails instantly
    //   - draw one card to replace the discarded one
    //      - update the value and set information to empty
    pub fn discard(&mut self, card_idx: usize) {
        let current_player = &mut self.players[self.current_player];
        let discarded_card = current_player.hand[card_idx].card;
        self.discard.push(discarded_card);

        let discarded_count = self
            .discard
            .iter()
            .filter(|&&c| c == discarded_card)
            .count();
        if discarded_count as u8 == NUM_VALUES[discarded_card.value.as_idx()] {
            todo!("game failed!")
        }

        let next_card = self.draw.pop();
        match next_card {
            Some(card) => {
                current_player.hand[card_idx] = CardWithInformation {
                    card,
                    information: vec![],
                }
            }
            None => {
                todo!("figure out how to handle empty draw pile later")
            }
        }

        self.current_player = (self.current_player + 1) % self.players.len();
    }

    // - play(card in hand)
    //   - check table state of card's color
    //   - if state + 1 is card's value
    //      - table state += 1
    //   - else
    //      - add card to discard pile
    //      - errors -= 1
    //      - if errors == 0
    //          - game ends instantly
    //   - draw one card to replace the discarded one
    pub fn play(&mut self, card_idx: usize) {
        let current_player = &mut self.players[self.current_player];
        let played_card = current_player.hand[card_idx].card;

        let table_value = self.table[played_card.color];
        if table_value + 1 == played_card.value.0 {
            self.table[played_card.color] += 1;
        } else {
            self.discard.push(played_card);
            self.errors -= 1;
            if self.errors == 0 {
                todo!("game failed!")
            }
        }

        let next_card = self.draw.pop();
        match next_card {
            Some(card) => {
                current_player.hand[card_idx] = CardWithInformation {
                    card,
                    information: vec![],
                }
            }
            None => {
                todo!("figure out how to handle empty draw pile later")
            }
        }

        self.current_player = (self.current_player + 1) % self.players.len();
    }

    // - hint(player, information{color X or value U})
    //   ! no need for the card list, since info is enough to derive to which cards it applies
    //   - check if hints > 0
    //      - if no, disallow move
    //   - hints -= 1
    //   - calculate the list of cards for which the information is true for hinted player
    //   - add respective information to each card in their hand
    pub fn hint(&mut self, player_idx: usize, information_kind: InformationKind) {
        if self.hints == 0 {
            todo!("invalid move - out of hints!")
        }
        self.hints -= 1;

        let hinted_player = &mut self.players[player_idx];
        for card_with_info in &mut hinted_player.hand {
            card_with_info.information.push(Information {
                kind: information_kind,
            });
        }

        self.current_player = (self.current_player + 1) % self.players.len();
    }
}
