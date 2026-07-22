use dioxus::prelude::*;

use crate::Route;

#[derive(Debug, Clone, sqlx::FromRow)]
struct PlaceSummary {
    id: i32,
    name: String,
    kind: String,
}

fn icon_for(kind: &str) -> &'static str {
    match kind {
        "City" => "🏙️",
        "National Park" => "🏞️",
        "Island" => "🏝️",
        "Mountain Valley" => "⛰️",
        "Historic Town" => "🏯",
        "Bay" => "🌊",
        "Rural Area" => "🌾",
        "Onsen Town" => "♨️",
        "Historic Village" => "🏘️",
        "Mountain Route" => "🏔️",
        "Mountain Town" => "🌲",
        "Shrine" => "⛩️",
        "Sacred Mountain" => "🗻",
        "Natural Landmark" => "🏜️",
        "Remote Valley" => "🌉",
        "Gorge" => "🌿",
        _ => "📍",
    }
}

#[component]
pub fn PlaceNav(region_id: i32) -> Element {
    let places = use_resource(move || async move {
        let pool = crate::DB_POOL.get().expect("pool not initialized");
        let result: Vec<PlaceSummary> = sqlx::query_as(
            "SELECT id, name, kind FROM places WHERE region_id = $1 ORDER BY id",
        )
        .bind(region_id)
        .fetch_all(pool)
        .await
        .expect("places query failed");
        result
    });

    rsx! {
        if let Some(places) = places() {
            if !places.is_empty() {
                div {
                    class: "place-nav",
                    h2 { class: "place-nav-heading", "Notable Places to Visit" }
                    div {
                        class: "place-nav-list",
                        for place in places {
                            Link {
                                key: "{place.id}",
                                class: "place-nav-btn",
                                to: Route::Place { id: place.id },
                                span { class: "place-nav-icon", "{icon_for(&place.kind)}" }
                                span { "{place.name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
