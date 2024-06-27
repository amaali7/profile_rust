use leptos::*;
use leptos_router::A;

/// A parameterized navbar
#[component]
pub fn NavBar<F, IV>(children: Children, logo: F, check: RwSignal<bool>) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
      {logo()}
      <NavBarBotton check=check/>
      <div class="nav-link">{children()}</div>
    }
}

/// A parameterized navbar botton
#[component]
pub fn NavBarBotton(check: RwSignal<bool>) -> impl IntoView {
    view! {
      <input type="checkbox" id="nav-check" prop:checked=check/>
      <div class="nav-btn">
        <label for="nav-check">
          <span></span>
          <span></span>
          <span></span>
        </label>
      </div>
    }
}

/// A parameterized navbar item
#[component]
pub fn NavBarItem(text: String, link: String, check: RwSignal<bool>) -> impl IntoView {
    view! {
      <A
        href=link.clone()
        exact=true
        on:click=move |_| { check.update(|value| { *value = !(*value) }) }
      >
        {text}
      </A>
    }
}

/// A parameterized navbar item
#[component]
pub fn NavBarLogo() -> impl IntoView {
    // let check = store_value(false);
    view! {
      <div class="current-part">
        <div class="nav-logo">
          <A href="/">
            <span class="span-1">"Profile"</span>
            <span class="span-5">"::"</span>
            <span class="span-4">"<"</span>
            <span class="span-1">"Me"</span>
            <span class="span-4">">"</span>
            <span class="span-1">"()"</span>
          </A>
        </div>
      </div>
    }
}
