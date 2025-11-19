use dioxus::prelude::*;

/// A parameterized navbar
#[component]
pub fn NavBar(children: Element, check: Signal<bool>) -> Element {
    rsx! {
        NavBarLogo {  }
        NavBarBotton { check }
        div { class: "nav-link", {children} }
    }
}

/// A parameterized navbar button
#[component]
pub fn NavBarBotton(check: Signal<bool>) -> Element {
    let checked = check();

    rsx! {
        input {
            r#type: "checkbox",
            id: "nav-check",
            checked: checked,
            oninput: move |event| check.set(event.checked())
        }
        div {
            class: "nav-btn",
            label {
                r#for: "nav-check",
                span {}
                span {}
                span {}
            }
        }
    }
}

/// A parameterized navbar item
#[component]
pub fn NavBarItem(text: String, link: String, check: Signal<bool>) -> Element {
    rsx! {
        a {
            href: "{link}",
            onclick: move |_| check.set(!check()),
            "{text}"
        }
    }
}

/// A parameterized navbar logo
#[component]
pub fn NavBarLogo() -> Element {
    rsx! {
        div { class: "current-part",
            div { class: "nav-logo",
                a {
                    href: "/",
                    span { class: "span-1", "Profile" }
                    span { class: "span-5", "::" }
                    span { class: "span-4", "<" }
                    span { class: "span-1", "Me" }
                    span { class: "span-4", ">" }
                    span { class: "span-1", "()" }
                }
            }
        }
    }
}
