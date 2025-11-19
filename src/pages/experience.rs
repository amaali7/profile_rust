use dioxus::prelude::*;

/// Default Home Page
#[component]
pub fn Experience() -> Element {
    // Note: The commented out model functionality from Leptos would need to be
    // implemented separately in Dioxus using conditional rendering

    rsx! {
            section { id: "Experience", class: "page-section experience-page",
                h1 { class: "rounded-header", style: "width: 300px", "Freelancer" }
                div { class: "container",
                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/Clock.png"),
                                    alt: "Clock",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10",
                                a {
                                    href: "https://www.facebook.com/profile.php?id=100086326303701&mibextid=ZbWKwL",
                                    class: "linko",
                                    target: "_blank",
                                    "Mechatronics"
                                }
                            }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "SpaceTime Clock" }
                            h6 { class: "span", "Oct 2021 - Nov 2022" }
                            p { class: "body",
                                "Designed stepper motor control circuits and implemented motor-driving logic using Python and C on a Raspberry Pi."
                                "Integrated a Real-Time Clock (RTC) module to maintain precise timekeeping independent of system reboots or network connectivity."
                                "Built a synchronized control system combining hardware timing, stepper positioning, and software scheduling for accurate mechanical time display."
                                br {}
                            }
                        }
                    }

                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/water.png"),
                                    alt: "Monitoring Water Sources",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10",
                                a {
                                    href: "http://github.com/amaali7/WHO",
                                    class: "linko",
                                    target: "_blank",
                                    "Web App"
                                }
                            }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "Monitoring Water Sources" }
                            h6 { class: "span", "Dec 2018 - Oct 2020" }
                            p { class: "body",
                                "Designed and built a Python-based system for collecting and analyzing biological and chemical water-quality data across distributed geographic zones. Implemented zone-specific data isolation, centralized administration access, backend data processing, trend analysis, and secure dashboards. Demonstrated strong skills in system architecture, backend development, permission models, and data-driven automation."
                            }
                        }
                    }

                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/uv.png"),
                                    alt: "UV Lamp",
                                    style: "height: 110px",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10",
                                a {
                                    href: "https://github.com/amaali7/UV_Final",
                                    class: "linko",
                                    target: "_blank",
                                    "Health Care"
                                }
                            }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "UV Stralization System" }
                            h6 { class: "span", "Feb 2019 - Dec 2020" }
                            p { class: "body",
                                "Developed an IoT-enabled UV sterilization system integrating motion sensors, current sensors, wireless control, a web-based UI, manual override features, and a real-time alarm subsystem. Delivered full embedded-to-web integration, covering sensor interfacing, network communication, control logic, and safe automation. Showcased expertise in embedded systems, wireless protocols, and building reliable control systems."
                            }
                        }
                    }

                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/smsproxy.png"),
                                    alt: "SMS Automatic Response System",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10",
                                a {
                                    href: "https://github.com/amaali7/ESP32-SMS",
                                    class: "linko",
                                    target: "_blank",
                                    "Communications Technology"
                                }
                            }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "SMS Automatic Response System" }
                            h6 { class: "span", "Apr 2019 - Mar 2020" }
                            p { class: "body",
                                "Built an end-to-end SMS interaction platform where an ESP32 device receives SMS messages, forwards them to a central server, processes them automatically or manually, and returns responses via SMS. Implemented embedded firmware, server-side logic, automation rules, and communication routing. Demonstrated capabilities in embedded programming, server development, and designing robust communication workflows."
                            }
                        }
                    }
                }

                h1 { class: "rounded-header", style: "width: 300px", "Teaching" }
                div { class: "container",
                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/uofk.png"),
                                    alt: "University Of Khartoum",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10", "University Of Khartoum" }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "Department Of Physics" }
                            h6 { class: "span", "Dec 2017 - Dec 2020" }
                            p { class: "body",
                                "Prepared and delivered physics laboratory experiments, ensuring proper calibration and functionality of lab equipment, and maintained computer hardware and software at computer labs."
                            }
                        }
                    }
    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/STEM.png"),
                                    alt: "Stem",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10", "STEM Education Sudan" }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "Electronics and Programing" }
                            h6 { class: "span", "Dec 2017 - Dec 2020" }
                            p { class: "body",
                                "Taught basic electronics and embedded systems (Arduino, ESP32/8266) during youth summer camps."
                            }
                        }
                    }


                    div { class: "card",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/nileU.png"),
                                    alt: "Nile University",
                                    class: "mada"
                                }
                            }
                            h1 { class: "marg10", "Nile University" }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "Department Of Physics" }
                            h6 { class: "span", "Nov 2017 - Feb 2018 " }
                            p { class: "body", "Delivered first-year physics laboratory sessions. " }
                        }
                    }

                    div { class: "card mas",
                        div { class: "top-box",
                            div { class: "logo",
                                img {
                                    src: asset!("images/almadain.jpg"),
                                    alt: "Almadin College for Science and Technology",
                                    class: "mada"
                                }
                            }
                            h1 { "Almadin College for Science and Technology" }
                        }
                        div { class: "main-box",
                            h1 { class: "header", "Department Of Physics" }
                            h6 { class: "span", "Nov 2017 - Feb 2019" }
                            p { class: "body", "Delivered first-year physics laboratory sessions. " }
                        }
                    }

                                }
            }
        }
}
