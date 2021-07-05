#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Shading {
    Open,
    Solid,
    Stripe,
}

impl Shading {
    pub fn all() -> Vec<Self> {
        vec![Self::Open, Self::Solid, Self::Stripe]
    }
}
