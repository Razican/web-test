use yew::{
    format::{Nothing, Text},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, Html, InputData, ShouldRender,
};

/// The messages the component can use to update itself.
#[derive(Debug)]
pub enum Msg {
    /// This message contains a new input email.
    InputEmail(String),
    /// This message contains a text response from the backend.
    StringResponse(Text),
}

/// Hello is a component that says hello to the name you provide.
#[derive(Debug)]
pub struct Hello {
    link: ComponentLink<Self>,
    hello: String,
    fetch_task: Option<FetchTask>,
}

impl Component for Hello {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            hello: String::new(),
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputEmail(name) if !name.is_empty() => {
                let request = Request::get(format!("/api/v1/hello/{}", name))
                    .body(Nothing)
                    .expect("Could not build request.");

                let callback = self
                    .link
                    .callback(|response: Response<Text>| Msg::StringResponse(response.into_body()));

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);

                false
            }
            Msg::InputEmail(_) => {
                self.hello = String::new();
                true
            }
            Msg::StringResponse(Ok(hello)) => {
                self.fetch_task = None;
                if hello != self.hello {
                    self.hello = hello;
                    true
                } else {
                    false
                }
            }
            Msg::StringResponse(Err(_)) => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let email_input = self.link.callback(|e: InputData| Msg::InputEmail(e.value));

        html! {
            <div class="container">
                <div class="row">
                    <div class="col"><label for="email">{ "What is your email? " }</label></div>
                    <div class="col"><input type="email" class="form-control" id="email" oninput={email_input} /></div>
                </div>
                <p class="lead">{ &self.hello }</p>
            </div>
        }
    }
}
