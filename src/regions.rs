use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn HokkaidoBtn() -> Element {
    let nav = use_navigator();
    rsx! {
        button {
            class: "hokkaido-btn",
            onclick: move |_| { nav.push(Route::Hokkaido {}); },

            "Hokkaido"
        }
    }
}

#[component]
pub fn TohokuBtn() -> Element {
    rsx! {
        button {
            class: "tohoku-btn",
            onclick: move |_| {},
            "Tohoku"
        }
    }
}

#[component]
pub fn ChubuBtn() -> Element {
    rsx! {
        button {
            class: "chubu-btn",
            onclick: move |_| {},
            "Chubu"
        }
    }
}

#[component]
pub fn KantoBtn() -> Element {
    rsx! {
        button {
            class: "kanto-btn",
            onclick: move |_| {},
            "Kanto"
        }
    }
}

#[component]
pub fn KansaiBtn() -> Element {
    rsx! {
        button {
            class: "kansai-btn",
            onclick: move |_| {},
            "Kansai"
        }
    }
}


#[component]
pub fn ChugokuBtn() -> Element {
    rsx! {
        button {
            class: "chugoku-btn",
            onclick: move |_| {},
            "Chugoku"
        }
    }
}

#[component]
pub fn ShikokuBtn() -> Element {
    rsx! {
        button {
            class: "shikoku-btn",
            onclick: move |_| {},
            "Shikoku"
        }
    }
}


#[component]
pub fn KyushuBtn() -> Element {
    rsx! {
        button {
            class: "kyushu-btn",
            onclick: move |_| {},
            "Kyushu"
        }
    }
}




