//! Registration form.

use crate::router::Route;
use common::registration::{ResponseDTO, SubmitDTO};
use reqwasm::http::Request;
use serde_json::to_string;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

/// Component messages.
#[derive(Debug)]
pub enum Msg {
    /// The form has been submitted.
    Submitted,
    /// Username changed.
    Username(String),
    /// Password changed.
    Password(String),
    /// First name changed.
    FirstName(String),
    /// Last name changed.
    LastName(String),
    /// Server response.
    ServerResponse(Result<(), ResponseDTO>),
}

/// Properties for the registration form
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    /// Registration code (from email).
    pub code: String,
}

/// Registration form component.
#[derive(Debug, Default)]
pub struct RegistrationForm {
    submitted: bool,
    general_err: Option<String>,
    submit_ok: bool,
    username: String,
    user_err: Option<String>,
    password: String,
    pass_err: Option<String>,
    first_name: String,
    last_name: String,
}

impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Username(username) => {
                if self.username != username {
                    self.username = username;
                    self.user_err = None;

                    true
                } else {
                    false
                }
            }
            Msg::Password(password) => {
                if self.password != password {
                    self.password = password;
                    self.pass_err = None;

                    true
                } else {
                    false
                }
            }
            Msg::FirstName(first_name) => {
                if self.first_name != first_name {
                    self.first_name = first_name;

                    true
                } else {
                    false
                }
            }
            Msg::LastName(last_name) => {
                if self.last_name != last_name {
                    self.last_name = last_name;

                    true
                } else {
                    false
                }
            }
            Msg::Submitted => {
                self.submitted = true;
                let username = self.username.clone();
                let password = self.password.clone();
                let first_name = self.first_name.clone();
                let last_name = self.last_name.clone();
                let Props { code } = ctx.props();
                let code = code.clone();

                ctx.link().send_future(async move {
                    let response = Request::post(&format!("/api/v1/register/user/{}", code))
                        .header("Accept", "application/json")
                        .header("Content-Type", "application/json")
                        .body(
                            to_string(&SubmitDTO {
                                username: &username,
                                password: &password,
                                first_name: &first_name,
                                last_name: &last_name,
                            })
                            .expect("could not serialize form DTO to JSON"),
                        )
                        .send()
                        .await
                        .expect("error sending request");

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
                Err(ResponseDTO {
                    username,
                    password,
                    other,
                }) => {
                    self.submitted = false;
                    self.user_err = username;
                    self.pass_err = password;
                    self.general_err = other;
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
                                        self.confirmation()
                                    } else if self.general_err.is_some() {
                                        self.general_error()
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
}

impl RegistrationForm {
    /// Renders the registration form.
    fn form(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let target = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let name = target.name();
            let value = target.value();

            match name.as_str() {
                "username" => Msg::Username(value),
                "password" => Msg::Password(value),
                "first_name" => Msg::FirstName(value),
                "last_name" => Msg::LastName(value),
                other => panic!("unexpected input name: {other}"),
            }
        });

        let onsubmit = ctx.link().callback(|e: FocusEvent| {
            e.prevent_default();
            e.stop_propagation();
            Msg::Submitted
        });

        html! {
            <>
                <h2>{{"Create your account"}}</h2>
                <form {onsubmit}>
                    <div>
                        <label for="username" class="form-label">{"Username"}</label>
                        <input type="text" name="username"
                            class={if self.user_err.is_some() {"form-control is-invalid"} else {"form-control"}}
                            id="username" aria-describedby={if self.user_err.is_some() {"userValidationFeedback"} else {"userHelp"}}
                            placeholder="Username" required=true oninput={oninput.clone()} />
                        {
                            if let Some(ref err) = self.user_err {
                                html! {<div id="userValidationFeedback" class="invalid-feedback">{"Error: "}{err}</div>}
                            } else {
                                html!{<div id="userHelp" class="form-text">{"The unique username you will use to log in to the website."}</div>}
                            }
                        }
                    </div>
                    <div>
                        <label for="password" class="form-label">{"Password"}</label>
                        <input type="password" name="password"
                            class={if self.pass_err.is_some() {"form-control is-invalid"} else {"form-control"}}
                            id="password" aria-describedby={if self.pass_err.is_some() {"passValidationFeedback"} else {"passHelp"}}
                            required=true oninput={oninput.clone()} />
                        {
                            if let Some(ref err) = self.pass_err {
                                html! {<div id="passValidationFeedback" class="invalid-feedback">{"Error: "}{err}</div>}
                            } else {
                                html!{<div id="passHelp" class="form-text">{"Select a strong password."}</div>}
                            }
                        }
                    </div>
                    <div>
                        <label for="first_name" class="form-label">{"First name(s)"}</label>
                        <input type="text" class="form-control" id="first_name" name="first_name"
                            aria-describedby="firstNameHelp" required=true oninput={oninput.clone()} />
                    </div>
                    <div>
                        <label for="last_name" class="form-label">{"Last name(s)"}</label>
                        <input type="text" class="form-control" id="last_name" name="last_name"
                            aria-describedby="lastNameHelp" required=true {oninput} />
                    </div>
                    <button type="submit" class="btn btn-primary" disabled={
                        self.submitted || self.username.is_empty() ||
                        self.password.is_empty() || self.first_name.is_empty() ||
                        self.last_name.is_empty()}>{"Submit"}</button>
                </form>
            </>
        }
    }

    /// Renders the registration confirmation.
    fn confirmation(&self) -> Html {
        let history = use_history().expect("component outside of the router");
        let onclick = Callback::once(move |e: MouseEvent| {
            e.prevent_default();
            history.push(Route::Home)
        });

        html! {
            <>
                <h2>{"Registration successful!"}</h2>
                <p>{"You can now log in."}</p>
                <p><a href="/" title="Home" {onclick}>{"Return home"}</a></p>
            </>
        }
    }

    /// Renders a general error message.
    fn general_error(&self) -> Html {
        let history = use_history().expect("component outside of the router");
        let onclick = Callback::once(move |e: MouseEvent| {
            e.prevent_default();
            history.push(Route::Home)
        });

        html! {
            <>
                <h2>{"An error occurred"}</h2>
                <p>{self.general_err.as_ref().expect("no general error found")}</p>
                <p><a href="/" title="Home" {onclick}>{"Return home"}</a></p>
            </>
        }
    }
}
