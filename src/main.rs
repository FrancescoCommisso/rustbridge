pub struct Game {
}
impl Game {
    fn with_permutation(permutation: Vec<usize>) -> Self {
        Game{}
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
fn main() {
    println!("{}",Game::with_permutation(vec![1,2,3,4,]));
}

mod testing;
