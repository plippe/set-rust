use yew::web_sys::Url;
use yew_router::prelude::*;

#[derive(Clone)]
pub struct SwitchWithBaseUri<A>(A);

impl<A> SwitchWithBaseUri<A> {
    pub fn new(underlying: A) -> Self {
        Self(underlying)
    }

    pub fn underlying(&self) -> &A {
        &self.0
    }

    fn base_path() -> String {
        let href = yew::utils::document()
            .base_uri()
            .ok()
            .flatten()
            .unwrap_or_else(|| "/".to_owned());

        let mut path = Url::new(&href).unwrap().pathname();
        if path.ends_with('/') {
            path.pop();
        }

        path
    }
}

impl<A: Switch> Switch for SwitchWithBaseUri<A> {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        match part.strip_prefix(&Self::base_path()) {
            None => (None, None),
            Some(part) => {
                let (route, state) = A::from_route_part(part.to_owned(), state);
                (route.map(Self), state)
            }
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}
