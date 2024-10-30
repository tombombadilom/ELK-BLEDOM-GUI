use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ControlProps {}

#[allow(non_snake_case)]
pub fn Control() -> Element {
    rsx! {
        div { 
            class: "bg-gray-800 p-4 rounded-lg shadow-lg",
            div { class: "space-y-4",
                h2 { class: "text-xl font-bold",
                    "Device Control"
                }
                div { class: "flex flex-col gap-4",
                    // Contrôles à ajouter
                }
            }
        }
    }
}
