#![allow(dead_code)]

use derive::Card;
use dyn_clone::DynClone;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum Tier {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub trait Card: DynClone + Debug {
    fn get_attack(&self) -> u32;
    fn get_health(&self) -> u32;
}

#[derive(Card, Clone, Debug)]
pub struct Dog {
    name: String,
    attack: u32,
    health: u32,
    tier: Tier,
}

impl Default for Dog {
    fn default() -> Dog {
        Dog {
            name: "dog".to_string(),
            attack: 2,
            health: 1,
            tier: Tier::One,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_attack_for_dog() {
        let dog = Dog::default();

        assert_eq!(dog.get_attack(), 2)
    }

    #[test]
    fn get_health_for_dog() {
        let dog = Dog::default();

        assert_eq!(dog.get_health(), 1)
    }
}
