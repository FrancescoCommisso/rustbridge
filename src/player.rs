pub mod player {
    use crate::hand::hand;

    pub struct Player {
        pub name: String,
        pub hand: hand::Hand,
    }

    pub fn new_player(name: String) -> Player {
        Player {
            name: name,
            hand: hand::new_hand(),
        }
    }

}
