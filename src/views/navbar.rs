use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-menu",
                div { class: "navbar-start",
                    Link {
                        class: "navbar-item",
                        to: Route::Home {},
                        "Home"
                    }
                    Link {
                        class: "navbar-item",
                        to: Route::Dev {},
                        "Lab"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
