use dioxus::prelude::*;



#[component]
pub fn Hokkaido() -> Element {
    let mut selected = use_signal(|| Option::<&'static str>::None);



    rsx! {
        div {
            h1 { "Hokkaido " }

            button { onclick: move |_| selected.set(Some("dairy")), "Diary Farming Culture"}
            
            button { onclick: move |_| selected.set(Some("ainu")), "Ainu Revival"}
            
            
            button { onclick: move |_| selected.set(Some("ski")), "Ski & Winter Tourism"}
            
            
            button { onclick: move |_| selected.set(Some("ramen")), "Sapporo Ramen"}
            

            if let Some(topic) = selected() {
                Flowchart { topic }
            }

        }
    }
}


#[component]
fn Flowchart(topic: &'static str) -> Element {
    let steps: &[&str] = match topic {
        "dairy"=> &[
            "Geographic isolation → late integration into Japan",
            "Meiji frontier colonization (1869) → government-sponsored farming",
            "Cold climate unsuitable for rice → cattle & dairy introduced",
            "Hokkaido becomes Japan's breadbasket",
            "Dairy farming becomes core regional identity & industry",
        ],
        "ainu" => &[
            "Geographic isolation → indigenous Ainu people undisturbed",
            "Meiji annexation → forced assimilation, Ainu culture suppressed",
            "Late 20th century indigenous rights movement globally",
            "2019: Ainu recognized as indigenous people by Japanese law",
            "Cultural revival: language, crafts, and ceremonies resurging",
        ],
        // ... other topics
        _ => &[],
    };


 rsx! {
        div {
            for (i, step) in steps.iter().enumerate() {
                div {
                    span { "{step}" }
                    if i < steps.len() - 1 {
                        span { " ↓ " }
                    }
                }
            }
        }
    }
}