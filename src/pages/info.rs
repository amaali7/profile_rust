use dioxus::prelude::*;

/// Default Home Page
#[component]
pub fn Info() -> Element {
    rsx! {
        section { id: "Info", class: "page-section info-page",
            h1 { class: "rounded-header", style: "width: 300px", "Refrences" }
            div { class: "info",
                div { class: "card",
                    div { class: "top-box",
                        h1 { "Alnadhief Hamed Ahmed Alfedeel" }
                    }
                    div { class: "main-box",
                        div { class: "element",
                            span { class: "header el",
                                "Position : "
                                span { "Assistant Professor" }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "At : "
                                span { "University of Khartoum" }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "Phone : "
                                a { href: "tel:+96-65-5961-9769",
                                    span { "+96-65-5961-9769" }
                                }
                                   " "
                                a { href: "tel:+249-124-662-377",
                                    span { "+249-124-662-377" }
                                }
                                " "
                                a { href: "tel:+249123816018",
                                    span { "+249-123-816-018" }
                                }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "E-Mail : "
                                span { "alfaln001@uofk.edu, anadief@gmail.com" }
                            }
                        }
                    }
                }
                div { class: "card",
                    div { class: "top-box",
                        h1 { "Aymen Hamid" }
                    }
                    div { class: "main-box",
                        div { class: "element",
                            span { class: "header el",
                                "Position : "
                                span { "Assistant Professor" }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "At : "
                                span { "University of Khartoum" }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "Phone : "
                                a { href: "tel:+249912253710",
                                    span { "+249-912-253-710" }
                                }
                            }
                        }
                        div { class: "element",
                            span { class: "header el",
                                "E-Mail : "
                                span { "aymanimh@gmail.com, aihamid@uok.edu" }
                            }
                        }
                    }
                }
            }
            div { class: "contact",
                h1 { class: "rounded-header", style: "width: 300px", "Contact" }
                div { class: "contact-element",
                    div { class: "element",
                        span { class: "header el",
                            "Phone : "
                            a { href: "tel:+249994747217",
                                span { "+249-994-747-217" }
                            }
                            br {}
                            "      : "
                            a { href: "tel:+249121452356",
                                span { "+249-121-452-356" }
                            }
                        }
                    }
                    " "
                    div { class: "element",
                        span { class: "header el",
                            "Address : "
                            span { "Sudan, Khartoum," }
                            br {}
                            "      "
                            span { "Ed-Babiker north" }
                        }
                    }
                }
                div { class: "contact-element",
                    div { class: "element",
                        span { class: "header el",
                            "Phone : "
                            a { href: "tel:+201147617586",
                                span { "+20-114-761-7586" }
                            }
                        }
                    }
                    " "
                    div { class: "element",
                        span { class: "header el",
                            "Address : "
                            span { "Egypt, Cairo, Hadayeq El-Maadi" }
                        }
                    }
                }
                div { id: "Contact", class: "contact-links",
                    a {
                        href: "https://github.com/amaali7",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link",
                        "GitHub"
                    }
                    a {
                        href: "https://www.linkedin.com/in/abdallah-adam-489aab153",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link",
                        "LinkedIn"
                    }
                    a {
                        href: "https://www.facebook.com/amaali17/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link",
                        "Facebook"
                    }
                    a {
                        href: "https://twitter.com/Amaali_7",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link",
                        "Twitter"
                    }
                    // Note: The Telegram link is commented out in the original
                    // a {
                    //     href: "https://t.me/Amaali7",
                    //     target: "_blank",
                    //     rel: "noopener noreferrer",
                    //     class: "contact-link",
                    //     "Telegram"
                    // }
                }
            }
        }
    }
}
