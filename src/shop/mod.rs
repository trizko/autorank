use crate::card::Card;
use std::io::Error;

#[derive(Debug)]
pub struct Shop {
    inventory: Vec<Box<dyn Card>>,
}

impl Shop {
    pub fn new() -> Self {
        Shop { inventory: vec![] }
    }

    pub fn from_card_options(card_options: Vec<Box<dyn Card>>) -> Self {
        let mut inventory: Vec<Card> = vec![];

        // build inventory for shop
        for card in card_options {
            for _ in 0..10 {
                inventory.push(card.clone());
            }
        }

        Shop { inventory }
    }

    pub fn take(&mut self, count: usize) -> Vec<Box<dyn Card>> {
        let mut rng = rand::thread_rng();
        let mut result: Vec<Card> = vec![];

        for _ in 0..count {
            let rand_index: usize = rng.gen_range(0..self.inventory.len()) - 1;
            let card = self.inventory.remove(rand_index);
            result.push(card)
        }

        result
    }

    pub fn give(&mut self, cards: Vec<Box<dyn Card>>) -> Result<(), Error> {
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
