use yew::*;
use yew_router::prelude::*;

use crate::ui::Routes;

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
        type A = RouterAnchor<Routes>;

        html! {
            <>
                <nav class="navbar navbar-expand-sm navbar-light bg-light">
                    <div class="container">
                        <span class="navbar-brand" />
                        <a class="navbar-toggler" data-bs-toggle="collapse" data-bs-target=".navbar-collapse">
                            <span class="navbar-toggler-icon"></span>
                        </a>
                        <div class="collapse navbar-collapse justify-content-end" id="collapsableNavbar">
                            <ul class="navbar-nav">
                                <li class="nav-item">
                                    <A route=Routes::Index classes="nav-link">{ "New Game" }</A>
                                </li>
                                <li class="nav-item">
                                    <A route=Routes::Instructions classes="nav-link">{ "Instructions" }</A>
                                </li>
                            </ul>
                        </div>
                    </div>
                </nav>
                <div class="container my-3">
                    <Router<Routes, ()> render=Router::render(|r: Routes| r.view()) />
                </div>
            </>
        }
    }
}
