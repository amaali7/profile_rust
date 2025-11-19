use dioxus::prelude::*;

/// Default Home Page
#[component]
pub fn Skills() -> Element {
    rsx! {
        section { id: "Skills", class: "page-section skills-page",
            div { class: "conta",
                h1 { class: "rounded-header", "Programing" }
                h6 { class: "skill", "Rust" }
                h6 { class: "skill", "C/C++ Language" }
                h6 { class: "skill", "Nix/NixOS" }
                h6 { class: "skill", "Python" }
                h6 { class: "skill", "Sass" }
                h6 { class: "skill", "Bash Script" }
                h6 { class: "skill", "HTML" }
                h6 { class: "skill", "Tex" }
            }
            div { class: "conta",
                h1 { class: "rounded-header", "Framworks & Libs" }
                h6 { class: "skill", "Embedded Rust" }
                h6 { class: "skill", "Aya Rust (ebpf)" }
                h6 { class: "skill", "Axum" }
                h6 { class: "skill", "Dioxus" }
                h6 { class: "skill", "Tokio" }
                h6 { class: "skill", "API Interfacing" }
                h6 { class: "skill", "SurrealDB" }
                h6 { class: "skill", "Arduino/ESP32" }
            }
            div { class: "conta",
                h1 { class: "rounded-header", "Soft Skills" }
                h6 { class: "skill", "Communication" }
                h6 { class: "skill", "Integrity" }
                h6 { class: "skill", "Teamwork & Collaboration" }
                h6 { class: "skill", "Problem Solving" }
                h6 { class: "skill", "Initiative" }
                h6 { class: "skill", "Commitment" }
                h6 { class: "skill", "Time Management" }
            }
            div { class: "conta",
                h1 { class: "rounded-header", "Languages" }
                h6 { class: "skill", "Native Arabic" }
                h6 { class: "skill", "Profissional English" }
            }
        }
    }
}
