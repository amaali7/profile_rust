use dioxus::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> Element {
    rsx! {
        section { id: "Home", class: "page-section home-page",
            div { class: "images-container",
                div { class: "images",
                    div { class: "ferris-wheel",
                        img {
                            class: "personal-image",
                            src: asset!("/images/pi.jpg"),
                            alt: ""
                        }
                        div { class: "wheel",
                            div { class: "cabin cabin-1",
                                img {
                                    src: asset!("/images/Bash.png"),
                                    alt: "Cabin 1"
                                }
                            }
                            div { class: "cabin cabin-2",
                                img {
                                    src: asset!("/images/Tex.png"),
                                    alt: "Cabin 2"
                                }
                            }
                            div { class: "cabin cabin-3",
                                img {
                                    src: asset!("/images/nix.png"),
                                    alt: "Cabin 3"
                                }
                            }
                            div { class: "cabin cabin-4",
                                img {
                                    src: asset!("/images/leptos.png"),
                                    alt: "Cabin 4"
                                }
                            }
                            div { class: "cabin cabin-5",
                                img {
                                    src: asset!("/images/aya.svg"),
                                    alt: "Cabin 5"
                                }
                            }
                            div { class: "cabin cabin-6",
                                img {
                                    src: asset!("/images/C.png"),
                                    alt: "Cabin 6"
                                }
                            }
                            div { class: "cabin cabin-7",
                                img {
                                    src: asset!("/images/rust.png"),
                                    alt: "Cabin 7"
                                }
                            }
                            div { class: "cabin cabin-8",
                                img {
                                    src: asset!("/images/Python.png"),
                                    alt: "Cabin 8"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "text",
                div { class: "text-area",
                    h1 { "Summary" }
                    div {
                        "Backend and embedded systems engineer with strong expertise in Rust (Tokio, Axum), C/C++, Python, database building, and system infrastructure design and integration. Experienced in developing end-to-end IoT and backend architectures, including real-time data ingestion, analytics, and visualization. Skilled in API interfacing, Nix/NixOS, eBPF (Aya), SurrealDB, and embedded firmware development. Capable of designing reliable, high-performance systems that bridge hardware and software."                    }
                    a { href: "#info", "info" }
                }
            }
        }
    }
}
