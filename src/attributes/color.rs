use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    Green,
    Purple,
    Red,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        super::gen_one_of(rng, &[Color::Green, Color::Purple, Color::Red])
    }
}
