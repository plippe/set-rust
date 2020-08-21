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
        match rng.gen::<u8>() % 3 {
            0 => Number::One,
            1 => Number::Two,
            2 => Number::Three,
            _ => unreachable!(),
        }
    }
}
