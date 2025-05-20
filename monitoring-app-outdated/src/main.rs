#![allow(non_snake_case)]

mod pages;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level};
use pages::common::Route;
use crate::pages::common::UserSharedStatus;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(|| {
        use_context_provider(|| Signal::new(UserSharedStatus {
            username: String::from(""),
            token: String::from(""),
            logged_in: false,
        }));
        rsx! {
        Router::<Route>{}
    }
    })
}











