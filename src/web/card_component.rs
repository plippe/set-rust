use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;

use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct CardProperties {
    pub card: Card,
}

pub struct CardComponent {
    props: CardProperties,
}

impl CardComponent {
    fn number(&self) -> usize {
        match self.props.card.number() {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
        }
    }

    fn color(&self) -> String {
        match self.props.card.color() {
            Color::Green => "green".to_string(),
            Color::Red => "red".to_string(),
            Color::Purple => "purple".to_string(),
        }
    }

    fn fill(&self) -> String {
        match self.props.card.shading() {
            Shading::Open => "none".to_string(),
            Shading::Solid => self.color(),
            Shading::Stripe => format!("url(#striped-{})", self.color()),
        }
    }

    // https://jacobbelanger.com/projects/SET-Card/
    fn path(&self) -> String {
        match self.props.card.symbol() {
            Symbol::Diamond => "M25 0 L50 50 L25 100 L0 50 Z".to_string(),
            Symbol::Oval => "M25,99.5C14.2,99.5,5.5,90.8,5.5,80V20C5.5,9.2,14.2,0.5,25,0.5S44.5,9.2,44.5,20v60 C44.5,90.8,35.8,99.5,25,99.5z".to_string(),
            Symbol::Squiggle => "M38.4,63.4c0,16.1,11,19.9,10.6,28.3c-0.5,9.2-21.1,12.2-33.4,3.8s-15.8-21.2-9.3-38c3.7-7.5,4.9-14,4.8-20 c0-16.1-11-19.9-10.6-28.3C1,0.1,21.6-3,33.9,5.5s15.8,21.2,9.3,38C40.4,50.6,38.5,57.4,38.4,63.4z".to_string(),
        }
    }
}

impl Component for CardComponent {
    type Properties = CardProperties;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card fadeIn">
                <div class="card-content">
                    { for (0..self.number()).map(|_| {
                        html! {
                            <svg viewBox="-2 -2 54 104">
                                <defs>
                                    <pattern id= { format!("striped-{}", self.color()) } patternUnits="userSpaceOnUse" width="1" height="7">
                                        <path d="M-1,1 H5" stroke={ self.color() } stroke-width="5" />
                                    </pattern>
                                </defs>

                                <path
                                    d={ self.path() }
                                    fill={ self.fill() }
                                    stroke={ self.color() }
                                    stroke-width="5" />
                            </svg>
                        }
                    })}
                </div>
            </div>
        }
    }
}
