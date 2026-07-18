mod game;

use game::Game;

fn main() {
    let num_players = 3;
    let game = Game::new(num_players);

    // current player makes move
    // - discard(card in hand)
    //   - add discarded card to the discard pile
    //   - check if it was at the limit
    //      - if yes, game fails instantly
    //   - draw one card to replace the discarded one
    //      - update the value and set information to empty
    // - hint(player, information{color X or value U})
    //   ! no need for the card list, since info is enough to derive to which cards it applies
    //   - check if hints > 0
    //      - if no, disallow move
    //   - hints -= 1
    //   - calculate the list of cards for which the information is true for hinted player
    //   - add respective information to each card in their hand
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

    println!("{}", &game);
}
