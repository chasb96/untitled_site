use yew::prelude::*;

#[function_component]
pub fn ContentPane() -> Html {
    html! {
        <div class="container bg-black mt-4 border border-dark rounded">
            { "Content" }
        </div>
    }
}