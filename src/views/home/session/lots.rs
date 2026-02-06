use crate::server::SessionState;
use dioxus::prelude::*;

#[component]
pub fn Lots(session: ReadSignal<SessionState>) -> Element {
    let session = session();
    let lots = session.lots.clone();
    rsx! {
        div { class: "section",
            div { class: "title", "Lots"}
            table { class: "table is-bordered is-striped is-hoverable is-narrow",
                thead {
                    tr {
                        th { "Account" }
                        th { "Time" }
                        th { "Product" }
                        th { "Quantity" }
                        th { "Action" }
                    }
                }
                tbody {
                    for lot in lots.iter() {
                        tr {
                            td { "{lot.account}" }
                            td { "{lot.time}" }
                            td { "{lot.product}" }
                            td { "{lot.quantity}" }
                            td { button { class: "button is-primary is-outlined is-small", "Edit" } }
                        }
                    }
                }
            }
            button { class: "button is-primary is-small is-outlined",
                "Add"
            }
        }
    }
}
