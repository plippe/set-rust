use itertools::Itertools;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Number {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Diamond,
    Oval,
    Squiggle,
}

impl Distribution<Shape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen::<u8>() % 3 {
            0 => Shape::Diamond,
            1 => Shape::Oval,
            2 => Shape::Squiggle,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Texture {
    Filled,
    Hollow,
    Shaded,
}

impl Distribution<Texture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Texture {
        match rng.gen::<u8>() % 3 {
            0 => Texture::Filled,
            1 => Texture::Hollow,
            2 => Texture::Shaded,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Card {
    number: Number,
    color: Color,
    shape: Shape,
    texture: Texture,
}

impl Card {
    fn is_same_attribute<A: Eq>(
        card1: &Card,
        card2: &Card,
        card3: &Card,
        get_attribute: fn(&Card) -> A,
    ) -> bool {
        get_attribute(card1) == get_attribute(card2) && get_attribute(card1) == get_attribute(card3)
    }

    fn is_different_attribute<A: Eq>(
        card1: &Card,
        card2: &Card,
        card3: &Card,
        get_attribute: fn(&Card) -> A,
    ) -> bool {
        get_attribute(card1) != get_attribute(card2)
            && get_attribute(card1) != get_attribute(card3)
            && get_attribute(card2) != get_attribute(card3)
    }

    fn is_same_or_different_attribute<A: Eq>(
        card1: &Card,
        card2: &Card,
        card3: &Card,
        get_attribute: fn(&Card) -> A,
    ) -> bool {
        Self::is_same_attribute(card1, card2, card3, get_attribute)
            || Self::is_different_attribute(card1, card2, card3, get_attribute)
    }

    fn is_set(card1: &Card, card2: &Card, card3: &Card) -> bool {
        Self::is_same_or_different_attribute(card1, card2, card3, |c: &Card| c.number)
            && Self::is_same_or_different_attribute(card1, card2, card3, |c: &Card| c.color)
            && Self::is_same_or_different_attribute(card1, card2, card3, |c: &Card| c.shape)
            && Self::is_same_or_different_attribute(card1, card2, card3, |c: &Card| c.texture)
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            number: rng.gen(),
            color: rng.gen(),
            shape: rng.gen(),
            texture: rng.gen(),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut rng = StdRng::seed_from_u64(0);
    let cards = (0..16).map(|_| rng.gen::<Card>());
    let sets = cards
        .combinations(3)
        .map(|cs| match cs[..] {
            [c1, c2, c3] => (c1, c2, c3),
            _ => unreachable!(),
        })
        .filter(|(c1, c2, c3)| Card::is_set(c1, c2, c3));

    sets.for_each(|(c1, c2, c3)| println!("Set:\n - {:?}\n - {:?}\n - {:?}", c1, c2, c3));
}
