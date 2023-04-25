#![allow(dead_code)]
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    name: String,
    attack: u32,
    health: u32,
}

impl Card {
    pub fn new(name: &str, attack: u32, health: u32) -> Self {
        Card {
            name: name.to_string(),
            attack,
            health,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Player {
    board: Vec<Card>,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>, board: Vec<Card>) -> Self {
        Player { board, hand }
    }
}

#[derive(Debug)]
pub struct Shop {
    inventory: Vec<Card>,
}

impl Shop {
    pub fn new() -> Self {
        Shop { inventory: vec![] }
    }

    pub fn from_card_options(card_options: &[Card]) -> Self {
        let mut inventory: Vec<Card> = vec![];

        // build inventory for shop
        for card in card_options {
            for _ in 0..10 {
                inventory.push(card.clone());
            }
        }

        Shop { inventory }
    }

    pub fn take(&mut self, count: usize) -> Vec<Card> {
        let mut rng = rand::thread_rng();
        let mut result: Vec<Card> = vec![];

        for _ in 0..count {
            let rand_index: usize = rng.gen_range(0..self.inventory.len()) - 1;
            let card = self.inventory.remove(rand_index);
            result.push(card)
        }

        result
    }
}

impl Default for Shop {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec;

    #[test]
    fn card_works() {
        let card_1 = Card::new("cat", 1, 2);
        let card_2 = Card::new("cat", 1, 2);
        assert_eq!(card_1, card_2);
    }
    #[test]
    fn player_works() {
        let player_1 = Player::new(vec![], vec![]);
        let player_2 = Player::new(vec![], vec![]);
        assert_eq!(player_1, player_2);
    }
    #[test]
    fn shop_take_5_returns_5_cards() {
        let card_options: &[Card] = &[Card::new("cat", 1, 2), Card::new("dog", 2, 1)];
        let mut shop = Shop::from_card_options(card_options);

        assert_eq!(shop.take(5).len(), 5);
    }
}
