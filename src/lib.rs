use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::components::navbar::NavBar;
use crate::pages::education::Education;
use crate::pages::experience::Experience;
use crate::pages::home::Home;
use crate::pages::info::Info;
use crate::pages::not_found::NotFound;
use crate::pages::skills::Skills;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <nav>
                <NavBar/>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/info" view=Info/>
                    <Route path="/skills" view=Skills/>
                    <Route path="/experience" view=Experience/>
                    <Route path="/education" view=Education/>
                    <Route path="/*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
