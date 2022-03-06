mod app;
mod routes;
mod util;
mod read;

use yew::prelude::*;
use app::App;

fn main() {
    yew::start_app::<App>();
    // yew::start_app::<Dev>();
}

#[function_component(Dev)]
fn dev() -> Html {
    html! {
    <h1 class="text-3xl font-bold underline italic">
    {"Hello world!"}
    </h1>
    }
}
