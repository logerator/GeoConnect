use dioxus::prelude::*;

pub fn JapanMap() -> Element {
    rsx! {
        div {
            class: "p-8",

            h1 {
                class: "text-4xl font-bold mb-4",
                "Japan Regions Map"
            }

            p {
                class: "mb-4 text-gray-600",
                "Click a region to learn more."
            }

            img {
                src: asset!("/assets/japan.svg"),
                width: "700px",
                alt: "Map of Japan regions"
            }
        }
    }
}