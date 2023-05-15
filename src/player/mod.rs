use crate::card::Card;

#[derive(Debug, PartialEq)]
pub struct Player {
    board: Vec<dyn Card>,
    hand: Vec<dyn Card>,
}

impl Player {
    pub fn new(hand: Vec<dyn Card>, board: Vec<dyn Card>) -> Self {
        Player { board, hand }
    }
}