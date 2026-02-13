use dioxus::prelude::*;

#[component]
pub fn ProgressIndicator(title: String, progress: usize, total: usize) -> Element {
    let reach = total - progress;
    let progress_portion = ((progress as f32 / total as f32) * 100.0).round() as usize;
    let reach_portion = 100 - progress_portion;
    rsx! {
        div { class: "field",
            div { class: "label is-small", "{title}" }
            div { class: "level mb-1",
                div { class: "level-left",
                    div { class: "level-item",
                        span { class: "title is-7", "{progress_portion}%"}
                    }
                }
                div { class: "level-right",
                    div { class: "level-item",
                        span { class: "subtitle is-7", "{reach_portion}%"}
                    }
                }
            }
            progress { class: "progress is-small is-info mb-1", value: "{progress}", max: "{total}"}
            div { class: "level mt-1",
                div { class: "level-left",
                    div { class: "level-item",
                        span { class: "title is-7", "{progress}"}
                    }
                }
                div { class: "level-right",
                    div { class: "level-item",
                        span { class: "subtitle is-7", "{reach}"}
                    }
                }
            }
        }
    }
}
