use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="container text-center mt-5">
            <h1 class="text-white">{ "You look lost. Are you having a Biden moment?" }</h1>
            <h5 class="text-white-50">{ "The resource you requested could not be found." }</h5>
        </div>
    }
}