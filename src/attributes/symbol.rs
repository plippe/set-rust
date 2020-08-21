use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Symbol {
    Diamond,
    Oval,
    Squiggle,
}

impl Distribution<Symbol> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Symbol {
        match rng.gen::<u8>() % 3 {
            0 => Symbol::Diamond,
            1 => Symbol::Oval,
            2 => Symbol::Squiggle,
            _ => unreachable!(),
        }
    }
}
