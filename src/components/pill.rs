use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulmaColor {
    Link,
    Primary,
    Info,
    Success,
    Warning,
    Danger,
    Light,
}

impl BulmaColor {
    pub fn class(&self) -> &'static str {
        match self {
            BulmaColor::Link => "is-link",
            BulmaColor::Primary => "is-primary",
            BulmaColor::Info => "is-info",
            BulmaColor::Success => "is-success",
            BulmaColor::Warning => "is-warning",
            BulmaColor::Danger => "is-danger",
            BulmaColor::Light => "is-light",
        }
    }
}

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
