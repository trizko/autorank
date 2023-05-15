#[derive(Debug, PartialEq, ClOne)]
pub enum Tier {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

trait Card {
    fn get_attack(&self) -> u32 {
        self.attack
    }
    fn get_health(&self) -> u32 {
        self.health
    }
}

#[derive(Card)]
pub struct Dog {
    name: String,
    attack: u32,
    health: u32,
    tier: Tier,
}

impl Default for Dog {
    fn default() -> Dog {
        Dog {
            name: "dog",
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

        assert_eq!(dog.get_attack, 2)
    }
}