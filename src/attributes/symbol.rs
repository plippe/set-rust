#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Symbol {
    Circle,
    Square,
    Triangle,
}

impl Symbol {
    pub fn all() -> Vec<Self> {
        vec![Self::Circle, Self::Square, Self::Triangle]
    }
}
