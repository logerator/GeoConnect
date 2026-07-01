use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn HokkaidoBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "hokkaido-btn",
            onclick: move |_| {
                nav.push(Route::Hokkaido {});
            },

            "Hokkaido"
        }
    }
}

#[component]
pub fn TohokuBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "tohoku-btn",
            onclick: move |_| {
                nav.push(Route::Tohoku {});
            },
            "Tohoku"
        }
    }
}

#[component]
pub fn ChubuBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "chubu-btn",
            onclick: move |_| {
                nav.push(Route::Chubu {});
            },
            "Chubu"
        }
    }
}

#[component]
pub fn KantoBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "kanto-btn",
            onclick: move |_| {
                nav.push(Route::Kanto {});
            },
            "Kanto"
        }
    }
}

#[component]
pub fn KansaiBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "kansai-btn",
            onclick: move |_| {
                nav.push(Route::Kansai {});
            },
            "Kansai"
        }
    }
}


#[component]
pub fn ChugokuBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "chugoku-btn",
            onclick: move |_| {
                nav.push(Route::Chugoku {});
            },
            "Chugoku"
        }
    }
}

#[component]
pub fn ShikokuBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "shikoku-btn",
            onclick: move |_| {
                nav.push(Route::Shikoku {});
            },
            "Shikoku"
        }
    }
}


#[component]
pub fn KyushuBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "kyushu-btn",
            onclick: move |_| {
                nav.push(Route::Kyushu {});
            },
            "Kyushu"
        }
    }
}