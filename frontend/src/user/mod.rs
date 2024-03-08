mod api;

use yew::prelude::*;
use api::User as ApiUser;
use yew_router::hooks::use_navigator;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: AttrValue,
}

#[function_component]
pub fn User(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let user_state = use_state(|| None);
    let mut user = ApiUser::default();
    
    if user_state.is_none() {
        let username_prop = props.username.clone();
        let user_state = user_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match ApiUser::find_by_username(&username_prop).await {
                Ok(Some(user)) => user_state.set(Some(user)),
                _ => navigator.push(&Route::NotFound),
            }
        });
    }

    if let Some(user_state) = (*user_state).clone() {
        user = user_state.clone();
    }

    html! {
        <div class="container bg-black mt-4 border border-dark rounded">
            <div class="row p-2">
                <div class="col-md-3 border-end border-dark">
                </div>

                <div class="col-md-9">
                    <h3 class="text-white">{ user.username }</h3>
                </div>
            </div>
        </div>
    }
}