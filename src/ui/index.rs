use yew::prelude::*;

use crate::ui::{AppAnchor, AppRouter, Routes};

pub struct Index;
impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
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
                <div class="navbar navbar-expand-sm navbar-light bg-light">
                    <div class="container">
                        <span class="navbar-brand" />
                        <a class="navbar-toggler" data-bs-toggle="collapse" data-bs-target=".navbar-collapse">
                            <span class="navbar-toggler-icon"></span>
                        </a>
                        <div class="collapse navbar-collapse justify-content-end" id="collapsableNavbar">
                            <ul class="navbar-nav">
                                <li class="nav-item">
                                    <AppAnchor route=Routes::Index.with_base_uri() classes="nav-link">{ "New Game" }</AppAnchor>
                                </li>
                                <li class="nav-item">
                                    <AppAnchor route=Routes::Instructions.with_base_uri() classes="nav-link">{ "Instructions" }</AppAnchor>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <div class="container my-3">
                    <AppRouter render=AppRouter::render(Routes::render) />
                </div>
            </>
        }
    }
}
