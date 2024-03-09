mod api;

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
    let state = use_state(|| None);
    
    if state.is_none() {
        defer_assign_state(
            state.clone(), 
            &props.username, 
            navigator
        )
    }

    
    let mut username = "";
    if let Some(user) = &(*state) {
        username = &user.username;
    };

    html! {
        <div class="container bg-black mt-4 border border-dark rounded">
            <div class="row p-2">
                <div class="col-md-3 border-end border-dark">
                </div>

                <div class="col-md-9">
                    <h3 class="text-white mt-2">{ username }</h3>
                </div>
            </div>
        </div>
    }
}

fn defer_assign_state(
    state: UseStateHandle<Option<ApiUser>>, 
    username_prop: &str,
    navigator: Navigator
) {
    let username_prop = username_prop.to_owned();
    
    wasm_bindgen_futures::spawn_local(async move {
        let res = ApiUser::find_by_username(&username_prop).await;

        let user = match res {
            Ok(Some(user)) => user,
            Ok(None) => return navigator.push(&Route::NotFound),
            Err(_) => return navigator.push(&Route::InternalServerError),
        };

        state.set(Some(user));
    });
}