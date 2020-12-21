#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod attributes;
mod cards;
mod errors;
mod sets;
mod web;

use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;

use crate::web::card_component::CardComponent;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>

                <div class="container">
                    <div class="game">
                        <div class="board">
                            <CardComponent card=Card::new(Number::One, Color::Green, Symbol::Diamond, Shading::Stripe) />
                            <CardComponent card=Card::new(Number::Three, Color::Purple, Symbol::Oval, Shading::Solid) />
                            <CardComponent card=Card::new(Number::Two, Color::Red, Symbol::Squiggle, Shading::Open) />

                            <CardComponent card=Card::new(Number::Two, Color::Green, Symbol::Oval, Shading::Open) />
                            <CardComponent card=Card::new(Number::One, Color::Purple, Symbol::Diamond, Shading::Solid) />
                            <CardComponent card=Card::new(Number::Three, Color::Red, Symbol::Squiggle, Shading::Stripe) />
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
