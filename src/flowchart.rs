use dioxus::prelude::*;

const READ_MORE_THRESHOLD: usize = 90;

/// A step's raw text can optionally encode a short summary plus a bulleted
/// elaboration, using " :: " to separate them and " • " to separate bullets,
/// e.g. "Summary sentence :: first detail • second detail • third detail".
/// Steps without " :: " are treated as plain text and fall back to
/// character-count truncation instead.
struct StepContent {
    summary: String,
    bullets: Option<Vec<String>>,
}

fn parse_step(step: &str) -> StepContent {
    match step.split_once(" :: ") {
        Some((summary, rest)) => StepContent {
            summary: summary.trim().to_string(),
            bullets: Some(rest.split(" • ").map(|b| b.trim().to_string()).collect()),
        },
        None => StepContent {
            summary: step.to_string(),
            bullets: None,
        },
    }
}

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

                        {
                            let content = parse_step(&step);
                            let is_expanded = expanded()[i];

                            match content.bullets {
                                Some(bullets) => rsx! {
                                    p { class: "flow-node-text flow-node-summary", "{content.summary}" }
                                    if is_expanded {
                                        ul {
                                            class: "flow-node-bullets",
                                            for (bi, bullet) in bullets.into_iter().enumerate() {
                                                li { key: "{bi}", "{bullet}" }
                                            }
                                        }
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
                                        button {
                                            class: "flow-toggle",
                                            onclick: move |_| {
                                                let mut e = expanded();
                                                e[i] = true;
                                                expanded.set(e);
                                            },
                                            "Read more"
                                        }
                                    }
                                },
                                None if content.summary.chars().count() > READ_MORE_THRESHOLD => rsx! {
                                    if is_expanded {
                                        p { class: "flow-node-text", "{content.summary}" }
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
                                        p {
                                            class: "flow-node-text",
                                            "{content.summary.chars().take(READ_MORE_THRESHOLD).collect::<String>()}…"
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
                                    }
                                },
                                None => rsx! {
                                    p { class: "flow-node-text", "{content.summary}" }
                                },
                            }
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
