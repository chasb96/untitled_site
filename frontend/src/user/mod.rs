mod api;

use std::sync::RwLock;
use yew::prelude::*;
use api::User as ApiUser;
use yew_router::{hooks::use_navigator, navigator::Navigator};
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub username: AttrValue,
}

#[function_component]
pub fn User(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let state = use_state(|| RwLock::new(None));
    
    if let Ok(guarded_user) = state.read() {
        if guarded_user.is_none() {
            defer_assign_state(
                state.clone(), 
                &props.username, 
                navigator
            )
        }
    }

    let mut user = ApiUser::default();
    if let Ok(guarded_user) = state.read() {
        if let Some(state_user) = guarded_user.clone() {
            user = state_user;
        }
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

fn defer_assign_state(
    state: UseStateHandle<RwLock<Option<ApiUser>>>, 
    username_prop: &str,
    navigator: Navigator
) {
    let username_prop = username_prop.to_owned();

    wasm_bindgen_futures::spawn_local(async move {
        let user = match ApiUser::find_by_username(&username_prop).await {
            Ok(Some(user)) => user,
            Ok(None) => return navigator.push(&Route::NotFound),
            Err(_) => return navigator.push(&Route::InternalServerError),
        };

        if let Ok(mut state_user) = state.write() {
            *state_user = Some(user);
        }
    });
}