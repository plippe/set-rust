use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Shading {
    Open,
    Solid,
    Stripe,
}

impl Distribution<Shading> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shading {
        match rng.gen::<u8>() % 3 {
            0 => Shading::Open,
            1 => Shading::Solid,
            2 => Shading::Stripe,
            _ => unreachable!(),
        }
    }
}
