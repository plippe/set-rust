use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Symbol {
    Circle,
    Square,
    Triangle,
}

impl Distribution<Symbol> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Symbol {
        *[Symbol::Circle, Symbol::Square, Symbol::Triangle]
            .choose(rng)
            .unwrap()
    }
}
