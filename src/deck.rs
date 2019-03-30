pub mod deck {

    use rand::seq::SliceRandom;
    use rand::thread_rng;

    pub struct Deck {
        pub cards: Vec<card::Card>,
    }

    impl Deck {
        pub fn shuffle(&mut self) {
            let mut rng = thread_rng();
            &self.cards.shuffle(&mut rng);
        }
        // pub fn len(&self)-> usize{
        //     &self.cards.len()
        // }
        pub fn print_deck(&self) {
            for c in &self.cards {
                println!("{}", c);
            }
        }
        pub fn get_card(&self, index: usize) -> card::Card {
            card::make_card(self.cards[index].index)
        }
    }

    pub fn new_full_deck() -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        for x in 0..52 {
            deck.cards.push(card::make_card(x));
        }
        deck
    }

    pub fn new_to_index(max: u32) -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        for x in 0..max {
            deck.cards.push(card::make_card(x));
        }
        deck
    }

    pub fn new_from_vec(v: Vec<u32>) -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        for x in 0..v.len() {
            deck.cards.push(card::make_card(v[x]));
        }
        deck
    }

    pub mod card {

        enum Suit {
            Heart,
            Diamond,
            Spade,
            Club,
        }

        impl std::fmt::Display for Suit {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    Suit::Heart => write!(f, "Heart"),
                    Suit::Diamond => write!(f, "Diamond"),
                    Suit::Spade => write!(f, "Spade"),
                    Suit::Club => write!(f, "Club"),
                }
            }
        }

        pub struct Card {
            pub index: u32,
            suit: Suit,
            pub value: u32,
        }

        impl std::fmt::Display for Card {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "Index: {:?}    Value: {}   Suit: {}",
                    self.index, self.value, self.suit
                )
            }
        }

        pub fn make_card(i: u32) -> Card {
            let card = Card {
                index: i,
                value: { (i % 13) + 2 },
                suit: determine_suit(i),
            };
            card
        }

        fn determine_suit(index: u32) -> Suit {
            if index <= 12 {
                return Suit::Club;
            }
            if index >= 13 && index <= 26 {
                return Suit::Diamond;
            }
            if index >= 27 && index <= 39 {
                return Suit::Heart;
            }
            if index >= 40 && index <= 51 {
                return Suit::Spade;
            } else {
                panic!("Card index: {} out of range 0-51", index);
            }
        }

    }

}
