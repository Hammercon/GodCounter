mod structures;

use structures::shoe::Shoe;
use structures::hand::Hand;

fn main() {
    let mut shoe = Shoe::new(6); // 6-deck shoe
    println!("Initialized shoe with {} cards.", shoe.total_cards());

    let mut player_hand = Hand::new();
    println!("New hand created.");

    // === Simulate drawing two cards from the shoe ===
    let card1 = 1; // Ace
    let card2 = 0; // 10-value

    // Remove from shoe
    shoe.remove_card(card1);
    shoe.remove_card(card2);

    // Add to hand
    player_hand.add_card(card1);
    player_hand.add_card(card2);

    println!("\nAfter drawing Ace and 10:");
    println!("Hand total: {}", player_hand.get_total());
    println!("Is hand soft? {}", player_hand.is_soft());
    println!("Card count in hand: {}", player_hand.card_count());

    println!("\nShoe state after drawing:");
    println!("Total cards left in shoe: {}", shoe.total_cards());
    println!(
        "Cards left (Ace): {}, (10): {}",
        shoe.get_card_count(1),
        shoe.get_card_count(0)
    );

    // === Probability test ===
    let prob_five = shoe.card_probability(5);
    println!("Probability of drawing a 5: {:.2}%", prob_five * 100.0);

    let prob_ace = shoe.card_probability_unchecked(1);
    println!("(Unsafe) Probability of Ace: {:.2}%", prob_ace * 100.0);
}
