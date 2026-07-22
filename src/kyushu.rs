use dioxus::prelude::*;
use crate::flowchart::FlowChart;

#[derive(Debug, Clone, sqlx::FromRow)]
struct Fact {
    title: String,
    body: String,
}

#[component]
pub fn Kyushu() -> Element {
    let mut selected = use_signal(|| Option::<Fact>::None);
    let mut show_okinawa = use_signal(|| false);
    let mut okinawa_selected = use_signal(|| Option::<Fact>::None);

    let facts = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<Fact> = sqlx::query_as(
            "SELECT title, body FROM facts WHERE region_id = 8"
        )
        .fetch_all(pool)
        .await
        .expect("facts query failed");
        result
    });

    let okinawa_facts = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<Fact> = sqlx::query_as(
            "SELECT title, body FROM facts WHERE region_id = 9"
        )
        .fetch_all(pool)
        .await
        .expect("facts query failed");
        result
    });

    rsx! {
        div {
            class: "region-page",
            h1 { "Kyushu " }

            if let Some(facts) = facts() {
                for fact in facts {
                    button {
                        class: "fact-btn",
                        onclick: {
                            let fact = fact.clone();
                            move |_| selected.set(Some(fact.clone()))
                        },
                        "{fact.title}"
                    }
                }
            }

            if let Some(fact) = selected() {
                div {
                    class: "fact-detail",
                    h2 { "{fact.title}" }
                    FlowChart { steps: fact.body.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>() }
                }
            }

            div {
                class: "okinawa-callout",
                button {
                    class: "okinawa-btn",
                    onclick: move |_| show_okinawa.toggle(),
                    span { class: "okinawa-btn-icon", "🌊" }
                    "Okinawa"
                }
                p {
                    class: "okinawa-blurb",
                    "Okinawa is often grouped with Kyushu, but its island history gave it a culture all its own — click to see how it differs."
                }
            }

            if show_okinawa() {
                div {
                    class: "okinawa-section",
                    h2 { "Okinawa" }

                    if let Some(facts) = okinawa_facts() {
                        for fact in facts {
                            button {
                                class: "fact-btn okinawa-fact-btn",
                                onclick: {
                                    let fact = fact.clone();
                                    move |_| okinawa_selected.set(Some(fact.clone()))
                                },
                                "{fact.title}"
                            }
                        }
                    }

                    if let Some(fact) = okinawa_selected() {
                        div {
                            class: "fact-detail",
                            h2 { "{fact.title}" }
                            FlowChart { steps: fact.body.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>() }
                        }
                    }
                }
            }
        }
    }
}
