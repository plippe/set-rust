mod games;
mod index;
mod instructions;

use crate::decks::deck::Deck;
use yew::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum Routes {
    #[to = "/!"]
    Index,
    #[to = "/instructions!"]
    Instructions,
}

impl Routes {
    pub fn view(&self) -> Html {
        match self {
            Routes::Index => html! { <games::Index deck=Deck::new().shuffle() /> },
            Routes::Instructions => html! { <instructions::Index /> },
        }
    }
}

pub fn main() {
    yew::start_app::<index::Index>()
}
