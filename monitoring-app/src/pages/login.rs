#![allow(non_snake_case)]

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
    let mut context = use_context::<Signal<LoginInfo>>();
    rsx! {div{
            button {
            onclick: move |event| {
                // let actual = context().logged_in;
                context.write().logged_in = !context().logged_in ;
                info!("Submit button pressed : {:?}", context());
            },
            "Login"
            }
    }
    }
}


pub fn Login() -> Element {
    use_context_provider(||Signal::new(LoginInfo{
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