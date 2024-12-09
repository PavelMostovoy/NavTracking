#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::pages::common::BackToLanding;

pub fn Selection() -> Element {
    info!("Selection page opened");
    rsx! {
        h3{
            "Selection Page"
        },
        BackToLanding{}
    }
}
