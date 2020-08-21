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
        match rng.gen::<u8>() % 3 {
            0 => Color::Green,
            1 => Color::Purple,
            2 => Color::Red,
            _ => unreachable!(),
        }
    }
}
