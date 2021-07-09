use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::cards::pips::color::Color;
use crate::cards::pips::number::Number;
use crate::cards::pips::shading::Shading;
use crate::cards::pips::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    color: Color,
    number: Number,
    shading: Shading,
    symbol: Symbol,
}

impl Card {
    pub fn new(color: Color, number: Number, shading: Shading, symbol: Symbol) -> Card {
        Card {
            color,
            number,
            shading,
            symbol,
        }
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn number(&self) -> Number {
        self.number
    }
    pub fn shading(&self) -> Shading {
        self.shading
    }
    pub fn symbol(&self) -> Symbol {
        self.symbol
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}
