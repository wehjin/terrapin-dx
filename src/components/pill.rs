use crate::bulma::BulmaColor;
use dioxus::prelude::*;

#[component]
pub fn LabelPill(label: String, value: String, color: BulmaColor, tail: Option<String>) -> Element {
    let color_class = if let BulmaColor::Light = color {
        "is-light has-text-black-ter"
    } else {
        color.class()
    };
    match tail {
        None => rsx!(
            div { class: "tags has-addons mb-2 is-flex-wrap-nowrap",
                span { class: "tag is-dark", "{label}" }
                span { class: "tag", class: "{color_class}", "{value}" }
            }
        ),
        Some(tail) => rsx!(
            div { class: "tags has-addons mb-2 is-flex-wrap-nowrap",
                span { class: "tag", class: "{color_class}", "{label}" }
                span { class: "tag is-dark", "{value}" }
                span { class: "tag", class: "{color_class}",  "{tail}" }
            }
        ),
    }
}
