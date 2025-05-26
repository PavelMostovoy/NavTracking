use dioxus::html::completions::CompleteWithBraces::option;
use dioxus::prelude::*;
use crate::components::{date_selector::DateSelector, dropdown::DropdownSelector};

#[component]
pub(crate) fn Selector(id: i32) -> Element {
    rsx! {
        div {
            id: "selector",
            h3 { "Tracker and Data selection" }
            div {
                DropdownSelector{ index: 0}
                DropdownSelector{ index: 1}
            }
            div{
            }
            div{
                DateSelector {}
            }
            button {
                onclick: move |_| {},
                "Online Monitoring"
            }
        }
    }
}
