mod games;
mod index;
mod instructions;
mod yew_helpers;

use crate::decks::deck::Deck;
use yew::prelude::*;
use yew_helpers::switch::SwitchWithBaseUri;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone, Copy)]
pub enum Routes {
    #[to = "/!"]
    Index,
    #[to = "/#instructions!"]
    Instructions,
}

impl Routes {
    pub fn render(route: SwitchWithBaseUri<Self>) -> Html {
        match route.underlying() {
            Routes::Index => html! { <games::Index deck=Deck::new().shuffle() /> },
            Routes::Instructions => html! { <instructions::Index /> },
        }
    }

    pub fn with_base_uri(&self) -> SwitchWithBaseUri<Self> {
        SwitchWithBaseUri::new(*self)
    }
}

type AppAnchor = RouterAnchor<SwitchWithBaseUri<Routes>>;
type AppRouter = Router<SwitchWithBaseUri<Routes>, ()>;

pub fn main() {
    yew::start_app::<index::Index>()
}
