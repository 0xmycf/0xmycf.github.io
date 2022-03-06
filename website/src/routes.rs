use yew::prelude::*;
use yew_router::prelude::*;
use crate::util::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/testone")]
    TestOne,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn main_switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
                util::lipsum_text()
        },
        Route::TestOne  => html! { <TestOne /> },
        Route::NotFound => html! { <h1> { "404 - side not found." } </h1> },
    }
}

#[function_component(TestOne)]
fn test_one() -> Html {
    html! {
        util::lipsum_text()
    }
}

