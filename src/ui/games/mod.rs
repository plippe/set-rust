pub mod card;

use yew::prelude::*;

use crate::cards::card::Card;
use crate::cards::pack::Pack;
use crate::cards::set::Set;
use crate::errors::Error;
use crate::ui::games::card::CardComponent;

#[derive(Clone, Debug)]
pub struct Index {
    pack: Pack,
    selected: Vec<Card>,
    warning: Option<String>,
    props: IndexProps,
    link: ComponentLink<Self>,
}

impl Index {
    fn toggle_selected(&self, card: &Card) -> Self {
        let mut selected = self.selected.clone();
        if selected.contains(&card) {
            selected.retain(|c| c != card);
        } else {
            selected.push(*card);
        }

        Self {
            selected,
            ..self.clone()
        }
    }

    fn extract_set(&self) -> Option<Result<Set, Vec<Error>>> {
        match self.selected.as_slice() {
            [a, b, c] => Some(Set::try_from_cards(*a, *b, *c)),
            _ => None,
        }
    }

    fn select(&self, card: &Card) -> Self {
        let new = self.toggle_selected(card);
        let set = new.extract_set();

        match set {
            None => new,
            Some(Err(_)) => Self {
                selected: Vec::new(),
                warning: Some("This isn't a valid set".to_owned()),
                ..new
            },
            Some(Ok(set)) => Self {
                pack: self.pack.swap_selected(set),
                selected: Vec::new(),
                ..new
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IndexMsgs {
    OnCardClick(Card),
    OnWarningCloseClick,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct IndexProps {
    pub seed: u64,
}

impl Component for Index {
    type Message = IndexMsgs;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            pack: Pack::from_seed(props.seed),
            selected: Vec::new(),
            warning: None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::OnCardClick(card) => {
                let new = self.select(&card);

                self.pack = new.pack;
                self.selected = new.selected;
                self.warning = new.warning;
                true
            }
            Self::Message::OnWarningCloseClick => {
                self.warning = None;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.pack = Pack::from_seed(props.seed);
            self.selected = Vec::new();
            self.warning = None;
            self.props = props;

            true
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="row row-cols-3 row-cols-md-4 g-2">
                    {
                        self
                            .pack
                            .hand()
                            .iter()
                            .map(|card| {
                                let msg = Self::Message::OnCardClick(*card);
                                let selected = self.selected.contains(card);

                                html! {
                                    <div class="col">
                                        <CardComponent card=*card selected=selected onclick=self.link.callback(move |_| msg)/>
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                </div>
                {
                    self.warning
                        .as_ref()
                        .map(|warning| {
                            html! {
                                <div class="position-fixed top-0 start-0 w-100 p-3">
                                    <div class="alert alert-warning alert-dismissible">
                                        { warning }
                                        <a onclick=self.link.callback(|_| Self::Message::OnWarningCloseClick) class="btn-close"></a>
                                    </div>
                                </div>
                            }
                        })
                        .unwrap_or_default()
                }
            </>
        }
    }
}
