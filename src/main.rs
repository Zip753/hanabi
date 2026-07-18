mod game;

use game::Game;

fn main() {
    let num_players = 3;
    let mut game = Game::new(num_players);

    println!("{}", &game);

    for iter in 0..7 {
        let card_idx = iter % 5;

        println!();
        println!(">>> Discarding card with index {}", card_idx);
        game.discard(card_idx);

        println!();
        println!("{}", &game);
    }
}
