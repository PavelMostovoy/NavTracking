#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::pages::common::BackToLanding;

pub fn Map() -> Element {
    info!("Map page opened");
    rsx! {
        h2{
            "Map Page"
        },
        BackToLanding{}
    }
}