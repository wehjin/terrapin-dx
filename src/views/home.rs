use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "title", "Yo!" }
            }
        }
    }
}
