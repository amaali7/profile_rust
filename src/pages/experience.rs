// use crate::components::model::Model;
use leptos::*;

/// Default Home Page
#[component]
pub fn Experience() -> impl IntoView {
    // let model_h = create_rw_signal(false);
    //     <Show when=move || { model_h.get() }>
    //       <Model close_model=model_h>
    //         <div className="top-box">
    //           <h1>Video</h1>
    //         </div>
    //         <div className="main-box">
    //           <iframe src="https://www.youtube.com/embed/HGAQa51KhMM"></iframe>
    //         </div>
    //       </Model>
    //     </Show>
    // <div>
    //                   <button
    //                     class="video"
    //                     on:click=move |_| { model_h.update(|v| *v = !(*v)) }
    //                   >
    //                     Video
    //                   </button>
    //                 </div>

    view! {
      <section id="Experience" class="page-section experience-page">
        <h1 class="rounded-header" style="width : '300px'">
          Freelancer
        </h1>
        <div class="container">
          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img src="img/Clock.png" alt="Clock" class="mada"/>

              </div>
              <h1 class="marg10">
                <a
                  href="https://www.facebook.com/profile.php?id=100086326303701&mibextid=ZbWKwL"
                  class="linko"
                  target="_blank"
                >
                  Mechatronics
                </a>
              </h1>
            </div>
            <div class="main-box">
              <h1 class="header">SpaceTime Clock</h1>
              <h6 class="span">2021 - 2022</h6>
              <p class="body">
                Participation in the project was done by making control circuits
                and programming the control system using Python and C languages on
                the Raspberry Pi. <br/>
              </p>
            </div>
          </div>

          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img src="img/water.png" alt="UofK" class="mada"/>
              </div>
              <h1 class="marg10">
                <a
                  href="http://github.com/amaali7/WHO"
                  class="linko"
                  target="_blank"
                >
                  Web App
                </a>
              </h1>
            </div>
            <div class="main-box">
              <h1 class="header">Monitoring Water Sources</h1>
              <h6 class="span">2018 - 2019</h6>
              <p class="body">
                A web application that help to collecting biological and chemical
                information about water sources and isolating data based on
                geographical zoon and the central administration is the only one
                have access to all the data.
              </p>
            </div>
          </div>

          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img
                  src="img/uv.png"
                  alt="UV Lamp"
                  height="110px"
                  class="mada"
                />
              </div>
              <h1 class="marg10">
                <a
                  href="https://github.com/amaali7/UV_Final"
                  class="linko"
                  target="_blank"
                >
                  Health Care
                </a>
              </h1>
            </div>
            <div class="main-box">
              <h1 class="header">Smart UV Lamb</h1>
              <h6 class="span">2018 - 2019</h6>
              <p class="body">
                A smart UV sterilization system that includes motion and current
                sensors, controlled using wireless network , supports a web user
                interface, a manual control system, and alarm system.
              </p>
            </div>
          </div>
          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img src="img/smsproxy.png" alt="SMS Proxy" class="mada"/>
              </div>
              <h1 class="marg10">
                <a
                  href="https://github.com/amaali7/ESP32-SMS"
                  class="linko"
                  target="_blank"
                >
                  Communications Technology
                </a>
              </h1>
            </div>
            <div class="main-box">
              <h1 class="header">SMS Smart Proxy</h1>
              <h6 class="span">2018 - 2019</h6>
              <p class="body">
                An interactive response system that transfers SMS to a central
                server that responds appropriately to them, either automatically
                or manually, and the response is sent back to the sender by SMS.
              </p>
            </div>
          </div>
        </div>
        <h1 class="rounded-header" style=" width: '300px'">
          Teaching
        </h1>
        <div class="container">
          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img
                  src="img/uofk.png"
                  alt="University Of Khartoum"
                  class="mada"
                />
              </div>
              <h1 class="marg10">University Of Khartoum</h1>
            </div>
            <div class="main-box">
              <h1 class="header">Department Of Physics</h1>
              <h6 class="span">2017 - 2020</h6>
              <p class="body">
                I work at physics laboratories in the preparation and teaching
                experiments and maintenance of laboratory equipment, computer
                software and hardware.
              </p>
            </div>
          </div>
          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img src="img/nileU.png" alt="Nile University" class="mada"/>
              </div>
              <h1 class="marg10">Nile University</h1>
            </div>
            <div class="main-box">
              <h1 class="header">Department Of Physics</h1>
              <h6 class="span">2018 - 2019</h6>
              <p class="body">Teaching in 1st year Laboratories</p>
            </div>
          </div>
          <div class="card mas">
            <div class="top-box">
              <div class="logo">
                <img
                  src="img/almadain.jpg"
                  alt="Almadin College for Science and Technology"
                  class="mada"
                />
              </div>
              <h1>Almadin College for Science and Technology</h1>
            </div>
            <div class="main-box">
              <h1 class="header">Department Of Physics</h1>
              <h6 class="span">2017 - 2019</h6>
              <p class="body">Teaching in 1st year Laboratories</p>
            </div>
          </div>
          <div class="card">
            <div class="top-box">
              <div class="logo">
                <img src="img/STEM.png" alt="Stem" class="mada"/>
              </div>
              <h1 class="marg10">STEM Education Sudan</h1>
            </div>
            <div class="main-box">
              <h1 class="header">Electronics and Programing</h1>
              <h6 class="span">2017 - 2020</h6>
              <p class="body">
                "Educate children basic electronics and simple embedded like
                Arduino, ESP32 and ESP8266, with several sensors during summer
                camp's."
              </p>
            </div>
          </div>
        </div>
      </section>
    }
}
