mod structures;
mod algorithms;

use structures::shoe::Shoe;
use structures::hand::Hand;
use algorithms::basics;

fn main() {
    let mut shoe = Shoe::new(6); // 6-deck shoe

    let mut player_hand = Hand::new();
    let mut dealer_hand = Hand::new();

    // === Simulate drawing two cards from the shoe ===
    let card1 = 0;
    let card2 = 0;
    let card3 = 2;

    // Remove from shoe
    shoe.remove_card(card1);
    shoe.remove_card(card2);
    shoe.remove_card(card3);

    // Add to hand
    player_hand.add_card(card1);
    player_hand.add_card(card2);
    dealer_hand.add_card(card3);

    println!("{}", algorithms::basics::stand(&mut shoe, &mut dealer_hand, player_hand.get_total()));
}
