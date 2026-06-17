
// Main file (This is a test)

use dioxus::prelude::*;
mod map;
mod regions;
use map::JapanMap;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
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

fn main() {
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
        div {
            id: "hero",
             h1 { "🌍 GeoConnect" }

            div { id: "links",
                a { "📚 Learn the WHY in Japanese culture" }
                a { "👺 Explore different Region's Distinct Culture" }
                a { "👹 Prepare for Life in Japan" }
                a { "🎌 Minimize Culture Shock" }


                Link {
                    to: Route::Map { id: 1},
                    "Go to the Map"
                }
         
               
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
        div {
            id: "Map",

            
            JapanMap {}

            
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
           
        }

        Outlet::<Route> {}
    }
}


