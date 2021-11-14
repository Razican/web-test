use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

/// The messages the component can use to update itself.
pub enum Msg {
    /// This message contains a new input name.
    InputName(String),
}

/// Hello is a component that says hello to the name you provide.
pub struct Hello {
    link: ComponentLink<Self>,
    name: String,
}

impl Component for Hello {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputName(name) => {
                if name != self.name {
                    self.name = name;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let name_input = self.link.callback(|e: InputData| Msg::InputName(e.value));

        html! {
            <div>
                <p>{ "What is your name? " }
                    <input type="text" oninput={name_input} />
                </p>
                <p>{ format!("Hello {}!", self.name) }</p>
            </div>
        }
    }
}
