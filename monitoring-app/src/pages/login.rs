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
                oninput: move |event| {context.write().username = event.value();
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
    let mut response = use_signal(|| String::from("..."));


    let log_in = move |_| {
        spawn(async move {
            let mut data = HashMap::new();
            data.insert("user_name", "admin");
            data.insert("password", "password");


            let client = reqwest::Client::new()
                .post("https://api.mostovoi.org/hash")
                .json(&data)
                .basic_auth("admin", Some("admin123"))
                .send()
                .await;

            match client {
                Ok(data) => {
                    response.set(String::from(data.text().await.unwrap_or(String::from("nothing received"))));
                }
                Err(err) => {
                    log::info!("Request failed with error: {err:?}")
                }
            }

        }
        );
    };


    let mut context = use_context::<Signal<LoginInfo>>();

    if context().username.is_empty() | context().password.is_empty() {
        rsx! {div{
            button {disabled:true,
            "Login"
            }
    }
    }
    } else {
        rsx! {div{
            button {
            onclick: move |event| {
                context.write().logged_in = !context().logged_in ;
                info!("Submit button pressed : {:?}", context());
                spawn(async move {
                        let mut data = HashMap::new();
                        data.insert("user_name", "admin");
                        data.insert("password", "password");


                        let client = reqwest::Client::new()
                        .post("https://api.mostovoi.org/hash")
                        .json(&data)
                        .basic_auth("admin", Some("admin123"))
                        .send()
                       .await;

            match client {
                Ok(data) => {
                     log::info!("Request successful")
                }
                Err(err) => {
                    log::info!("Request failed with error: {err:?}")
                }
            }

                    });


            },
            "Login"
            },
        button {onclick: post_request,
            "Post Request"},

        // button{
        // onclick: log_in, "Response: {response}"}
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