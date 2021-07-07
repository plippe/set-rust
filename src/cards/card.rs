use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq)]
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
