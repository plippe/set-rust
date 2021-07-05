use yew::prelude::*;

use crate::ui::components::table::TableComponent;

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
                <h1>{ "Set" }</h1>
                <TableComponent />
            </>
        }
    }
}