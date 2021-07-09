use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Number {
    One,
    Two,
    Three,
}

impl Distribution<Number> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Number {
        *[Number::One, Number::Two, Number::Three]
            .choose(rng)
            .unwrap()
    }
}
