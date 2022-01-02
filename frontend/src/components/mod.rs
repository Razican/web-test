//! Frontend components.
//!
//! This module contains the main `MySupport` component.

pub mod email_registration;
pub mod home;
pub mod login;
pub mod nav;
pub mod register;

use crate::router::*;
pub use email_registration::*;
pub use home::*;
pub use login::*;
pub use nav::*;
pub use register::*;
use yew::prelude::*;
use yew_router::prelude::*;

/// Component messages.
#[derive(Debug)]
pub enum Msg {}

/// Main component for the MySupport application.
#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Nav />
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}
