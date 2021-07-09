use itertools::Itertools;

use crate::cards::{card::Card, set::Set, stock::Stock};

#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    stock: Stock,
    hand: Vec<Card>,
}

impl Pack {
    pub fn from_seed(seed: u64) -> Self {
        let mut stock = Stock::from_seed(seed);
        let mut hand = Vec::new();

        while !Self::is_hand_valid(&hand) {
            hand = (0..12).flat_map(|_| stock.next()).collect();
        }

        Self { stock, hand }
    }

    pub fn swap_selected(&self, set: Set) -> Self {
        let mut stock = self.stock.clone();
        let mut hand = Vec::new();

        while !Self::is_hand_valid(&hand) {
            hand = self
                .hand
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

        Self { stock, hand }
    }

    fn is_hand_valid(hand: &[Card]) -> bool {
        hand.len() == 12
            && hand.iter().unique().collect_vec().len() == 12
            && hand
                .iter()
                .tuple_combinations()
                .any(|(a, b, c)| Set::try_from_cards(*a, *b, *c).is_ok())
    }

    pub fn hand(&self) -> Vec<Card> {
        self.hand.clone()
    }
}
