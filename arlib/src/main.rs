use arlib::card::{Card, Dog};
use arlib::shop::Shop;

fn main() {
    let cards: Vec<Box<dyn Card>> = vec![Box::<Dog>::default()];

    let shop = Shop::from_card_options(cards);

    println!("{:?}", shop);
}
