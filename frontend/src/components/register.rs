//! Registration form.

use yew::prelude::*;

/// Component messages.
#[derive(Debug)]
pub enum Msg {}

/// Properties for the registration form
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub code: String,
}

/// Registration form component.
#[derive(Debug)]
pub struct RegistrationForm;

impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // Do nothing for now
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { ref code } = ctx.props();

        html! {
            <div class="container">{{"Registration form, code: "}}{{ code }}</div>
        }
    }
}
