use autorank::{Card, Shop, Tier};

fn main() {
    let card_options: &[Card] = &[
        Card::new("cat", 1, 2, Tier::ONE),
        Card::new("dog", 2, 1, Tier::ONE),
    ];

    let shop = Shop::from_card_options(card_options);

    println!("{:?}", shop);
}
