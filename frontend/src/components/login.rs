//! Login component.

use yew::prelude::*;

/// Component messages.
#[derive(Debug)]
pub enum Msg {}

/// Login component.
#[derive(Debug)]
pub struct Login;

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Do nothing for now
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">{{"Login"}}</div>
        }
    }
}
