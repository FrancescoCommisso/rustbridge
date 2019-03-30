pub mod board {

    use crate::deck::deck;
    use crate::player::player;

    pub struct Board {
        pub deck: deck::Deck,
        pub players: Vec<player::Player>,
    }

    impl Board {
        pub fn deal_cards(&mut self) {
            for c in 0..self.deck.cards.len() {
                let current_player = c % 4;
                self.players[current_player]
                    .hand
                    .all_cards
                    .push(self.deck.get_card(c));
            }
        }
    }

    pub fn new_board() -> Board {
        let mut board = Board {
            deck: deck::new_full_deck(),
            players: create_players(),
        };
        board
    }

    fn create_players() -> Vec<player::Player> {
        let mut player = Vec::new();
        let mut p1 = player::new_player(String::from("North"));
        let mut p2 = player::new_player(String::from("East"));
        let mut p3 = player::new_player(String::from("Sout"));
        let mut p4 = player::new_player(String::from("West"));
        player.push(p1);
        player
    }

}

fn main() {}
