//! Frontend router.

use crate::components::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// Application router.
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/register/:code")]
    Register { code: String },
    #[at("/register")]
    EmailRegistration,
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
}

pub fn switch(switch: &Route) -> Html {
    match switch {
        Route::Register { code } => {
            html! { <RegistrationForm code={code.clone()} /> }
        }
        Route::EmailRegistration => {
            html! { <EmailRegistration /> }
        }
        Route::Login => {
            html! { <Login /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
    }
}
