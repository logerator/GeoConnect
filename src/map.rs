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
        div { class: "japan-map-content",

            h1 { class: "map-title",
                "Japan Regions Map"
            }

            p { class: "map-subtitle",
                "Click a region to learn more"
            }

            div { class: "map-container",
                img {
                    src: asset!("/assets/japan.svg"),
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