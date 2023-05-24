use autorank::card::{Card, Dog};
use autorank::shop::Shop;

fn main() {
    let cards: Vec<Box<dyn Card>> = vec![
        Box::new(Dog::default()),
    ];

    let shop = Shop::from_card_options(cards);

    println!("{:?}", shop);
}
