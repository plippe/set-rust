use crate::cards::card::Card;
use crate::decks::deck::Deck;
use crate::errors::Error;
use crate::sets::set::Set;
use crate::ui::components::card::CardComponent;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct TableComponent {
    deal: Vec<Card>,
    selected: Vec<Card>,
    warning: Option<String>,
    props: TableProps,
    link: ComponentLink<Self>,
}

impl TableComponent {
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
            [a, b, c] => Option::Some(Set::try_from_cards(*a, *b, *c)),
            _ => Option::None,
        }
    }

    fn swap_selected(&self) -> Self {
        let mut deck = self.props.deck.clone();
        let deal = self
            .deal
            .clone()
            .into_iter()
            .flat_map(|card| {
                if self.selected.contains(&card) {
                    deck.next()
                } else {
                    Option::Some(card)
                }
            })
            .collect();

        Self {
            deal,
            props: TableProps { deck },
            ..self.clone()
        }
    }

    fn select(&self, card: &Card) -> Self {
        let new = self.toggle_selected(card);
        let set = new.extract_set();

        match set {
            None => new,
            Some(Err(_)) => Self {
                selected: Vec::new(),
                warning: Option::Some("This isn't a valid set".to_owned()),
                ..new
            },
            Some(Ok(_)) => Self {
                selected: Vec::new(),
                ..new.swap_selected()
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TableMsgs {
    OnCardClick(Card),
    OnWarningCloseClick,
}

#[derive(Properties, Clone, Debug)]
pub struct TableProps {
    pub deck: Deck,
}

impl Component for TableComponent {
    type Message = TableMsgs;
    type Properties = TableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut props = props;
        let deal = (0..12).flat_map(|_| props.deck.next()).collect();
        let selected = Vec::new();
        let warning = Option::None;

        Self {
            deal,
            selected,
            warning,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::OnCardClick(card) => {
                let new = self.select(&card);

                self.selected = new.selected;
                self.warning = new.warning;
                self.deal = new.deal;
                self.props = new.props;
                true
            }
            Self::Message::OnWarningCloseClick => {
                self.warning = Option::None;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="row row-cols-3 row-cols-md-4 g-2">
                    {
                        self
                            .deal
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
