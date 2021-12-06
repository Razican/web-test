//! Homepage component.

use yew::prelude::*;

/// Component messages.
#[derive(Debug)]
pub enum Msg {}

/// Homepage component.
#[derive(Debug)]
pub struct Home;

impl Component for Home {
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
            <main class="container">{{"Home component"}}</main>
        }
    }
}
