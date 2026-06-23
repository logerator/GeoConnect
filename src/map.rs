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
