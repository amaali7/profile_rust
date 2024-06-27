use leptos::*;

/// Default Home Page
#[component]
pub fn Info() -> impl IntoView {
    view! {
      <section id="Info" class="page-section info-page">
        <h1 class="rounded-header" style="width: '300px'">
          Refrences
        </h1>
        <div class="info">
          <div class="card">
            <div class="top-box">
              <h1>Alnadhief Hamed Ahmed Alfedeel</h1>
            </div>
            <div class="main-box">
              <div class="element">
                <span class="header el">
                  Position : <span>Assistant Professor</span>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  At : <span>University of Khartoum</span>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  Phone : {" "} <a href="tel:+249-124-662-377">
                    <span>+249-124-662-377</span>
                  </a> {" "} <a href="tel:+249123816018">
                    <span>+249-123-816-018</span>
                  </a>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  E-Mail : <span>alfaln001@uofk.edu, anadief@gmail.com</span>
                </span>
              </div>
            </div>
          </div>
          <div class="card">
            <div class="top-box">
              <h1>Aymen Hamid</h1>
            </div>
            <div class="main-box">
              <div class="element">
                <span class="header el">
                  Position : <span>Assistant Professor</span>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  At : <span>University of Khartoum</span>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  Phone : {" "} <a href="tel:+249912253710">
                    <span>+249-912-253-710</span>
                  </a>
                </span>
              </div>
              <div class="element">
                <span class="header el">
                  E-Mail : <span>aymanimh@gmail.com, aihamid@uok.edu</span>
                </span>
              </div>
            </div>
          </div>
        </div>
        <div class="contact">
          <h1 class="rounded-header" style=" width: '300px'">
            Contact
          </h1>
          <div class="contact-element">
            <div class="element">
              <span class="header el">
                Phone : {" "} <a href="tel:+249994747217">
                  <span>+249-994-747-217</span>
                </a> <br/> {"      "} : {" "} <a href="tel:+249121452356">
                  <span>+249-121-452-356</span>
                </a>
              </span>
            </div>
            {" "}
            <div class="element">
              <span class="header el">
                Address : <span>Sudan, Khartoum,</span> <br/> {"      "}
                <span>Adbabiker north</span>
              </span>
            </div>
          </div>
          <div class="contact-element">
            <div class="element">
              <span class="header el">
                Phone : {" "} <a href="tel:+201147617586">
                  <span>+20-114-761-7586</span>
                </a>
              </span>
            </div>
            {" "}
            <div class="element">
              <span class="header el">
                Address : <span>Egypt, Cairo, Hadayeq El-Maadi</span>
              </span>
            </div>
          </div>
          <div id="Contact" class="contact-links">
            <a
              href="https://github.com/amaali7"
              target="_blank"
              rel="noopener noreferrer"
              class="contact-link"
            >
              GitHub
            </a>
            <a
              href="https://www.linkedin.com/in/abdallah-adam-489aab153"
              target="_blank"
              rel="noopener noreferrer"
              class="contact-link"
            >
              LinkedIn
            </a>
            <a
              href="https://www.facebook.com/amaali17/"
              target="_blank"
              rel="noopener noreferrer"
              class="contact-link"
            >
              Facebook
            </a>
            <a
              href="https://twitter.com/Amaali_7"
              target="_blank"
              rel="noopener noreferrer"
              class="contact-link"
            >
              Twitter
            </a>
          // <a
          // href="https://t.me/Amaali7"
          // target="_blank"
          // rel="noopener noreferrer"
          // class="contact-link"
          // >
          // Telegram
          // </a>
          </div>
        </div>
      </section>
    }
}
