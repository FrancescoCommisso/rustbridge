pub struct Game {}

impl Game {
    fn with_permutation(permutation: Vec<usize>) -> Self {
        Game {}
    }
}
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Game!\n")
    }
}
pub fn with_permutation(permutation: Vec<usize>) -> Game {
    Game::with_permutation(permutation)
}
pub fn format_game(game: Game) -> String {
    game.to_string()
}

mod board;
mod deck;
mod hand;
mod player;

fn main() {
    // let mut deck = deck::deck::new_full_deck();
    // deck.shuffle();
    // deck.print_deck();

    let b = board::board::new_board();
    b.deck.print_deck()

    // cards::deck::new_shuffled();
    // println!("{}", Game::with_permutation(vec![1, 2, 3, 4,]));
}

mod testing;
