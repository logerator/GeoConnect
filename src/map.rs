use dioxus::prelude::*;

pub fn JapanMap() -> Element {
    rsx! {
        div {
            id: "japan-map-section",

            h1 {
                "Japan Regions Map"
            }

            p {
                "Click a region to learn more."
            }

            div {
                id: "japan-map-wrapper",

                img {
                    src: asset!("/assets/japan.svg"),
                    id: "japan-map-img",
                    alt: "Map of Japan regions"
                }
            }
        }
    }
}