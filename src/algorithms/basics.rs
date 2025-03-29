use crate::structures::shoe::Shoe;
use crate::structures::hand::Hand;

pub fn stand(shoe: &mut Shoe, dealer_hand: &mut Hand, player_total: u8) -> f64 {
    let dealer_total: u8 = dealer_hand.get_total();

    if dealer_total > 21 {
        return 2.0;
    }

    if dealer_total > 16 {
        if player_total > dealer_total {
            return 2.0;
        }
        if player_total < dealer_total {
            return 0.0;
        }
        if player_total == dealer_total {
            return 1.0;
        }
    }

    let mut to_return: f64 = 0.0;
    let mut card_prob: f64;

    let mut new_dealer_hand: Hand;

    for i in 0..10 {
        card_prob = shoe.card_probability_unchecked(i);
        if card_prob > 0.0 {
            unsafe {
                new_dealer_hand = *dealer_hand;
            }
            shoe.remove_card_unchecked(i);
            new_dealer_hand.add_card_unchecked(i);

            to_return += card_prob * stand(shoe, &mut new_dealer_hand, player_total);

            shoe.add_card_unchecked(i);
        }
    }

    to_return
}
