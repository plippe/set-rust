use crate::cards::card::Card;
use crate::decks::deck::Deck;
use crate::ui::components::card::CardComponent;
use yew::prelude::*;

pub struct TableComponent {
    deck: Deck,
    hand: Vec<Card>,
}

impl Component for TableComponent {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut deck = Deck::new().shuffle();

        let hand = (0..12).flat_map(|_| deck.next()).collect();

        Self { deck, hand }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.hand.iter().map(|card| html! {
                    <>
                        <CardComponent card=card.clone() />
                        <br />
                    </>
                }).collect::<Html>() }
            </div>
        }
    }
}
