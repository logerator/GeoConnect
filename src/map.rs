use dioxus::prelude::*;

use crate::Route;

const JAPAN_SVG: Asset = asset!("/assets/japan.svg");

#[component]
pub fn JapanMap() -> Element {
    rsx! {
        div {
            class: "japan-map-content",

            h1 {
                class: "map-title",
                "Japan Regions Map"
            }

            p {
                class: "map-subtitle",
                "Click a region to learn more"
            }

            div {
                class: "map-container",

                div {
                    class: "map-image-wrapper",

                    img {
                        class: "japan-map-img",
                        src: JAPAN_SVG,
                        alt: "Japan regions map",
                    }

                    Link {
                        class: "map-hotspot hokkaido-hotspot",
                        to: Route::Hokkaido {},
                        "Hokkaido"
                    }

                    Link {
                        class: "map-hotspot tohoku-hotspot",
                        to: Route::Tohoku {},
                        "Tohoku"
                    }

                    Link {
                        class: "map-hotspot chubu-hotspot",
                        to: Route::Chubu {},
                        "Chubu"
                    }

                    Link {
                        class: "map-hotspot kanto-hotspot",
                        to: Route::Kanto {},
                        "Kanto"
                    }

                    Link {
                        class: "map-hotspot kansai-hotspot",
                        to: Route::Kansai {},
                        "Kansai"
                    }

                    Link {
                        class: "map-hotspot chugoku-hotspot",
                        to: Route::Chugoku {},
                        "Chugoku"
                    }

                    Link {
                        class: "map-hotspot shikoku-hotspot",
                        to: Route::Shikoku {},
                        "Shikoku"
                    }

                    Link {
                        class: "map-hotspot kyushu-hotspot",
                        to: Route::Kyushu {},
                        "Kyushu"
                    }
                }
            }
        }
    }
}