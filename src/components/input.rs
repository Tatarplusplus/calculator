use crate::components::Keys;
use dioxus::prelude::*;

#[component]
pub fn InputLog() -> Element {
    let mut input = use_signal(|| "".to_string());
    rsx! {
        div {id: "welcome",
            h1 {"Calculator"},
        }

        div { id: "input-log",
            input {
                value: "{input}",
                padding: "100px",
                font_size: "64px",
                oninput: move |event| if event.value().contains(" =") {
                    input.set(crate::parsing::compare_input(input)) } else {input.set(event.value())}
                }
            }
        Keys {input}
    }
}
