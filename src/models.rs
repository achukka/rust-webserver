use serde::Deserialize;


pub struct BlackjackGame {
    // Add your game state fields here
    cards: i32
}

impl BlackjackGame {
    pub fn new(cards: i32) -> Self {
        // Initialize game state here
        BlackjackGame {
            cards
        }
    }

    pub fn get_cards(&self) -> i32 {
        self.cards
    }
}


#[derive(Debug, Deserialize)]
pub struct Info {
    pub(crate) name: String
}