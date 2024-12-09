#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};
use crate::pages::{Login,Map,Details,Selection};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    LandingPage {},
    #[route("/login")]
    Login {},
    #[route("/selection")]
    Selection {},
    #[route("/details")]
    Details {},
    #[route("/map")]
    Map {},
}

#[component]
pub fn BackToLanding() -> Element {
    let nav = use_navigator();
    rsx! {
            button {
            onclick: move |event| {
                info!("Landing : {event:?}");
                nav.replace(Route::LandingPage {});
            },
            "Back to Landing"
        }
}
}

fn LandingPage() -> Element {
    rsx! {
        h1 {"Landing Page"},
                nav {
            ul {
                li {
                    Link { to: Route::Login {}, "Login" }
                },
                li {
                    Link { to: Route::Details {}, "Details" }
                },
                li {
                    Link { to: Route::Selection {}, "Selection" }
                },
                li {
                    Link { to: Route::Map {}, "Map" }
                },
            }
        }
    }
}