#![allow(dead_code)]
use rand::Rng;
use std::fmt::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Tier {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card {
    name: String,
    attack: u32,
    health: u32,
    tier: Tier,
}

impl Card {
    pub fn new(name: &str, attack: u32, health: u32, tier: Tier) -> Self {
        Card {
            name: name.to_string(),
            attack,
            health,
            tier,
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

    pub fn give(&mut self, cards: Vec<Card>) -> Result<(), Error> {
        for card in cards {
            self.inventory.push(card);
        }

        Ok(())
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
        let card_1 = Card::new("cat", 1, 2, Tier::ONE);
        let card_2 = Card::new("cat", 1, 2, Tier::ONE);
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
        let card_options: &[Card] = &[
            Card::new("cat", 1, 2, Tier::ONE),
            Card::new("dog", 2, 1, Tier::ONE),
        ];
        let mut shop = Shop::from_card_options(card_options);

        assert_eq!(shop.take(5).len(), 5);
    }

    #[test]
    fn shop_give_grows_inventory_length() {
        let cards: Vec<Card> = vec![
            Card::new("cat", 1, 2, Tier::ONE),
            Card::new("dog", 2, 1, Tier::ONE),
        ];
        let mut shop = Shop::default();

        let _ = shop.give(cards).expect("should return Ok(())");

        assert_eq!(shop.inventory.len(), 2);
    }
}
