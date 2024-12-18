#![allow(non_snake_case)]

use std::collections::HashMap;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info};
use crate::pages::common::BackToLanding;

#[derive(Clone, Debug)]
struct LoginInfo {
    username: String,
    password: String,
    logged_in: bool,
}

#[component]
fn UserNameField() -> Element {
    let mut context = use_context::<Signal<LoginInfo>>();
    rsx! {
        div{
        "Username : ",
         input { r#type: "text",
                color: "green",
                oninput: move |event| {context.write().username = event.value().to_lowercase();
                    info!("Context Value set to  : {:?}",context().username );}
            }

        }
    }
}

#[component]
fn PasswordField() -> Element {
    let mut context = use_context::<Signal<LoginInfo>>();
    rsx! {div{
        "Password : ",
         input { r#type: "password",
            color: "green",
        oninput: move |event| {
                context.write().password = event.value();
                info!("Context Password set to  : {:?}", context().password );}
        }
    }
        }
}

#[component]
fn Submit() -> Element {
    let mut context = use_context::<Signal<LoginInfo>>();
    let mut response = use_signal(|| String::from("..."));


    let log_in = move |_| {
        spawn(async move {

            let client = reqwest::Client::new()
                .get("https://api.mostovoi.org/auth")
                .basic_auth(context().username, Some(context().password))
                .send()
                .await;

            match client {
                Ok(data) => {
                    if data.status().is_success() {
                        response.set(String::from("Logged in"));
                        context.write().logged_in = true;
                        info!("Response : {response:?}");
                    }else {
                        response.set(String::from("Not Logged in"));
                        context.write().logged_in = false;
                        info!("Response : {response:?}");
                    }
                }
                Err(err) => {
                    info!("Request failed with error: {err:?}")
                }
            }

        }
        );
    };

    if context().username.is_empty() | context().password.is_empty() | context().logged_in {
        rsx! {div{
            button {disabled:true,
            if context().logged_in {"Logged"} else {"Login"}
            }
    }
    }
    } else {
        rsx!{div{
            button {
            onclick: log_in,
            "Login"
            }
    }
    }
    }
}


pub fn Login() -> Element {
    use_context_provider(|| Signal::new(LoginInfo {
        username: "".to_string(),
        password: "".to_string(),
        logged_in: false,
    }));

    info!("Login Page opened");
    rsx! {
            h3 {"Login Page"},
            UserNameField{},
            PasswordField{},
            Submit{},
            BackToLanding{}
    }
}