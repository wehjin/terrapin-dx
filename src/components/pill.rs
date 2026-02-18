use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BulmaColor {
    Link,
    Primary,
    Info,
    Success,
    Warning,
    Danger,
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
        }
    }
}

#[component]
pub fn LabelPill(label: String, value: String, color: BulmaColor, tail: Option<String>) -> Element {
    match tail {
        None => rsx!(
            div { class: "tags has-addons mb-2",
                span { class: "tag is-dark", "{label}" }
                span { class: "tag", class: "{color.class()}", "{value}" }
            }
        ),
        Some(tail) => rsx!(
            div { class: "tags has-addons mb-2",
                span { class: "tag", class: "{color.class()}", "{label}" }
                span { class: "tag is-dark", "{value}" }
                span { class: "tag", class: "{color.class()}",  "{tail}" }
            }
        ),
    }
}
