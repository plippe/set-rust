use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Number {
    One,
    Two,
    Three,
}

impl Distribution<Number> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Number {
        super::gen_one_of(rng, &[Number::One, Number::Two, Number::Three])
    }
}
