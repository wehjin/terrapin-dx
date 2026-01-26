use dioxus::prelude::*;
#[component]
pub fn Login(on_login: EventHandler<String>) -> Element {
    let mut submit_name = use_signal(|| None::<String>);
    rsx! {
        section { class: "section",
            div { class: "container",
                h1 { class: "title", "Login" }
                div { class: "field has-addons",
                    div { class: "control",
                        input { class: "input",
                            type: "text",
                            name: "login-name",
                            placeholder: "Enter user name",
                            oninput: move | evt | {
                                let value = evt.value().to_string();
                                if value.len() > 0 && value.chars().all( | c | c.is_alphanumeric()) {
                                    submit_name.set(Some(value));
                                } else {
                                    submit_name.set(None);
                                }
                            }
                        }
                    }
                    div { class: "control",
                        button { class: "button is-primary", type: "submit", disabled: submit_name().is_none(),
                            onclick: move |_| on_login.call(submit_name().unwrap()),
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
