use std::time::Duration;

use leptos::*;

/// Default Home Page
#[component]
pub fn Education() -> impl IntoView {
    let (image, set_image) = create_signal(0);
    let images = [
        "img/uofk-slide1.jpg",
        "img/uofk-slide2.jpg",
        "img/uofk-slide3.jpg",
    ];
    create_effect(move |_| {
        set_interval(
            move || {
                if image() == 2 {
                    set_image(0);
                } else {
                    set_image(image() + 1);
                }
            },
            Duration::from_secs(3),
        );
    });
    view! {
        <section id="Education" class="page-section education-page">
            <div class="image-section">
                <img src=move || { format!("./{}", images[image.get()]) } class="uofk-images" alt="UofK  1"/>
            </div>
            <div class="text-section">
                <h1>src/lib.rs</h1>
                <div class="text">
                    <span class="span-1">struct</span>
                    Education
                    <span class="span-1">{"{"}</span>
                    <br/>

                    {}
                    <span class=" tab-1">institute</span>
                    <span class="span-4">:</span>
                    <span class="span-1">String</span>
                    <span class="span-4">,</span>
                    <br/>

                    {}
                    <span class=" tab-1">department</span>
                    <span class="span-4">:</span>
                    <span class="span-1">String</span>
                    <span class="span-4">,</span>
                    <br/>

                    {}
                    <span class=" tab-1">degree</span>
                    <span class="span-4">:</span>
                    <span class="span-1">String</span>
                    <span class="span-4">,</span>
                    <br/>

                    {}
                    <span class=" tab-1">from</span>
                    <span class="span-4">:</span>
                    <span class="span-1">u16</span>
                    <span class="span-4">,</span>
                    <br/>

                    {}
                    <span class=" tab-1">till</span>
                    <span class="span-4">:</span>
                    <span class="span-1">u16</span>
                    <span class="span-4">,</span>
                    <br/>

                    <span class="span-1">{"}"}</span>
                    <span class="span-4">;</span>
                    <br/>
                    <br/>
                    <span class="span-1">impl</span>
                    Default
                    <span class="span-4">for</span>
                    Education
                    <span class="span-1">{"{"}</span>
                    <br/>

                    {}
                    <span class="span-4 tab-1">fn</span>
                    <span class="span-1">" default"</span>
                    <span class="span-1">"() "</span>
                    <span class="span-4">"->"</span>
                    Self
                    <span class="span-1">"{"</span>
                    <br/>

                    <span class="tab-1">Self</span>
                    <span class="span-1">"{"</span>
                    <br/>
                    <span class="tab-2">institute</span>
                    <span class="span-4">" : "</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"UOfK"</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"."</span>
                    <span class="span-3">"to_owned"</span>
                    <span class="span-1">"()"</span>
                    <span class="span-4">,</span>

                    <br/>
                    <span class="tab-2">department</span>
                    <span class="span-4">" : "</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"Physics"</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"."</span>
                    <span class="span-3">"to_owned"</span>
                    <span class="span-1">"()"</span>
                    <span class="span-4">,</span>
                    <br/>
                    <span class="tab-2">degree</span>
                    <span class="span-4">" : "</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"B.Sc (Honors)"</span>
                    <span class="span-4">"\""</span>
                    <span class="span-4">"."</span>
                    <span class="span-3">"to_owned"</span>
                    <span class="span-1">"()"</span>
                    <span class="span-4">,</span>
                    <br/>
                    <span class="tab-2">from</span>
                    <span class="span-4">" : "</span>
                    <span class="span-5">2017</span>
                    <span class="span-4">,</span>
                    <br/>
                    <span class="tab-2">till</span>
                    <span class="span-4">" : "</span>
                    <span class="span-5">2017</span>
                    <span class="span-4">,</span>
                    <br/>
                    <span class="span-1 tab-2">"}"</span>

                    <br/>

                    {}

                    <span class="span-1 tab-1">{"}"}</span>
                    <br/>
                    <span class="span-1">{"}"}</span>
                </div>
            </div>
        </section>
    }
}
