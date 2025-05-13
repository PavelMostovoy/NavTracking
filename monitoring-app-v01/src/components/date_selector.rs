use dioxus::prelude::*;
use chrono::NaiveDate;
use crate::SelectedDate;

#[component]
pub fn DateSelector() -> Element {
    let mut selected_date = use_context::<Signal<SelectedDate>>();

    rsx! {
        div {
            label { "Select a date: " }
            input {
                r#type: "date",
                value: "{selected_date.read().date}",
                onchange: move |evt| {
                    if let Ok(date) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
                        selected_date.set(SelectedDate { date });
                    }
                }
            }
        }
    }
}

