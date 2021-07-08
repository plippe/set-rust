use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;
use crate::ui::games::card::CardComponent;
use yew::prelude::*;

pub struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Instructions" }</h1>
                <p>{ "
                    The object of the game is to identify a SET of 3 cards from 12 cards placed face-up on the table.
                    Each card has four features, which can vary as follows:
                " }</p>

                <ul>
                    <li><span class="fw-bold">{ "SYMBOLS: "}</span>{ "Each card contains a circle, square, or triangle" }</li>
                    <li><span class="fw-bold">{ "COLORS: "}</span>{ "The symbols are blue, pink, or yellow" }</li>
                    <li><span class="fw-bold">{ "NUMBER: "}</span>{ "Each card contains one, two, or three symbols" }</li>
                    <li><span class="fw-bold">{ "SHADING: "}</span>{ "The symbols are either solid, open, or striped" }</li>
                </ul>

                <p>{ "
                    A SET consists of 3 cards in which each of the card's features, looked at one by one, are the
                    same on each card, or, are different on each card. All of the features must separately satisfy
                    this rule. In other words: the shape must be either the same on all 3 cards, or different on each
                    of the 3 cards; the color must be either the same on all 3 cards, or different on each of the 3
                    cards, etc.
                " }</p>

                <h2>{ "A QUICK CHECK - Is it a SET?" }</h2>
                <p>{ "
                    If 2 are the same and 1 is different in any feature, then it is not a SET. For example, if 2 are
                    blue and 1 is pink then it is not a SET. A SET must be either all the same OR all different in each
                    individual feature.
                " }</p>

                <h2>{ "Examples" }</h2>
                <h3>{ "The following are SETs:" }</h3>
                <div class="row row-cols-3 row-cols-md-4 g-2 my-3">
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Two, Shading::Stripe, Symbol::Circle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Two, Shading::Open, Symbol::Circle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Two, Shading::Solid, Symbol::Circle) />
                    </div>
                </div>
                <p>{ "
                    All three cards are the same color; all three cards are the same shape; all three cards have the
                    same number; and all three cards have different shading.
                " }</p>

                <div class="row row-cols-3 row-cols-md-4 g-2 my-3">
                    <div class="col">
                        <CardComponent card=Card::new(Color::Yellow, Number::One, Shading::Stripe, Symbol::Triangle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Blue, Number::Two, Shading::Stripe, Symbol::Circle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Three, Shading::Stripe, Symbol::Square) />
                    </div>
                </div>
                <p>{ "
                    All three cards are different colors; all three cards are different shapes; all three cards have
                    different numbers; and all three cards have the same shading.
                " }</p>

                <div class="row row-cols-3 row-cols-md-4 g-2 my-3">
                    <div class="col">
                        <CardComponent card=Card::new(Color::Blue, Number::One, Shading::Stripe, Symbol::Circle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Yellow, Number::Two, Shading::Solid, Symbol::Square) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Three, Shading::Open, Symbol::Triangle) />
                    </div>
                </div>
                <p>{ "
                    All three cards are different colors; all three cards are different shapes; all three cards have
                    different numbers; and all three cards have different shading.
                " }</p>

                <h3>{ "The following are not SETs:" }</h3>
                <div class="row row-cols-3 row-cols-md-4 g-2 my-3">
                    <div class="col">
                        <CardComponent card=Card::new(Color::Yellow, Number::One, Shading::Solid, Symbol::Square) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Blue, Number::One, Shading::Open, Symbol::Square) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::One, Shading::Open, Symbol::Square) />
                    </div>
                </div>
                <p>{ "
                    All three cards are different colors; all three cards are the same shape; all three cards have the
                    same number; however, two cards have the same shading and one card has different shading.
                " }</p>

                <div class="row row-cols-3 row-cols-md-4 g-2 my-3">
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Two, Shading::Solid, Symbol::Triangle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Pink, Number::Two, Shading::Open, Symbol::Triangle) />
                    </div>
                    <div class="col">
                        <CardComponent card=Card::new(Color::Yellow, Number::Two, Shading::Open, Symbol::Triangle) />
                    </div>
                </div>
                <p>{ "
                    All three cards are the same shape; all three cards have the same number; all three cards have
                    different shading; however, two cards are the same color and one card is a different color.
                " }</p>

                <hr />

                <p class="text-end">
                    <a href="https://www.setgame.com/">{ "www.setgame.com" }</a> { " / " }
                    <a href="https://www.asmodeenordics.com/">{ "www.asmodeenordics.com" }</a>
                </p>
            </>
        }
    }
}
