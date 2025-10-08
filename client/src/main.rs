use yew::prelude::*;
use yew_router::prelude::*;

use pages::{Home, Login, NewProject};

mod components;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin/new_project")]
    NewProject,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Login => html! {
            <Login />
        },
        Route::NewProject => html! {
            <NewProject />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
