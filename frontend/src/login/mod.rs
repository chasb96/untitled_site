mod api;

use std::sync::RwLock;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::HtmlInputElement;
use yew_router::{hooks::use_navigator, navigator::Navigator};
use crate::{routes::Route, login::api::User};

#[function_component]
pub fn Login() -> Html {
    let username = use_state(|| RwLock::new(String::new()));
    let password = use_state(|| RwLock::new(String::new()));

    let on_username = username_change_callback(username.clone());
    let on_password = password_change_callback(password.clone());

    let onclick = submit_callback(
        username.clone(), 
        password.clone(), 
        use_navigator().unwrap()
    );

    html! {
        <div class="container-md bg-black mt-4 border border-dark rounded" style="max-width: 34rem;">
            <h2 class="mt-3 ms-3 text-white">{ "Login" }</h2>
            <hr class="border-dark opacity-100" />

            <form>
                <div class="mb-3 mx-3">
                    <label class="form-label text-white">{ "Username" }</label>
                    <input 
                        type="text" 
                        class="form-control bg-dark border-dark text-white" 
                        name="username" 
                        oninput={on_username}
                    />
                </div>

                <div class="mb-3 mx-3">
                    <label class="form-label text-white">{ "Password" }</label>
                    <input 
                        type="password" 
                        class="form-control bg-dark border-dark text-white" 
                        name="password" 
                        oninput={on_password}
                    />
                </div>

                <hr class="border-dark opacity-100 mt-4" />

                <div class="float-end me-3">
                    <button type="button" class="btn btn-primary" {onclick}>{ "Submit" }</button>
                </div>
                <div class="clearfix mb-3"></div>
            </form>
        </div>
    }
}

fn username_change_callback(username_state: UseStateHandle<RwLock<String>>) -> Callback<InputEvent> {
    Callback::from(move |input_event: InputEvent| {
        let target = input_event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            
        if let Some(input) = input {
            if let Ok(mut lr) = username_state.write() {
                *lr = input.value();
            }
        }
    })
}

fn password_change_callback(password_state: UseStateHandle<RwLock<String>>) -> Callback<InputEvent> {
    Callback::from(move |input_event: InputEvent| {
        let target = input_event.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            
        if let Some(input) = input {
            if let Ok(mut lr) = password_state.write() {
                *lr = input.value();
            }
        }
    })
}

fn submit_callback(
    username_state: UseStateHandle<RwLock<String>>, 
    password_state: UseStateHandle<RwLock<String>>,
    navigator: Navigator
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let navigator = navigator.clone();
        let username_state = username_state.clone();
        let password_state = password_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            if let (Ok(username), Ok(password)) = (username_state.read(), password_state.read()) {
                match User::authenticate(&username, &password).await {
                    Ok(_) => navigator.push(
                        &Route::User { 
                            username: format!("@{}", &username.to_string())
                        }
                    ),
                    _ => navigator.push(&Route::InternalServerError),
                }
            }
        });
    })
}