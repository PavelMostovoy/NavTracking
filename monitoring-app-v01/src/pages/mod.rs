use dioxus::prelude::Routable;
use dioxus::prelude::*;


mod map;
pub(crate) mod selector;
mod home;

use crate::pages::{map::MyMap, selector::Selector, home::Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate)enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/map/:id")]
    MyMap { id: i32 },
    #[route("/selector/:id")]
    Selector { id: i32 },
}

#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::MyMap { id: 1 },
                "Map"
            }
            Link {
                to: Route::Selector { id: 1 },
                "Selector"
            }
        }

        Outlet::<Route> {}
    }
}