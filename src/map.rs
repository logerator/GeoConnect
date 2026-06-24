use dioxus::prelude::*;
use crate::regions::HokkaidoBtn;
use crate::regions::TohokuBtn;
use crate::regions::ChubuBtn;
use crate::regions::KantoBtn;
use crate::regions::KansaiBtn;
use crate::regions::ChugokuBtn;
use crate::regions::ShikokuBtn;
use crate::regions::KyushuBtn;

#[component]
pub fn JapanMap() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen",

            h1 { class: "text-4xl font-bold mb-4", "Japan Regions Map" }

            p { class: "mb-4 text-gray-600", "Click a region to learn more." }

            div { class: "map-container",
                img {
                    src: asset!("/assets/japan.svg"),
                    width: "700px",
                    alt: "Map of Japan regions",
                }
                div { class: "region-buttons",
                    HokkaidoBtn {}
                    TohokuBtn {}
                    ChubuBtn {}
                    KantoBtn {}
                    KansaiBtn {}
                    ChugokuBtn {}
                    ShikokuBtn {}
                    KyushuBtn {}
                }
            }
        }
    }
}