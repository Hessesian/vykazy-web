use yew::{Html, html, function_component};
use crate::tools::vykaz::vykaz_component::MainScreen;

mod tools;
mod dashboard;
use yew_router::prelude::*;
use dashboard::dashboard_screen::Dashboard;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/tools")]
    Tools,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Dashboard /> },
        Route::Tools => html! {
            <MainScreen />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<Main>::new().render();
}
