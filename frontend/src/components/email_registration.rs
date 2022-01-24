//! Email registration component.

use common::registration::Email;
use reqwasm::http::Request;
use serde_json::to_string;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Component messages.
#[derive(Debug)]
pub enum Msg {
    /// Form submitted.
    Submitted,
    /// Email input change.
    Email(String),
    /// T&C checkbox change.
    TncCheckbox(bool),
    /// Server response.
    ServerResponse(Result<(), String>),
}

/// Email registration component.
#[derive(Debug, Default)]
pub struct EmailRegistration {
    email: String,
    email_err: Option<String>,
    email_input_node: NodeRef,
    tnc_checkbox: bool,
    submitted: bool,
    submit_ok: bool,
}

impl Component for EmailRegistration {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Email(email) => {
                if self.email != email {
                    self.email = email;
                    self.email_err = None;

                    true
                } else {
                    false
                }
            }
            Msg::TncCheckbox(res) => {
                if self.tnc_checkbox != res {
                    self.tnc_checkbox = res;
                    true
                } else {
                    false
                }
            }
            Msg::Submitted => {
                self.submitted = true;
                let email = self.email.clone();

                ctx.link().send_future(async move {
                    let response = Request::post("/api/v1/register/email")
                        .header("Accept", "application/json")
                        .header("Content-Type", "application/json")
                        .body(to_string(&Email::new(&email)).unwrap())
                        .send()
                        .await
                        .unwrap();

                    Msg::ServerResponse(if response.ok() {
                        Ok(())
                    } else {
                        let res = response
                            .json()
                            .await
                            .expect("could not parse JSON response");

                        Err(res)
                    })
                });

                true
            }
            Msg::ServerResponse(res) => match res {
                Ok(_res) => {
                    self.submit_ok = true;
                    true
                }
                Err(res) => {
                    self.submitted = false;
                    self.email_err = Some(res);
                    true
                }
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="full-page">
                <main class="container">
                    <div class="row d-flex align-items-center">
                        <div class="col-md-6 offset-md-3 card">
                            <div class="card-body">
                                {
                                    if self.submit_ok {
                                        debug_assert!(self.submitted);
                                        self.confirmation(ctx)
                                    } else {
                                        self.form(ctx)
                                    }
                                }
                            </div>
                        </div>
                    </div>
                </main>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(email_input) = self.email_input_node.cast::<HtmlInputElement>() {
                email_input.focus().unwrap();
            }
        }
    }
}

impl EmailRegistration {
    /// Renders the email registration form.
    fn form(&self, ctx: &Context<Self>) -> Html {
        let email_input = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            Msg::Email(target.value())
        });
        let tnc_change = ctx.link().callback(|e: Event| {
            let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();

            Msg::TncCheckbox(target.checked())
        });
        let onsubmit = ctx.link().callback(|e: FocusEvent| {
            e.prevent_default();
            e.stop_propagation();
            Msg::Submitted
        });

        html! {
            <>
                <h2>{{"Register your email"}}</h2>
                <form {onsubmit}>
                    <div>
                        <label for="email" class="form-label">{"Email address"}</label>
                        <input ref={self.email_input_node.clone()} type="email"
                            class={if self.email_err.is_some() {"form-control is-invalid"} else {"form-control"}}
                            id="email" aria-describedby={if self.email_err.is_some() {"emailValidationFeedback"} else {"emailHelp"}}
                            placeholder="user@example.com" required=true oninput={email_input} />
                        {
                            if let Some(ref err) = self.email_err {
                                html! {<div id="emailValidationFeedback" class="invalid-feedback">{"Error: "}{err}</div>}
                            } else {
                                html!{<div id="emailHelp" class="form-text">{"We will send the registration link to this email address"}</div>}
                            }
                        }
                    </div>
                    <div class="form-check">
                        <input type="checkbox" class="form-check-input" id="tncCheck" required=true checked={self.tnc_checkbox} onchange={tnc_change} />
                        <label class="form-check-label" for="tncCheck">{"I accept the terms and conditions"}</label>
                    </div>
                    <button type="submit" class="btn btn-primary" disabled={self.submitted || !self.tnc_checkbox || self.email.is_empty()}>{"Submit"}</button>
                </form>
            </>
        }
    }

    /// Renders the email registration confirmation.
    fn confirmation(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h2>{"Registration successful!"}</h2>
                <p>{"If you provided a correct email address, you will soon receive an email to continue your registration process"}</p>
                <p><a href="/" title="Home">{"Return home"}</a></p>
            </>
        }
    }
}
