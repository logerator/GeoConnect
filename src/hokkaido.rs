use dioxus::prelude::*;
use crate::flowchart::FlowChart;
use crate::place_nav::PlaceNav;

#[derive(Debug, Clone, sqlx::FromRow)]
struct Fact {
    title: String,
    body: String,
}

#[component]
pub fn Hokkaido() -> Element {
    let mut selected = use_signal(|| Option::<Fact>::None);

    let facts = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<Fact> = sqlx::query_as(
            "SELECT title, body FROM facts WHERE region_id = 1"
        )
        .fetch_all(pool)
        .await
        .expect("facts query failed");
        result
    });

    rsx! {
        div {
            class: "region-page",
            h1 { "Hokkaido " }

            PlaceNav { region_id: 1 }

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
                    FlowChart { key: "{fact.title}", steps: fact.body.split(" | ").map(|s| s.to_string()).collect::<Vec<String>>() }
                }
            }
        }
    }
}
