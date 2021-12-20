use reqwasm::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Html, InputEvent};

/// The messages the component can use to update itself.
#[derive(Debug)]
pub enum Msg {
    /// This message contains a new input email.
    InputEmail(String),
    /// This message contains a text response from the backend.
    StringResponse(Result<String, reqwasm::Error>),
}

/// Hello is a component that says hello to the name you provide.
#[derive(Debug)]
pub struct Hello {
    hello: String,
}

impl Component for Hello {
    type Message = Msg;
    type Properties = ();

    fn create(_context: &Context<Self>) -> Self {
        Self {
            hello: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputEmail(name) if !name.is_empty() => {
                ctx.link().send_future(async move {
                    let response = Request::get(&format!("/api/v1/hello/{}", name))
                        .send()
                        .await
                        .unwrap();

                    let res = response.text().await;

                    Msg::StringResponse(res)
                });

                false
            }
            Msg::InputEmail(_) => {
                self.hello = String::new();
                true
            }
            Msg::StringResponse(Ok(hello)) => {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let email_input = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            Msg::InputEmail(target.value())
        });

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
