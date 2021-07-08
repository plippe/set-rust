#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Color {
    Blue,
    Pink,
    Yellow,
}

impl Color {
    pub fn all() -> Vec<Self> {
        vec![Self::Blue, Self::Pink, Self::Yellow]
    }
}
