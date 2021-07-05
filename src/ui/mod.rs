mod components;
mod index;

use crate::ui::index::Index;

pub fn main() {
    yew::start_app::<Index>()
}
