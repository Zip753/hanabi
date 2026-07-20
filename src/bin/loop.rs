use hanabi::game::{self, Game, InformationKind};

fn main() {
    let num_players = 3;
    let mut game = Game::new(num_players);

    println!("{}", &game);

    let mut hinted_color = 1;
    for iter in 0..6 {
        let card_idx = iter % 5;

        println!();
        match iter % 3 {
            0 => {
                println!(">>> Discarding card with index {}", card_idx);
                game.discard(card_idx);
            }
            1 => {
                println!(">>> Playing card with index {}", card_idx);
                game.play(card_idx);
            }
            2 => {
                let hinted_player_idx = 0;
                let information = InformationKind::Color(hinted_color);
                println!(
                    ">>> Hinting player 1 about {} color",
                    game::COLOR_CODE[hinted_color]
                );
                game.hint(hinted_player_idx, information);

                hinted_color += 1;
            }
            _ => unreachable!(),
        }

        println!();
        println!("{}", &game);
    }
}
