use yew::prelude::*;

use crate::cards::card::Card;
use crate::cards::pips::color::Color;
use crate::cards::pips::number::Number;
use crate::cards::pips::shading::Shading;
use crate::cards::pips::symbol::Symbol;

pub struct CardComponent {
    props: CardProps,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    pub card: Card,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
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
            Color::Blue => "#648FFF".to_string(),
            Color::Pink => "#DC267F".to_string(),
            Color::Yellow => "#FFB000".to_string(),
        }
    }

    fn fill(&self) -> String {
        match self.props.card.shading() {
            Shading::Open => "none".to_string(),
            Shading::Solid => self.color(),
            Shading::Stripe => format!("url(#striped-{})", self.color()),
        }
    }

    fn path(&self) -> String {
        match self.props.card.symbol() {
            Symbol::Circle => {
                "M 50, 50 m -40, 0 a 40,40 0 1,0 80,0 a 40,40 0 1,0 -80,0".to_string()
            }
            Symbol::Square => "M 10,10 90,10 90,90 10,90 Z".to_string(),
            Symbol::Triangle => "M 50,15 90,90 10,90 Z".to_string(),
        }
    }
}

impl Component for CardComponent {
    type Message = ();
    type Properties = CardProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let border = if self.props.selected {
            "border-dark"
        } else {
            ""
        };
        html! {
            <div class=format!("card {} btn h-100", border) onclick=self.props.onclick.clone()>
                <div class="h-100 w-100 p-3 position-absolute top-50 start-50 translate-middle">
                    {
                        (0..self.number())
                            .map(|_| {
                                html! {
                                    <svg viewBox="0 0 100 100" height=format!("calc(100% / {})", self.number()) width="100%">
                                        <defs>
                                            <pattern id= { format!("striped-{}", self.color()) } patternUnits="userSpaceOnUse" width="1" height="7">
                                                <path d="M-1,1 H5" stroke={ self.color() } stroke-width="3" />
                                            </pattern>
                                        </defs>

                                        <path
                                            d={ self.path() }
                                            fill={ self.fill() }
                                            stroke={ self.color() }
                                            stroke-width="9" />
                                    </svg>
                                }
                            })
                            .collect::<Html>()
                    }
                </div>
            </div>
        }
    }
}
