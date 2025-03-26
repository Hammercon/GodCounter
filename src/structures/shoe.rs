#[derive(Copy, Clone, Debug)]
pub struct Shoe {
    cards: [u32; 10],     // [10s, A, 2, ..., 9]
    total_cards: u32,
}

impl Shoe {
    #[inline(always)]
    pub fn new(decks: u32) -> Self {
        let mut cards = [decks * 4; 10];
        cards[0] = decks * 16;

        Self {
            cards,
            total_cards: decks * 52,
        }
    }

    // === SAFE METHODS ===

    #[inline(always)]
    pub fn add_card(&mut self, index: usize) {
        if index < 10 {
            self.cards[index] += 1;
            self.total_cards += 1;
        }
    }

    #[inline(always)]
    pub fn remove_card(&mut self, index: usize) -> bool {
        if index < 10 && self.cards[index] > 0 {
            self.cards[index] -= 1;
            self.total_cards -= 1;
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn card_probability(&self, index: usize) -> f64 {
        if index >= 10 || self.total_cards == 0 {
            0.0
        } else {
            self.cards[index] as f64 / self.total_cards as f64
        }
    }

    // === GETTERS ===

    #[inline(always)]
    pub fn get_card_count(&self, index: usize) -> u32 {
        if index < 10 {
            self.cards[index]
        } else {
            0
        }
    }

    #[inline(always)]
    pub fn total_cards(&self) -> u32 {
        self.total_cards
    }

    #[inline(always)]
    pub fn all_cards(&self) -> &[u32; 10] {
        &self.cards
    }

    // === UNSAFE VERSIONS ===

    #[inline(always)]
    pub fn add_card_unchecked(&mut self, index: usize) {
        unsafe {
            *self.cards.get_unchecked_mut(index) += 1;
        }
        self.total_cards += 1;
    }

    #[inline(always)]
    pub fn remove_card_unchecked(&mut self, index: usize) -> bool {
        unsafe {
            let count = self.cards.get_unchecked_mut(index);
            if *count > 0 {
                *count -= 1;
                self.total_cards -= 1;
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn card_probability_unchecked(&self, index: usize) -> f64 {
        if self.total_cards == 0 {
            return 0.0;
        }

        unsafe {
            *self.cards.get_unchecked(index) as f64 / self.total_cards as f64
        }
    }

    #[inline(always)]
    pub fn get_card_count_unchecked(&self, index: usize) -> u32 {
        unsafe { *self.cards.get_unchecked(index) }
    }

    #[inline(always)]
    pub fn all_cards_unchecked(&self) -> *const u32 {
        self.cards.as_ptr()
    }
}
