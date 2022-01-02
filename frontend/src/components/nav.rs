use std::collections::HashMap;

use crate::router::Route;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let history = use_history().expect("navigation bar instantiated outside of the router");
    let onclick = Callback::once(move |e: MouseEvent| {
        e.prevent_default();

        let target = e.target().unwrap().dyn_into::<HtmlElement>().unwrap();

        let href = target.get_attribute("href").unwrap();

        let route = Route::from_path(&href, &HashMap::new()).unwrap();

        history.push(route)
    });

    html! {
        <nav>
            <ul class="nav nav-pills justify-content-end">
                <li class="nav-item">
                    <a class="nav-link active" aria-current="page" href="/" onclick={onclick.clone()}>{"Home"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/register" onclick={onclick.clone()}>{"Register"}</a>
                </li>
                <li class="nav-item">
                    <a class="nav-link" href="/login" onclick={onclick.clone()}>{"Log in"}</a>
                </li>
            </ul>
        </nav>
    }
}
