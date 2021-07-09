use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    Blue,
    Pink,
    Yellow,
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        *[Color::Blue, Color::Pink, Color::Yellow]
            .choose(rng)
            .unwrap()
    }
}
