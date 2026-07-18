pub mod game;

use game::Game;

fn main() {
    let num_players = 3;
    let game = Game::new(num_players);

    println!("{}", &game);
}
