use dioxus::prelude::*;

#[derive(Debug, Clone, sqlx::FromRow)]
struct Fact {
    title: String,
    body: String,
}

#[component]
pub fn Kyushu() -> Element {
    let mut selected = use_signal(|| Option::<Fact>::None);

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

    rsx! {
        div {
            h1 { "Kyushu " }

            if let Some(facts) = facts() {
                for fact in facts {
                    button {
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
                    h2 { "{fact.title}" }
                    for step in fact.body.split(" | ") {
                        div { "{step}" }
                    }
                }
            }
        }
    }
}
