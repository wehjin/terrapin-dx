use crate::server::SessionState;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct Editor {
    pub account: String,
    pub time: String,
    pub product: String,
    pub quantity: String,
}

#[component]
pub fn Lots(session: ReadSignal<SessionState>) -> Element {
    let mut editor_signal = use_signal(|| None::<Editor>);
    match editor_signal() {
        Some(editor) => rsx!(EditLot {
            editor,
            on_end: move |ending| match ending {
                Ending::Save => editor_signal.set(None),
                Ending::Cancel => editor_signal.set(None),
            }
        }),
        None => rsx!(LotsView {
            session,
            on_edit: move |_| editor_signal.set(Some(Editor::default()))
        }),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Ending {
    Save,
    Cancel,
}

#[component]
fn EditLot(editor: Editor, on_end: EventHandler<Ending>) -> Element {
    rsx! {
        div { class: "modal is-active",
            div { class: "modal-background" }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Edit Lot" }
                    button { class: "delete", aria_label: "close", onclick: move |_| on_end.call(Ending::Cancel) }
                }
                section { class: "modal-card-body",
                    div { class: "field",
                        label { class: "label", "Account" }
                        div { class: "control",
                            input { class:"input", type: "text" }
                        }
                    }
                    div { class: "field",
                        label { class: "label", "Time" }
                        div { class: "control",
                            input { class:"input", type: "text" }
                        }
                    }
                    div { class: "columns",
                        div { class: "field column is-half",
                            label { class: "label", "Product" }
                            div { class: "control",
                                div { class: "select is-fullwidth",
                                    select {
                                        option { "RKLB" }
                                        option { "PL" }
                                    }
                                }
                            }
                        }
                        div { class: "field column is-half",
                            label { class: "label", "Quantity" }
                            div { class: "control",
                                input { class:"input", type: "number" }
                            }
                        }
                    }
                }
                footer { class: "modal-card-foot",
                    div { class: "buttons",
                        button { class: "button is-primary", disabled: true,
                            onclick: move |_| on_end.call(Ending::Save),
                            "Save"
                        }
                        button { class: "button is-light",
                            onclick: move |_| on_end.call(Ending::Cancel),
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LotsView(session: ReadSignal<SessionState>, on_edit: EventHandler<()>) -> Element {
    let session = session();
    rsx! {
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
                for lot in session.lots.iter() {
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
            onclick: move |_| { on_edit.call(())},
            "Add"
        }
    }
}
