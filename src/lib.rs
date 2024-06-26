use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::components::navbar::{NavBar, NavBarItem, NavBarLogo};
use crate::pages::education::Education;
use crate::pages::experience::Experience;
use crate::pages::home::Home;
use crate::pages::info::Info;
use crate::pages::skills::Skills;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let botton_h = create_rw_signal(false);
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Abdallah Ali Profile"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>

            <main>
                <nav class="navbar">
                    <NavBar check=botton_h logo=|| view! { <NavBarLogo/> }>
                        <NavBarItem
                            text=".home()".to_owned()
                            link="#Home".to_owned()
                            check=botton_h
                        />
                        <NavBarItem
                            text=".education()".to_owned()
                            link="#Education".to_owned()
                            check=botton_h
                        />
                        <NavBarItem
                            text=".experience()".to_owned()
                            link="#Experience".to_owned()
                            check=botton_h
                        />
                        <NavBarItem
                            text=".skills()".to_owned()
                            link="#Skills".to_owned()
                            check=botton_h
                        />

                        <NavBarItem
                            text=".info()".to_owned()
                            link="#Info".to_owned()
                            check=botton_h
                        />

                    </NavBar>
                </nav>

                <Home/>
                <Education/>
                <Experience/>
                <Skills/>
                <Info/>

            </main>
        </Router>
    }
}
