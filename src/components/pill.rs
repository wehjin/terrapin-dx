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
pub fn LabelPill(label: String, value: String, color: BulmaColor) -> Element {
    rsx! {
        div { class: "tags has-addons mb-2",
            span { class: "tag is-dark", "{label}" }
            span { class: "tag", class: "{color.class()}", "{value}" }
        }
    }
}
