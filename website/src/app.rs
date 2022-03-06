use yew_router::prelude::Link;
use yew_router::{BrowserRouter, Switch};
use crate::routes::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>

            // wrapper to make footer sticky to the bottom
            <div class="flex flex-col h-screen justify-between">

                // main content
                <main class="px-5 mb-auto sm:px-40 xl:px-96">
                    <div class="sm:flex">

                    <div class="flex flex-col">
                        // header
                        <header class="flex px-5 sm:px-0 my-1 sm:my-5 text-2xl font-bold">
                            <div class="text-yellow-500">
                                <Link<Route> to={Route::Home}> {"mycf"} </Link<Route>>
                            </div>
                        </header>

                        <Navbar />

                    </div>

                        <div class="mx-5">
                            <Switch<Route> render={Switch::render(main_switch)}/>
                        </div>

                    </div>
                </main>

                // The footer
                <footer class="flex p-6 space-x-5 text-sm justify-center sm:justify-between">

                       // copyright badge 
                        <div>
                            {"©0xmycf"}
                        </div>
                            
                        // build with badge
                        // Needs to have privacy and imprint there too
                        <div>
                            { "Build with " }
                            <a href="https://www.rust-lang.org" class="hover:text-sky-500"> { "Rust" } </a>
                            {", "}
                            <a href="https://yew.rs" class="hover:text-sky-500"> { "Yew" } </a>
                            {" and "}
                            <a href="https://tailwindcss.com" class="hover:text-sky-500"> { "Tailwind" } </a>
                            {"."}
                        </div>

                       // socials     
                        <div>
                            <ul>
                              <li>
                                <a href="https://github.com/0xmycf" class="hover:text-sky-500">
                                    {"GitHub"}
                                </a>
                              </li>
                            </ul>
                        </div>
                </footer>
            </div>
        </BrowserRouter>
    }
}

// The Navbar
struct Navbar;
impl Component for Navbar {
    type Message    = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Navbar
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="sidebar" class="sm:flex sm:flex-col sm:text-right sm:mr-5 px-5 sm:px-0 mb-5 sm:mb-0 space-y-5 space-x-5 sm:space-x-0 lowercase">
              // <Link<Route> to={Route::TestOne}> {"One"} </Link<Route>>
              // <Link<Route> to={Route::Stuff}> {"Stuff"} </Link<Route>>
              <a>{ "I" }</a>
              <a>{ "am" }</a>
              <a>{ "not" }</a>
              <a>{ "finished" }</a>
              <a>{ "yet." }</a>
            </div>
        }
    }
}
