use yew::prelude::*;
use yew_router::prelude::*;
use crate::{error_pages::{InternalServerError, NotFound}, home::Home, login::Login, sign_up::SignUp, user::User};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign_up")]
    SignUp,
    #[at("/login")]
    Login,
    #[at("/:username")]
    User { username: String },
    #[at("/not_found")]
    NotFound,
    #[at("/internal_server_error")]
    InternalServerError
}

pub fn route(route: Route) -> Html {
    match route {
        Route::User { username } if username.starts_with('@') => html! { <User username={username[1..].to_string()} /> },
        Route::SignUp => html! { <SignUp /> },
        Route::Login => html! { <Login /> },
        Route::Home => html! { <Home /> },
        Route::InternalServerError => html! { <InternalServerError /> },
        _ => html! { <NotFound /> },
    }
}