use itertools::Itertools;
use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::convert::TryFrom;

#[derive(Debug)]
enum Error {
    SetNumberInvalid,
    SetColorInvalid,
    SetSymbolInvalid,
    SetShadingInvalid,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Symbol {
    Diamond,
    Oval,
    Squiggle,
}

impl Distribution<Symbol> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Symbol {
        match rng.gen::<u8>() % 3 {
            0 => Symbol::Diamond,
            1 => Symbol::Oval,
            2 => Symbol::Squiggle,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Shading {
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

#[derive(Debug, Clone, Copy)]
struct Card {
    number: Number,
    color: Color,
    symbol: Symbol,
    shading: Shading,
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            number: rng.gen(),
            color: rng.gen(),
            symbol: rng.gen(),
            shading: rng.gen(),
        }
    }
}

#[derive(Debug)]
struct Set(Card, Card, Card);

impl TryFrom<(Card, Card, Card)> for Set {
    type Error = Vec<Error>;

    fn try_from(cards: (Card, Card, Card)) -> Result<Self, Self::Error> {
        fn validate_attribute<A: Clone + Eq + std::hash::Hash, B>(
            cards: (Card, Card, Card),
            get_attribute: fn(&Card) -> A,
            get_error: fn() -> B,
        ) -> Result<(), B> {
            let bad_count = 2;
            let count = vec![
                get_attribute(&cards.0),
                get_attribute(&cards.1),
                get_attribute(&cards.2),
            ]
            .into_iter()
            .unique()
            .count();

            if count == bad_count {
                Err(get_error())
            } else {
                Ok(())
            }
        }

        match (
            validate_attribute(cards, |c| c.number, || Error::SetNumberInvalid),
            validate_attribute(cards, |c| c.color, || Error::SetColorInvalid),
            validate_attribute(cards, |c| c.symbol, || Error::SetSymbolInvalid),
            validate_attribute(cards, |c| c.shading, || Error::SetShadingInvalid),
        ) {
            (Ok(()), Ok(()), Ok(()), Ok(())) => Ok(Set(cards.0, cards.1, cards.2)),
            (number, color, symbol, shading) => {
                let errs = vec![number, color, symbol, shading]
                    .into_iter()
                    .filter_map(Result::err)
                    .collect();

                Err(errs)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut rng = StdRng::seed_from_u64(0);
    let cards = (0..16).map(|_| rng.gen::<Card>());
    cards
        .combinations(3)
        .map(|cs| match cs[..] {
            [c1, c2, c3] => Set::try_from((c1, c2, c3)),
            _ => unreachable!(),
        })
        .for_each(|r| println!("{:?}", r));
}
