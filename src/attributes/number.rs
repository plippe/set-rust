#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Number {
    One,
    Two,
    Three,
}

impl Number {
    pub fn all() -> Vec<Self> {
        vec![Self::One, Self::Two, Self::Three]
    }
}
