use yew::prelude::*;

#[function_component]
pub fn InternalServerError() -> Html {
    html! {
        <div class="container text-center mt-5">
            <h1 class="text-white">{ "Whoops!" }</h1>
            <h5 class="text-white-50">{ "Something went wrong." }</h5>
        </div>
    }
}