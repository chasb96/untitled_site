mod nav;
mod routes;
mod home;
mod user;
mod api_error;
mod error_pages;
mod sign_up;
mod login;

use yew::prelude::*;
use nav::Nav;
use yew_router::{BrowserRouter, Switch};
use crate::routes::{route, Route};

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav></Nav>

            <Switch<Route> render={route} />
        </BrowserRouter>
    }
}
