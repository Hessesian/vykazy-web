use yew::prelude::*;
use yew_router::prelude::*;
use yew_template::*;
use crate::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    return yew_template::template_html! {"src/templates/Dashboard.html",
                    tools = {route_tools()}};
}

fn route_tools() -> Html {
    html! {
            <Link<Route> classes={classes!("navbar-item")} to={Route::Tools}>
            { "Tools" }
            </Link<Route>>
    }
}