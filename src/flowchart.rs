use dioxus::prelude::*;

const READ_MORE_THRESHOLD: usize = 80;

#[component]
pub fn FlowChart(steps: Vec<String>) -> Element {
    let step_count = steps.len();
    let mut expanded = use_signal(move || vec![false; step_count]);

    rsx! {
        div {
            class: "flowchart",
            for (i, step) in steps.into_iter().enumerate() {
                div {
                    class: "flow-item",
                    key: "{i}",

                    div {
                        class: if i == 0 {
                            "flow-node flow-node--origin"
                        } else if i + 1 == step_count {
                            "flow-node flow-node--outcome"
                        } else {
                            "flow-node"
                        },

                        if i == 0 {
                            span { class: "flow-node-label", "Origin" }
                        } else if i + 1 == step_count {
                            span { class: "flow-node-label", "Today" }
                        }

                        if step.chars().count() > READ_MORE_THRESHOLD && !expanded()[i] {
                            p {
                                class: "flow-node-text",
                                "{step.chars().take(READ_MORE_THRESHOLD).collect::<String>()}…"
                            }
                            button {
                                class: "flow-toggle",
                                onclick: move |_| {
                                    let mut e = expanded();
                                    e[i] = true;
                                    expanded.set(e);
                                },
                                "Read more"
                            }
                        } else if step.chars().count() > READ_MORE_THRESHOLD {
                            p { class: "flow-node-text", "{step}" }
                            button {
                                class: "flow-toggle",
                                onclick: move |_| {
                                    let mut e = expanded();
                                    e[i] = false;
                                    expanded.set(e);
                                },
                                "Show less"
                            }
                        } else {
                            p { class: "flow-node-text", "{step}" }
                        }
                    }

                    if i + 1 < step_count {
                        div { class: "flow-arrow", "\u{2192}" }
                    }
                }
            }
        }
    }
}
