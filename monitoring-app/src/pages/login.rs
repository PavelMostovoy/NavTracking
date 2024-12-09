#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::pages::common::BackToLanding;

pub fn Login() -> Element {
    info!("Login Page opened");
    rsx! {
            h3 {"Login Page"},
            BackToLanding{}
}
}