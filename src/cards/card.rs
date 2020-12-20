use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    number: Number,
    color: Color,
    symbol: Symbol,
    shading: Shading,
}

impl Card {
    pub fn new(number: Number, color: Color, symbol: Symbol, shading: Shading) -> Card {
        Card {
            number,
            color,
            symbol,
            shading,
        }
    }

    pub fn number(&self) -> Number {
        self.number
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn symbol(&self) -> Symbol {
        self.symbol
    }
    pub fn shading(&self) -> Shading {
        self.shading
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}
