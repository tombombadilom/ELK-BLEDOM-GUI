use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DeviceProps {
    name: String,
}

#[allow(non_snake_case)]
pub fn App() -> Element {
    let mut current_route = use_signal(|| "home");
    let _selected_color = use_signal(|| "rgb(255, 0, 0)".to_string());
    let _brightness = use_signal(|| 100);

    rsx! {
        div { 
            class: "min-h-screen bg-gradient-to-b from-app-bg-start to-app-bg-end text-white p-4",
            
            {match *current_route.read() {
                "home" => rsx! {
                    div { 
                        class: "p-4",
                        header { 
                            class: "flex justify-between items-center mb-8",
                            button {
                                class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                i { class: "fas fa-cog" }
                            }
                            h1 { 
                                class: "text-xl font-medium",
                                "Mes appareils"
                            }
                            div {
                                class: "flex gap-4",
                                button {
                                    class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                    i { class: "fas fa-th-large" }
                                }
                                button {
                                    class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                    i { class: "fas fa-plus" }
                                }
                            }
                        }

                        div {
                            class: "bg-card hover:bg-card-hover backdrop-blur-xs rounded-lg p-4",
                            onclick: move |_| current_route.set("device"),
                            div {
                                class: "flex items-center gap-4",
                                div {
                                    class: "w-8 h-8 bg-red-500/20 rounded flex items-center justify-center",
                                    i { class: "fas fa-lightbulb text-icon-red" }
                                }
                                span { 
                                    class: "font-medium",
                                    "ELK-BLEDOB"
                                }
                            }
                            
                            div {
                                class: "flex gap-4",
                                button {
                                    class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                    i { class: "fas fa-link" }
                                }
                                button {
                                    class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                    i { class: "fas fa-power-off" }
                                }
                            }
                        }
                    }
                },
                "device" => rsx! {
                    div { 
                        class: "p-4",
                        header { 
                            class: "flex justify-between items-center mb-6",
                            button {
                                class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                onclick: move |_| current_route.set("home"),
                                i { class: "fas fa-chevron-left" }
                            }
                            h1 { 
                                class: "text-xl font-medium",
                                "ELK-BLEDOB"
                            }
                            button {
                                class: "p-2 rounded-full bg-button hover:bg-button-hover",
                                i { class: "fas fa-edit" }
                            }
                        }

                        // Contenu de la page device
                        // ... votre code de la page device ...
                    }
                },
                _ => rsx! { "404 Not Found" }
            }}
        }
    }
}