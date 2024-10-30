use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn StatusBar() -> Element {
    rsx! {
        div { 
            class: "flex justify-between items-center px-4 py-2 text-sm",
            div { "11:52" }
            div { 
                class: "flex items-center gap-2",
                div { "3G" }
                div { "56%" }
            }
        }
    }
}