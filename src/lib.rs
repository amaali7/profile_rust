use dioxus::prelude::*;
// Modules
mod components;
mod pages;

// Top-Level pages
use crate::components::navbar::{NavBar, NavBarItem};
use crate::pages::education::Education;
use crate::pages::experience::Experience;
use crate::pages::home::Home;
use crate::pages::info::Info;
use crate::pages::skills::Skills;

#[component]
pub fn App() -> Element {
    let botton_h = use_signal(|| false);

    rsx! {
        // Document head metadata
        head {
            title { "Abdallah Ali Profile" }
            meta { charset: "UTF-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            link { rel: "stylesheet", href: asset!("/style/main.scss") }
        }

        // Main content with router
        main {
            nav { class: "navbar",
                NavBar {
                    check: botton_h,
                    NavBarItem {
                        text: "p.home()".to_owned(),
                        link: "#Home".to_owned(),
                        check: botton_h
                    }
                    NavBarItem {
                        text: "p.skills()".to_owned(),
                        link: "#Skills".to_owned(),
                        check: botton_h
                    }
                    NavBarItem {
                        text: "p.experience()".to_owned(),
                        link: "#Experience".to_owned(),
                        check: botton_h
                    }

                    NavBarItem {
                        text: "p.education()".to_owned(),
                        link: "#Education".to_owned(),
                        check: botton_h
                    }

                    NavBarItem {
                        text: "p.info()".to_owned(),
                        link: "#Info".to_owned(),
                        check: botton_h
                    }
                }
            }

            // Route components
            Home {}
            Skills {}
            Experience {}
            Education {}
            Info {}
        }
    }
}
