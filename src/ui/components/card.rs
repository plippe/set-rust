use yew::prelude::*;

use crate::attributes::color::Color;
use crate::attributes::number::Number;
use crate::attributes::shading::Shading;
use crate::attributes::symbol::Symbol;
use crate::cards::card::Card;

pub struct CardComponent {
    props: CardProps,
}

#[derive(Properties, Clone)]
pub struct CardProps {
    pub card: Card,
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card p-3">
                {
                    (0..self.number())
                        .map(|_| {
                            html! {
                                <svg viewBox="0 0 100 100" height=format!("{}px", 120 / self.number())>
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
        }
    }
}
