use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;
use crate::web::card_component::CardComponent;

use yew::prelude::*;

pub struct GameComponent {
    cards: Vec<Card>,
}

impl Component for GameComponent {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let cards = (0..12).map(|_| rng.gen::<Card>()).collect();

        Self { cards }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1 class="title">{ "SET " }<small>{ "(Card game)" }</small></h1>
                <div class="game">
                    <div class="sidebar">
                        <div class="options">
                            <button id="restart">{ "Restart" }</button>
                            <button id="add">{ "+3 cards" }</button>
                            <button id="hint">{ "Hint" }</button>
                        </div>
                        <div class="log-container">
                            <h2 class="log-header">{ "Log" }</h2>
                            <div class="log">
                            </div>
                        </div>
                    </div>
                    <div class="board">
                        { for self.cards.iter().map(|card| {
                            html! {
                                <CardComponent card=card />
                            }
                        })}
                    </div>
                </div>
                <div class="rules">
                    <h3>{ "How To Play:" }</h3>
                    <p>{ "Use your mouse to select SETs from the board." }</p>
                    <p>{ "A set consists of three cards satisfying all of these conditions:" }</p>
                    <ul>
                        <li>{ "They all have the same number or have three different numbers." }</li>
                        <li>{ "They all have the same symbol or have three different symbols." }</li>
                        <li>{ "They all have the same shading or have three different shadings." }</li>
                        <li>{ "They all have the same color or have three different colors." }</li>
                    </ul>
                    <p>{ "The rules of Set are summarized by: If you can sort a group of three cards into \"two of ____ and one of ____\", then it is not a set." }</p>
                    <p>{ "For example, these three cards form a set:" }</p>
                    <div class="set-example">
                        <CardComponent card=Card::new(Number::One, Color::Red, Symbol::Diamond, Shading::Stripe) />
                        <CardComponent card=Card::new(Number::Two, Color::Red, Symbol::Diamond, Shading::Solid) />
                        <CardComponent card=Card::new(Number::Three, Color::Red, Symbol::Diamond, Shading::Open) />
                    </div>
                    <p>{ "Given any two cards from the deck, there is one and only one other card that forms a set with them." }</p>
                    <p>{ "There are cases when there will not be any sets on the board, you can than use the \"+3 cards\" button for a better chance at finding a set." }</p>
                    <p>{ "You can use the \"Hint\" and \"+3 cards\" buttons if you ever get stuck." }</p>
                    <small>{ "Rules taken from " } <a href="https://en.wikipedia.org/wiki/Set_(game)">{ "Wikipedia" }</a></small><br />
                    <small>{ "Template taken from " } <a href="https://jacobbelanger.com/projects/SET-Card/">{ "JACOB BELANGER" }</a></small>
                </div>
            </div>
        }
    }
}
