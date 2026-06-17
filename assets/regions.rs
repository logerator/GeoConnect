use dioxus::prelude::*;

#[component]
fn HokkaidoBtn() -> Element {
    let on_click = move || {
        // Handle click event, e.g., display a popup for Hokkaido information
    };

    rsx! {
        button {
            class: "hokkaido-btn",
            "Hokkaido",
            onclick: on_click,
        }
    }
}