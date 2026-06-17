// Main file (This is a test) 

use dioxus::prelude::*;
mod map;
use map::JapanMap;

static DB_POOL: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/map/:id")]
    Map { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set in .env");

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Database connection failed");

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Test query failed");
    println!("Test query returned: {}", row.0);

    DB_POOL.set(pool).expect("Pool already initialized");
    println!("Connected to database!");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            h1 { "🌍 GeoConnect" }

            div { id: "links",
                a { "📚 Learn the WHY in Japanese culture" }
                a { "👺 Explore different Region's Distinct Culture" }
                a { "👹 Prepare for Life in Japan" }
                a { "🎌 Minimize Culture Shock" }

                Link { to: Route::Map { id: 1 }, "Go to the Map" }
            
            }
        }
    }
}





/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Map(id: i32) -> Element {
    rsx! {
        div { id: "Map",

            // Content
            h1 { "This is the map of Japan!" }
            p { "Click on a region to discover how geography shapes the culture in this region." }
            JapanMap {}
        
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
        
        }

        Outlet::<Route> {}
    }
}
