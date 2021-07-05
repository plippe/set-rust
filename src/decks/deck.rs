use rand::prelude::*;
use rand::rngs::StdRng;

use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;

#[derive(Debug, Clone)]
pub struct Deck(Vec<Card>);
impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for color in Color::all() {
            for number in Number::all() {
                for shading in Shading::all() {
                    for symbol in Symbol::all() {
                        cards.push(Card::new(color, number, shading, symbol));
                    }
                }
            }
        }

        Self(cards)
    }

    pub fn shuffle(&self) -> Self {
        let mut rng = StdRng::seed_from_u64(0);

        let mut cards = self.0.clone();
        cards.shuffle(&mut rng);

        Self(cards)
    }
}

impl Iterator for Deck {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
