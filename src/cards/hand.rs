use itertools::Itertools;

use crate::cards::{card::Card, set::Set, stock::Stock};

#[derive(Debug, Clone, PartialEq)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn from_stock(stock: &Stock) -> (Stock, Self) {
        let mut stock = stock.clone();
        let mut cards = Vec::new();

        while !Self::is_valid(&cards) {
            cards = (0..12).flat_map(|_| stock.next()).collect();
        }

        (stock, Self { cards })
    }

    pub fn swap(&self, stock: &Stock, set: &Set) -> (Stock, Self) {
        let mut stock = stock.clone();
        let mut cards = Vec::new();

        while !(Self::is_valid(&cards) && Self::is_swap(&cards, set)) {
            cards = self
                .cards
                .clone()
                .into_iter()
                .flat_map(|card| {
                    if set.contains(card) {
                        stock.next()
                    } else {
                        Some(card)
                    }
                })
                .collect();
        }

        (stock, Self { cards })
    }

    pub fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    fn is_valid(cards: &[Card]) -> bool {
        cards.len() == 12
            && cards.iter().unique().collect_vec().len() == 12
            && cards
                .iter()
                .tuple_combinations()
                .any(|(a, b, c)| Set::try_from_cards(*a, *b, *c).is_ok())
    }

    fn is_swap(cards: &[Card], set: &Set) -> bool {
        !cards.iter().any(|&c| set.contains(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck_macros::quickcheck]
    fn from_stock_length(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (_, hand) = Hand::from_stock(&stock);

        assert!(hand.cards.len() == 12)
    }

    #[quickcheck_macros::quickcheck]
    fn from_stock_unique(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (_, hand) = Hand::from_stock(&stock);

        assert!(hand.cards.iter().unique().collect_vec().len() == 12)
    }

    #[quickcheck_macros::quickcheck]
    fn from_stock_contains_set(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (_, hand) = Hand::from_stock(&stock);

        assert!(hand
            .cards
            .iter()
            .tuple_combinations()
            .any(|(a, b, c)| Set::try_from_cards(*a, *b, *c).is_ok()))
    }

    #[quickcheck_macros::quickcheck]
    fn swap_length(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (stock, hand) = Hand::from_stock(&stock);
        let set = hand
            .cards
            .iter()
            .tuple_combinations()
            .find_map(|(a, b, c)| Set::try_from_cards(*a, *b, *c).ok())
            .unwrap();
        let (_, hand) = hand.swap(&stock, &set);

        assert!(hand.cards.len() == 12)
    }

    #[quickcheck_macros::quickcheck]
    fn swap_unique(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (stock, hand) = Hand::from_stock(&stock);
        let set = hand
            .cards
            .iter()
            .tuple_combinations()
            .find_map(|(a, b, c)| Set::try_from_cards(*a, *b, *c).ok())
            .unwrap();
        let (_, hand) = hand.swap(&stock, &set);

        assert!(hand.cards.iter().unique().collect_vec().len() == 12)
    }

    #[quickcheck_macros::quickcheck]
    fn swap_contains_set(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (stock, hand) = Hand::from_stock(&stock);
        let set = hand
            .cards
            .iter()
            .tuple_combinations()
            .find_map(|(a, b, c)| Set::try_from_cards(*a, *b, *c).ok())
            .unwrap();
        let (_, hand) = hand.swap(&stock, &set);

        assert!(hand
            .cards
            .iter()
            .tuple_combinations()
            .any(|(a, b, c)| Set::try_from_cards(*a, *b, *c).is_ok()))
    }

    #[quickcheck_macros::quickcheck]
    fn swap_contains_new_cards(seed: u64) {
        let stock = Stock::from_seed(seed);
        let (stock, hand) = Hand::from_stock(&stock);
        let set = hand
            .cards
            .iter()
            .tuple_combinations()
            .find_map(|(a, b, c)| Set::try_from_cards(*a, *b, *c).ok())
            .unwrap();
        let (_, hand) = hand.swap(&stock, &set);

        assert!(!hand.cards.iter().any(|card| set.contains(*card)))
    }
}
