use leptos::*;

/// A parameterized navbar
#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <NavBarLogo/>
        <NavBarItem text=".home()".to_owned() link="/".to_owned()/>
        <NavBarItem text=".education()".to_owned() link="/education".to_owned()/>
        <NavBarItem text=".experience()".to_owned() link="/experience".to_owned()/>
        <NavBarItem text=".skills()".to_owned() link="/skills".to_owned()/>
        <NavBarItem text=".info()".to_owned() link="/info".to_owned()/>
    }
}

/// A parameterized navbar item
#[component]
pub fn NavBarItem(text: String, link: String) -> impl IntoView {
    view! {
        <a href=link>{text}</a>
    }
}

/// A parameterized navbar item
#[component]
pub fn NavBarLogo() -> impl IntoView {
    view! {
       <h1>"Profile::<Me>()"</h1>
    }
}
