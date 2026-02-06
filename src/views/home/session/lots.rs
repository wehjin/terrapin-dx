use crate::server::SessionState;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct Editor {
    pub account: String,
    pub time: String,
    pub products: Vec<String>,
    pub product: String,
    pub quantity: String,
}
impl Editor {
    pub fn new(products: Vec<String>) -> Self {
        let product = products[0].clone();
        Self {
            account: "".to_string(),
            time: "".to_string(),
            products,
            product,
            quantity: "".to_string(),
        }
    }
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
            on_edit: move |_| {
                let mut products = session()
                    .products
                    .iter()
                    .map(|product| product.symbol().to_string())
                    .collect::<Vec<_>>();
                products.sort();
                let editor = Editor::new(products);
                editor_signal.set(Some(editor))
            }
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
    let mut account_state = use_signal(|| editor.account);
    let mut time_state = use_signal(|| editor.time);
    let mut product_state = use_signal(|| editor.product);
    let mut quantity_state = use_signal(|| editor.quantity);
    let mut valid = use_signal(|| "".to_string());
    let products = editor.products.clone();
    use_effect(move || {
        let product = product_state();
        let account = account_state();
        let time = time_state();
        let quantity = quantity_state();
        let desc = format!(
            "Account: {}, Time: {}, Product: {}, Quantity: {}",
            account, time, product, quantity
        );
        valid.set(desc);
    });
    let products = editor.products.clone();
    rsx! {
        div { class: "modal is-active",
            div { class: "modal-background" }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "Edit Lot" }
                    button { class: "delete", aria_label: "close", onclick: move |_| on_end.call(Ending::Cancel) }
                }
                section { class: "modal-card-body",
                    p { class: "has-text-grey", "{valid}" }
                    div { class: "field",
                        label { class: "label", "Account" }
                        div { class: "control",
                            input { class:"input", type: "text", oninput: move |e| account_state.set(e.value()) }
                        }
                    }
                    div { class: "field",
                        label { class: "label", "Time" }
                        div { class: "control",
                            input { class:"input", type: "text", oninput: move |e| time_state.set(e.value()) }
                        }
                    }
                    div { class: "columns",
                        div { class: "field column is-half",
                            label { class: "label", "Product" }
                            div { class: "control",
                                div { class: "select is-fullwidth",
                                    select {
                                        onchange: move |e| product_state.set(e.value().clone()),
                                        for product in products.iter() {
                                            option {
                                                value: "{product}",
                                                selected: product == &product_state(),
                                                "{product}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "field column is-half",
                            label { class: "label", "Quantity" }
                            div { class: "control",
                                input { class:"input", type: "number", oninput: move |e| quantity_state.set(e.value()) }
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
