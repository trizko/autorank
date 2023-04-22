#![allow(dead_code)]

#[derive(Debug, PartialEq)]
struct Card {
    name: String,
    desc: String,
}

impl Card {
    fn new(name: &str, desc: &str) -> Self {
        Card {
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Player {
    board: Vec<Card>,
    hand: Vec<Card>,
}

impl Player {
    fn new(hand: Vec<Card>, board: Vec<Card>) -> Self {
        Player { board, hand }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn card_works() {
        let card_1 = Card::new("cat", "a cat");
        let card_2 = Card::new("cat", "a cat");
        assert_eq!(card_1, card_2);
    }
    #[test]
    fn player_works() {
        let player_1 = Player::new(vec![], vec![]);
        let player_2 = Player::new(vec![], vec![]);
        assert_eq!(player_1, player_2);
    }
}
