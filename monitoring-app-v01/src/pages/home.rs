use dioxus::dioxus_core::Element;
use dioxus::prelude::use_navigator;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::pages::Route;

/// Home page
#[component]
pub(crate) fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    let nav = use_navigator();
    rsx! {
        div {
            id: "hero",
            div { 
                class: "hero-content",
                h1 { "Navigation Tracking System" }
                p { "An application for real-time tracking and monitoring of navigation data." }

                div { 
                    class: "hero-buttons",
                    button {
                        class: "primary-button",
                        onclick: move |_| {
                            nav.replace(Route::LiveMap {});
                        },
                        "Live Tracking"
                    }
                    button {
                        class: "secondary-button",
                        onclick: move |_| {
                            nav.replace(Route::Selector { id: 1 });
                        },
                        "Select Trackers"
                    }
                }
            }

            div { 
                id: "links",
                h3 { "Quick Links" }
                a { 
                    href: "https://mostovoi.org", 
                    span { class: "icon", "🌐" }
                    span { "Homepage" }
                }
                a { 
                    onclick: move |event| {
                        info!("Map clicked: {event:?}");
                        nav.replace(Route::MyMap {id: 0});
                    },
                    span { class: "icon", "🗺️" }
                    span { "View Map" }
                }
                a { 
                    onclick: move |_| {
                        nav.replace(Route::LiveMap {});
                    },
                    span { class: "icon", "📡" }
                    span { "Live Data" }
                }
            }

            div {
                class: "version-info",
                p { "Version 1.0 | © 2023 Navigation Tracking" }
            }
        }
    }
}
