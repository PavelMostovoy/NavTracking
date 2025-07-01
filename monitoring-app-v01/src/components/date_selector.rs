use dioxus::prelude::*;
use chrono::NaiveDate;
use dioxus_logger::tracing;
use crate::SelectedDate;

#[component]
pub fn DateSelector() -> Element {
    let mut selected_date = use_context::<Signal<SelectedDate>>();
    let current_date = selected_date.read().date.format("%Y-%m-%d").to_string();
    let formatted_display_date = selected_date.read().date.format("%B %d, %Y").to_string();

    rsx! {
        div {
            class: "date-selector-container",
            label { 
                class: "date-label",
                "Select Date" 
            }
            // Display the formatted date for visual feedback
            div {
                class: "date-display",
                span { class: "selected-date", "{formatted_display_date}" }
            }
            // Make the date input directly visible and functional
            input {
                r#type: "date",
                value: "{current_date}",
                style: "width: 100%; padding: 0.8rem 1rem; margin-top: 0.5rem; background-color: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 8px; color: var(--text-color);",
                onchange: move |evt| {
                    if let Ok(date) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
                        selected_date.set(SelectedDate { date });
                    } else {
                        // Provide visual feedback for invalid date
                        tracing::warn!("Invalid date format: {}", evt.value());
                    }
                }
            }
            div {
                class: "date-helper-text",
                "Historical data will be loaded for this date"
            }
        }
    }
}
