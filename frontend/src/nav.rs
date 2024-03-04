use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <nav class="navbar bg-black border-bottom border-dark">
            <a class="navbar-brand fs-2 text-primary fst-italic text-decoration-none mb-0 pt-0 pb-0 ms-3" href="#">
                { "site.com" }
            </a>
        </nav>
    }
}