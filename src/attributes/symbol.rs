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
        super::gen_one_of(rng, &[Symbol::Diamond, Symbol::Oval, Symbol::Squiggle])
    }
}
