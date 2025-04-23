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
            // img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://mostovoi.org", "📚 Homepage" },
            }
            button {
            onclick: move |event| {
                info!("Landing : {event:?}");
                nav.replace( Route::MyMap {id: 0});
            },
            "Back to Landing"
        }

        }
    }
}


