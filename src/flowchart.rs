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

/// Reads index `i`, defaulting to collapsed (`false`) if the signal's vector
/// hasn't caught up to the current step count yet — this can happen for a
/// render or two right after `steps` changes, since Dioxus may reuse this
/// component's signal state across different `steps` props. Never panics.
fn is_expanded(expanded: Signal<Vec<bool>>, i: usize) -> bool {
    expanded.read().get(i).copied().unwrap_or(false)
}

/// Sets index `i`, growing the vector first if needed. Never panics.
fn set_expanded(mut expanded: Signal<Vec<bool>>, i: usize, value: bool) {
    let mut e = expanded();
    if e.len() <= i {
        e.resize(i + 1, false);
    }
    e[i] = value;
    expanded.set(e);
}

#[component]
pub fn FlowChart(steps: Vec<String>) -> Element {
    let step_count = steps.len();
    let expanded = use_signal(move || vec![false; step_count]);

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
                            let expanded_now = is_expanded(expanded, i);

                            match content.bullets {
                                Some(bullets) => rsx! {
                                    p { class: "flow-node-text flow-node-summary", "{content.summary}" }
                                    if expanded_now {
                                        ul {
                                            class: "flow-node-bullets",
                                            for (bi, bullet) in bullets.into_iter().enumerate() {
                                                li { key: "{bi}", "{bullet}" }
                                            }
                                        }
                                        button {
                                            class: "flow-toggle",
                                            onclick: move |_| set_expanded(expanded, i, false),
                                            "Show less"
                                        }
                                    } else {
                                        button {
                                            class: "flow-toggle",
                                            onclick: move |_| set_expanded(expanded, i, true),
                                            "Read more"
                                        }
                                    }
                                },
                                None if content.summary.chars().count() > READ_MORE_THRESHOLD => rsx! {
                                    if expanded_now {
                                        p { class: "flow-node-text", "{content.summary}" }
                                        button {
                                            class: "flow-toggle",
                                            onclick: move |_| set_expanded(expanded, i, false),
                                            "Show less"
                                        }
                                    } else {
                                        p {
                                            class: "flow-node-text",
                                            "{content.summary.chars().take(READ_MORE_THRESHOLD).collect::<String>()}…"
                                        }
                                        button {
                                            class: "flow-toggle",
                                            onclick: move |_| set_expanded(expanded, i, true),
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
