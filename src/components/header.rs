use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Header() -> Element {
    rsx! {
        div { 
            class: "flex justify-between items-center px-4 py-3",
            button { 
                class: "p-2 rounded-full bg-gray-800/50",
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                    }
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                    }
                }
            }

            h1 { 
                class: "text-lg font-medium",
                "Mes appareils" 
            }

            div { 
                class: "flex gap-2",
                button { 
                    class: "p-2 rounded-full bg-gray-800/50",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M4 6h16M4 12h16M4 18h16"
                        }
                    }
                }
                button { 
                    class: "p-2 rounded-full bg-gray-800/50",
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M12 4v16m8-8H4"
                        }
                    }
                }
            }
        }
    }
}