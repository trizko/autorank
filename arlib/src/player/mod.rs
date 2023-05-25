use crate::card::Card;

pub struct Player {
    board: Vec<Box<dyn Card>>,
    hand: Vec<Box<dyn Card>>,
}

impl Player {
    pub fn new(hand: Vec<Box<dyn Card>>, board: Vec<Box<dyn Card>>) -> Self {
        Player { board, hand }
    }
}
