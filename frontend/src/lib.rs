mod nav;
mod content_pane;

use yew::prelude::*;
use nav::Nav;
use content_pane::ContentPane;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <Nav></Nav>
            <ContentPane></ContentPane>
        </div>
    }
}
