use std::collections::HashSet;

use crate::cards::card::Card;
use crate::errors::Error;

#[derive(Debug)]
pub struct Set(Card, Card, Card);

impl Set {
    pub fn try_from_cards(a: Card, b: Card, c: Card) -> Result<Self, Vec<Error>> {
        fn validate_attribute<A: Eq + std::hash::Hash, B>(
            a: Card,
            b: Card,
            c: Card,
            get_attribute: fn(Card) -> A,
            get_error: fn() -> B,
        ) -> Result<(), B> {
            let count = vec![a, b, c]
                .into_iter()
                .map(get_attribute)
                .collect::<HashSet<A>>()
                .len();

            match count {
                2 => Err(get_error()),
                _ => Ok(()),
            }
        }

        match (
            validate_attribute(a, b, c, |c| c.number(), || Error::SetNumberInvalid),
            validate_attribute(a, b, c, |c| c.color(), || Error::SetColorInvalid),
            validate_attribute(a, b, c, |c| c.symbol(), || Error::SetSymbolInvalid),
            validate_attribute(a, b, c, |c| c.shading(), || Error::SetShadingInvalid),
        ) {
            (Ok(()), Ok(()), Ok(()), Ok(())) => Ok(Set(a, b, c)),
            (number, color, symbol, shading) => {
                let errors = vec![number, color, symbol, shading]
                    .into_iter()
                    .filter_map(Result::err)
                    .collect::<Vec<Error>>();

                Err(errors)
            }
        }
    }
}
