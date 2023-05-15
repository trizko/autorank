#[derive(Debug, PartialEq)]
pub struct Player {
    board: Vec<dyn Card>,
    hand: Vec<dyn Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>, board: Vec<Card>) -> Self {
        Player { board, hand }
    }
}