use autorank::Shop;
use autorank::Card;

fn main() {
    let card_options: &[Card] = &[
        Card::new("cat", 1, 2), 
        Card::new("dog", 2, 1),
    ];

    let shop = Shop::from_card_options(card_options);

    println!("{:?}", shop);
}