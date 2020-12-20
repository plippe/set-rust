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
        super::gen_one_of(rng, &[Shading::Open, Shading::Solid, Shading::Stripe])
    }
}
