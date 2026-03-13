use crate::api::import_portfolio_csv;
use dioxus::prelude::*;

#[component]
pub fn ImportPrices() -> Element {
    let mut status = use_signal::<String>(|| "Ready".to_string());

    rsx! {
        h1 { class: "title", "Import Prices"}
        h2 { class: "subtitle", "Update prices from portfolio CSV"}

        article { class: "message is-info",
            div { class: "message-body", "{status()}" }
        }
        div { class: "file",
            label { class: "file-label",
                input { class: "file-input",
                    type: "file",
                    accept: "text/csv",
                    multiple: false,
                    onchange: move |e| {
                        async move {
                            let file_data = e.files();
                            if let Some(file) = file_data.first() {
                                if let Ok(file_content) = file.read_string().await {
                                    let file_name = file.name();
                                    status.set("Importing…".to_string());
                                    let import_result = import_portfolio_csv(file_content).await;
                                    match import_result {
                                        Ok(_) => {
                                            status.set(format!("Done importing '{}'", file_name));
                                        },
                                        Err(e) => {
                                            status.set(format!("Failed to import '{}': {}", file_name, e));
                                        },
                                    };
                                }
                            }
                        }
                    }
                }
                span { class: "file-cta",
                    span { class: "file-icon", i { class: "fas fa-upload" } }
                    span { class: "file-label", "Portfolio CSV…"}
                }
            }
        }
    }
}
