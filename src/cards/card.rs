use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::attributes::*;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub number: Number,
    pub color: Color,
    pub symbol: Symbol,
    pub shading: Shading,
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            number: rng.gen(),
            color: rng.gen(),
            symbol: rng.gen(),
            shading: rng.gen(),
        }
    }
}
