#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};
use serde::{Deserialize, Serialize};
use crate::pages::common::{BackToLanding, UserSharedStatus};

#[derive(Clone, Debug, PartialEq)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Serialize,Deserialize)]
struct Token{
    jwt: String,
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
    let context = use_context::<Signal<LoginInfo>>();
    let mut global_context = use_context::<Signal<UserSharedStatus>>();


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

                        match data.json().await{
                            Ok(response)=>{
                                let token: Token = response;
                                info!("Token Received");
                                global_context.write().token = token.jwt;
                                global_context.write().username = context().username.clone();
                                global_context.write().logged_in = true;
                            }
                            Err(e)=>{info!("Request failed: {:?}", e);}
                        }

                    }else {
                        info!("Response : {0:?}", data.status());
                    }
                }
                Err(err) => {
                    info!("Request failed with error: {err:?}");
                }
            }

        }
        );
    };

    if context().username.is_empty() | context().password.is_empty() | global_context().logged_in {
        rsx! {div{
            button {disabled:true,
            if global_context().logged_in {"Logged"} else {"Login"}
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