use dioxus::prelude::*;
use crate::flowchart::FlowChart;

#[derive(Debug, Clone, sqlx::FromRow)]
struct Fact {
    title: String,
    body: String,
}

#[component]
pub fn Shikoku() -> Element {
    let mut selected = use_signal(|| Option::<Fact>::None);

    let facts = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<Fact> = sqlx::query_as(
            "SELECT title, body FROM facts WHERE region_id = 7"
        )
        .fetch_all(pool)
        .await
        .expect("facts query failed");
        result
    });

    rsx! {
        div {
            class: "region-page",
            h1 { "Shikoku " }

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
        }
    }
}
