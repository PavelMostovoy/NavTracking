#![allow(non_snake_case)]

use dioxus::html::r#use;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, warn, Level};
use gloo_utils::document;
use leaflet::{Map, MapOptions, Marker, TileLayer};
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

#[derive(Clone)]
struct MyContext {
    info: String,
}

#[derive(Props, Clone, PartialEq)]
struct MapProps {
    text: String,
    #[props(optional)]
    size: i32,
}
impl MapProps {
    fn text(&mut self) {
        self.text = "Updated Button".to_string();
    }
}


#[component]
fn my_button(mut props: MapProps) -> Element {
    let context = use_context::<Signal<MyContext>>();
    let mut count = use_signal(|| 0);
    let mut value = use_signal(|| props.text.clone());
    if props.size > 0 {
        rsx! {
       button{
                onclick : move |event|{
                    info!("Button clicked");
                },
                "{props.text} # {props.size}"}
   }
    } else {
        rsx! {
       button{onclick : move |_|{
                count += 1;
                props.text();
                value.set(props.text.clone());
        },
                "{value} : {count} {context().info}"
        }
   }
    }
}


#[component]
fn MyMap() -> Element {
    use_effect(move || {
        const GREENWICH_MERIDIAN: (f64, f64) = (42.477806, 3.101472);

        let container: HtmlElement = document()
            .get_element_by_id("map")
            .unwrap()
            .dyn_into()
            .unwrap();

        let map = Map::new_with_element(&container, &MapOptions::default());

        TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);

        map.set_view(&GREENWICH_MERIDIAN.into(), 14.0);

        Marker::new(&GREENWICH_MERIDIAN.into()).add_to(&map);
    });

    rsx! {
        div {
            "style": "width:200px; height:200px;",
            id: "map"
        }
    }
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(MyContext { info: "default text".to_string() }));
    rsx! {
        div {
            "style": "width:100px; height:100px; background-color: red",
            p{"inner"}
        }
        button {onclick: move |_| {}, "Button one"},
        // MyMap{}
        p{"Hi There !"}
        my_button{text: "hello"}
        my_button{text: "another", size:16}
    }
}
