pub mod hand {
    use crate::deck::deck::card::Card;
    pub struct Hand {
        pub all_cards: Vec<Card>,
    }

    pub fn new_hand() -> Hand {
        Hand {
            all_cards: Vec::new(),
        }
    }
}
