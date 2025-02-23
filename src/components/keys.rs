use dioxus::prelude::*;

#[component]
pub fn Keys(input: Signal<String>) -> Element {
    rsx! {
        for k in 0..10 {
            button {
                class: "calculator-numbers-keys",
                name: "key-{k}",
                onclick: move |_| input.write().push_str(format!("{k}").as_str()),
                "{k}"
            }
        }
        OpsKeys {input}
    }
}

#[component]
fn OpsKeys(input: Signal<String>) -> Element {
    rsx! {
        button {
            class: "calculator-keys",
            name: "key-plus",
            onclick: move |_| input.write().push_str(" + "),
            "+"
        }
        button {
            class: "calculator-keys",
            name: "key-minus",
            onclick: move |_| input.write().push_str(" - "),
            "-",
        }

        button {
            class: "calculator-keys",
            name: "key-multiplication",
            onclick: move |_| input.write().push_str(" * "),
            "*",
        }

        button {
            class: "calculator-keys",
            name: "key-division",
            onclick: move |_| input.write().push_str(" / "),
            "/",
        }
        button {
            class: "calculator-keys",
            name: "key-division",
            onclick: move |_| input.write().push_str("."),
            "."
        }
        button {
            class: "calculator-keys",
            name: "key-calculate",
            onclick: move |_| input.set(crate::parsing::compare_input(input)),
            " = "
        }
        button {
            class: "calculator-keys",
            name: "key-clear",
            onclick: move |_| input.write().clear(),
            "C"
        }
    }
}
