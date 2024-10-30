use dioxus::prelude::*;
use crate::components::control::Control;

pub fn MainContent() -> Element {
    rsx! {
        div { 
            class: "px-4 py-6",
            
            h2 { 
                class: "text-lg mb-4",
                "Bande lumineuse colorée à 3 fils" 
            }

            div { 
                class: "bg-gray-800/50 rounded-lg p-4",
                div { 
                    class: "flex items-center justify-between",
                    
                    div { 
                        class: "flex items-center gap-3",
                        div { 
                            class: "w-8 h-8 flex items-center justify-center",
                            svg {
                                class: "w-6 h-6 text-red-400",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
                                }
                            }
                        }
                        span { 
                            class: "font-medium",
                            "ELK-BLEDOB" 
                        }
                    }

                    div { 
                        class: "flex gap-2",
                        button { 
                            class: "p-2 rounded-full bg-gray-700/50",
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
                                }
                            }
                        }
                        button { 
                            class: "p-2 rounded-full bg-gray-700/50",
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M13 10V3L4 14h7v7l9-11h-7z"
                                }
                            }
                        }
                    }
                }
            }

            Control {}
        }
    }
}