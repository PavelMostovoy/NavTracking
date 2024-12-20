#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::pages::common::{BackToLanding, UserSharedStatus};


#[component]
pub fn Details() -> Element {
    let context = use_context::<Signal<UserSharedStatus>>();
    info!("Details page opened");
    rsx! {
        h3{
            "Details Page",
            p{"Username :  {context().username}"},
            p{"Token :  {context().token}"}
        },
        BackToLanding{}
    }
}