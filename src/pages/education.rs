use dioxus::prelude::*;
use dioxus_sdk::time::use_interval;
use std::time::Duration;

/// Default Home Page
#[component]
pub fn Education() -> Element {
    let mut image = use_signal(|| 0);
    let images = [
        asset!("images/uofk-slide1.jpg"),
        asset!("images/uofk-slide2.jpg"),
        asset!("images/uofk-slide3.jpg"),
    ];

    // This hook will run the closure every 1 second
    use_interval(Duration::from_secs(2), move |()| {
        image.set((image() + 1) % images.len());
    });
    rsx! {
        section { id: "Education", class: "page-section education-page",
            div { class: "image-section",
                img {
                    src: images[image()],
                    class: "uofk-images",
                    alt: "UofK {image()}"
                }
            }
            div { class: "text-section",
                h1 { "src/lib.rs" }
                div { class: "text",
                    span { class: "span-1", "struct" }
                    " Education"
                    span { class: "span-1", "{{" }
                    br {}

                    span { class: "tab-1", "institute" }
                    span { class: "span-4", ":" }
                    span { class: "span-1", "String" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-1", "department" }
                    span { class: "span-4", ":" }
                    span { class: "span-1", "String" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-1", "degree" }
                    span { class: "span-4", ":" }
                    span { class: "span-1", "String" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-1", "from" }
                    span { class: "span-4", ":" }
                    span { class: "span-1", "Date" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-1", "till" }
                    span { class: "span-4", ":" }
                    span { class: "span-1", "Date" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "span-1", "}}" }
                    span { class: "span-4", ";" }
                    br {}
                    br {}

                    span { class: "span-1", "impl" }
                    " Default"
                    span { class: "span-4", " for " }
                    "Education"
                    span { class: "span-1", " {{" }
                    br {}

                    span { class: "span-4 tab-1", "fn" }
                    span { class: "span-1", " default() " }
                    span { class: "span-4", "-> " }
                    "Self"
                    span { class: "span-1", " {{" }
                    br {}

                    span { class: "tab-1", "Self" }
                    span { class: "span-1", " {{" }
                    br {}

                    span { class: "tab-2", "institute" }
                    span { class: "span-4", " : " }
                    span { class: "span-4", "\"UOfK\"" }
                    span { class: "span-4", "." }
                    span { class: "span-3", "to_owned" }
                    span { class: "span-1", "()" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-2", "department" }
                    span { class: "span-4", " : " }
                    span { class: "span-4", "\"Physics\"" }
                    span { class: "span-4", "." }
                    span { class: "span-3", "to_owned" }
                    span { class: "span-1", "()" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-2", "degree" }
                    span { class: "span-4", " : " }
                    span { class: "span-4", "\"B.Sc (Honors)\"" }
                    span { class: "span-4", "." }
                    span { class: "span-3", "to_owned" }
                    span { class: "span-1", "()" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-2", "from" }
                    span { class: "span-4", " : " }
                    span { class: "span-5", "Oct 2017" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "tab-2", "till" }
                    span { class: "span-4", " : " }
                    span { class: "span-5", "Dec 2017" }
                    span { class: "span-4", "," }
                    br {}

                    span { class: "span-1 tab-2", "}}" }
                    br {}
                    span { class: "span-1 tab-1", "}}" }
                    br {}
                    span { class: "span-1", "}}" }
                }
            }
        }
    }
}
