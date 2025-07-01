use dioxus::prelude::*;
use crate::components::{date_selector::DateSelector, dropdown::DropdownSelector};
use crate::pages::Route;

#[component]
pub(crate) fn Selector(id: i32) -> Element {
    let nav = use_navigator();
    
    rsx! {
        div {
            class: "container",
            div {
                id: "selector",
                div {
                    class: "selector-header",
                    h2 { "Tracker Configuration" }
                    p { "Select trackers and date range to view historical data" }
                }

                div {
                    class: "selector-grid",
                    div {
                        class: "selector-card",
                        h3 { "Primary Tracker" }
                        p { class: "card-description", "Select the main tracker to follow" }
                        DropdownSelector{ index: 0 }
                    }

                    div {
                        class: "selector-card",
                        h3 { "Secondary Tracker" }
                        p { class: "card-description", "Select an additional tracker to compare" }
                        DropdownSelector{ index: 1 }
                    }
                }

                div {
                    class: "selector-card date-card",
                    h3 { "Date Selection" }
                    p { class: "card-description", "Choose a date to view historical data" }
                    DateSelector {}
                }

                div {
                    class: "selector-actions",
                    button {
                        class: "primary-button",
                        onclick: move |_| {
                            // Navigate immediately
                            nav.push(Route::MyMap { id: 1 });
                        },
                        "View Historical Data"
                    }

                    button {
                        class: "secondary-button",
                        onclick: move |_| {
                            // Navigate immediately
                            nav.push(Route::LiveMap {});
                        },
                        "Live Monitoring"
                    }
                }

            }
        }
    }
}
