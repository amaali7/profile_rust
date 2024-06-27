use leptos::*;

/// Default Home Page
#[component]
pub fn Skills() -> impl IntoView {
    view! {
      <section id="Skills" class="page-section skills-page">
        <div class="conta">
          <h1 class="rounded-header">Programing</h1>
          <h6 class="skill">Rust</h6>
          <h6 class="skill">C Language</h6>
          <h6 class="skill">Nix/NixOS</h6>
          <h6 class="skill">Python</h6>
          <h6 class="skill">Sass</h6>
          <h6 class="skill">Bash Script</h6>
          <h6 class="skill">HTML</h6>
          <h6 class="skill">Tex</h6>
        </div>
        <div class="conta">
          <h1 class="rounded-header">Framworks & Libs</h1>
          <h6 class="skill">Embedded Rust</h6>
          <h6 class="skill">"Aya Rust (ebpf)"</h6>
          <h6 class="skill">Actix</h6>
          <h6 class="skill">Leptos</h6>
          <h6 class="skill">Tokio</h6>
          <h6 class="skill">Leptonic</h6>
          <h6 class="skill">Arduino/ESP32</h6>
        </div>
        <div class="conta">
          <h1 class="rounded-header">Soft Skills</h1>
          <h6 class="skill">Communication</h6>
          <h6 class="skill">Integrity</h6>
          <h6 class="skill">Teamwork & Collaboration</h6>
          <h6 class="skill">Problem Solving</h6>
          <h6 class="skill">Initiative</h6>
          <h6 class="skill">Commitment</h6>
          <h6 class="skill">Time Management</h6>
        </div>
        <div class="conta">
          <h1 class="rounded-header">Languages</h1>
          <h6 class="skill">Arabic (Mother Tongue)</h6>
          <h6 class="skill">English</h6>
        </div>
      </section>
    }
}
