#[derive(Copy, Clone, Debug)]
pub struct Hand {
    cards: [u8; 20], // Max 10 cards (safe upper bound for blackjack)
    count: u8,       // Number of cards in the hand
    total: u8,       // Hand total
    soft: bool,      // True if hand is soft (Ace counted as 11)
}

impl Hand {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            cards: [0; 20],
            count: 0,
            total: 0,
            soft: false,
        }
    }

    // === SAFE METHODS ===

    /// Adds a card by index (0 = 10/J/Q/K, 1 = Ace, 2–9 = 2–9)
    #[inline(always)]
    pub fn add_card(&mut self, card_index: usize) {
        if self.count >= 20 || card_index > 9 {
            return;
        }

        let value = match card_index {
            0 => 10,
            1 => 11,
            _ => card_index as u8,
        };

        self.cards[self.count as usize] = card_index as u8;
        self.count += 1;
        self.total += value;

        if card_index == 1 {
            self.soft = true;
        }

        if self.total > 21 && self.soft {
            self.total -= 10;
            self.soft = false;
        }
    }

    /// Fastest safe way to get the total value of the hand
    #[inline(always)]
    pub fn get_total(&self) -> u8 {
        self.total
    }

    /// Returns true if the hand is soft (Ace counted as 11)
    #[inline(always)]
    pub fn is_soft(&self) -> bool {
        self.soft
    }

    /// Returns the number of cards in the hand
    #[inline(always)]
    pub fn card_count(&self) -> u8 {
        self.count
    }

    // === UNSAFE VERSIONS ===

    /// Unsafe version of `add_card`, skips all bounds checks
    #[inline(always)]
    pub fn add_card_unchecked(&mut self, card_index: usize) {
        let value = match card_index {
            0 => 10,
            1 => 11,
            _ => card_index as u8,
        };

        self.cards[self.count as usize] = card_index as u8;
        self.count += 1;
        self.total += value;

        if card_index == 1 {
            self.soft = true;
        }

        if self.total > 21 && self.soft {
            self.total -= 10;
            self.soft = false;
        }
    }
}
