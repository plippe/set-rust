use itertools::Itertools;
use std::convert::TryFrom;

use crate::cards::Card;
use crate::errors::Error;

#[derive(Debug)]
pub struct Set(Card, Card, Card);

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
