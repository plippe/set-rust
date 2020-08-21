use itertools::Itertools;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::convert::TryFrom;

mod attributes;
mod cards;
mod errors;
mod sets;

use crate::cards::Card;
use crate::sets::Set;

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
