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