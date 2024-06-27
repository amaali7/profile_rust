use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn Model(children: Children, #[prop(into)] close_model: RwSignal<bool>) -> impl IntoView {
    view! {
      <div
        class="model-container"
        on:click=move |_| {
            close_model
                .update(|cm| {
                    *cm = !(*cm);
                })
        }
      >

        <div class="card">{children()}</div>
      </div>
    }
}
