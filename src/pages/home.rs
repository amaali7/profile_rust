use leptos::*;
use leptos_router::A;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
      <section id="Home" class="page-section home-page">
        <div class="images-container">
        <div class="images">
          <div class="ferris-wheel">
            <img class="personal-image" src="img/pi.jpg" alt=""/>
              <div class="wheel">
                  <div class="cabin cabin-1"><img src="img/Bash.png" alt="Cabin 1"/></div>
                  <div class="cabin cabin-2"><img src="img/Tex.png" alt="Cabin 2"/></div>
                  <div class="cabin cabin-3"><img src="img/nix.png" alt="Cabin 3"/></div>
                  <div class="cabin cabin-4"><img src="img/leptos.png" alt="Cabin 4"/></div>
                  <div class="cabin cabin-5"><img src="img/aya.svg" alt="Cabin 5"/></div>
                  <div class="cabin cabin-6"><img src="img/C.png" alt="Cabin 6"/></div>
                  <div class="cabin cabin-7"><img src="img/rust.png" alt="Cabin 7"/></div>
                  <div class="cabin cabin-8"><img src="img/Python.png" alt="Cabin 8"/></div>
              </div>
          </div></div>
        </div>
        <div class="text">
          <div class="text-area">
            <h1>src/main.rs</h1>
            <div>
              <span class="span-4">"use "</span>
              <span class="span-1">create</span>
              <span class="span-4">{"::"}</span>
              <span class="span-3">physics</span>
              <span class="span-4">{"::"}</span>
              <span class="span-1">"*"</span>
              <span class="span-4">{";"}</span>
              <br/>
              <span class="span-4">"use "</span>
              <span class="span-1">create</span>
              <span class="span-4">{"::"}</span>
              <span class="span-3">hardware</span>
              <span class="span-4">{"::"}</span>
              <span class="span-1">"*"</span>
              <span class="span-4">{";"}</span>
              <br/>
              <span class="span-4">"use "</span>
              <span class="span-1">create</span>
              <span class="span-4">{"::"}</span>
              <span class="span-3">programming</span>
              <span class="span-4">{"::"}</span>
              <span class="span-1">"*"</span>
              <span class="span-4">{";"}</span>
              <br/>
              <span class="span-4">"use "</span>
              <span class="span-1">create</span>
              <span class="span-4">{"::"}</span>
              <span class="span-3">is_fun</span>
              <span class="span-4">{"::"}</span>
              <span class="span-1">"*"</span>
              <span class="span-4">{";"}</span>
              <br/>
              <br/>
              <span class="span-4">fn</span>
              <span class="span-3">" main "</span>
              <span class="span-1">{"("} {"){"}</span>
              <br/>

              <span class="span-1 tab-1">"let"</span>
              <span class="span-3">" person"</span>
              <span class="span-4">{" = "}</span>
              <span class="span-1">"Profile"</span>
              <span class="span-5">"::"</span>
              <span class="span-4">"<"</span>
              <span class="span-1">"Me"</span>
              <span class="span-4">">"</span>
              <span class="span-4">"::"</span>
              <span class="span-3">"new"</span>
              <span class="span-1">"()"</span>
              <span class="span-4">{";"}</span>
              <br/>
              <span class="span-1 tab-1">"match "</span>
              <span class="span-3">" person"</span>
              <span class="span-4">{"."}</span>
              <span class="span-3">"is_a_life"</span>
              <span class="span-1">"()"</span>
              <span class="span-1">" {"</span>

              <br/>
              <span class="span-1 tab-2">"Some"</span>
              <span class="span-1">"("</span>
              <span class="span-3">"me"</span>
              <span class="span-1">") "</span>
              <span class="span-4">" => "</span>
              <span class="span-3">" me"</span>
              <span class="span-4">{"."}</span>
              <span class="span-3">"run"</span>
              <span class="span-1">"()"</span>
              <span class="span-4">{","}</span>
              <br/>

              <span class="span-1 tab-2">"None"</span>
              <span class="span-4">" => "</span>
              <span class="span-3">"panic"</span>
              <span class="span-4">{"!"}</span>
              <span class="span-3">""</span>
              <span class="span-1">"("</span>
              <span class="span-4">{"\""}</span>
              <span class="span-1">"😅"</span>
              <span class="span-4">{"\""}</span>
              <span class="span-1">")"</span>
              <span class="span-4">{","}</span>
              <br/>
              <span class="span-1 tab-1">" }"</span>

              <br/>
              <span class="span-1">" }"</span>

            </div>
            <A href="#info">cargo  run</A>
          </div>
        </div>
      </section>
    }
}
