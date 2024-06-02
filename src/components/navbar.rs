use dioxus::prelude::*;

use crate::components::{self, logo::Logo, ThemeToggle::ThemeToggle};

#[component]
pub fn Navbar() -> Element {

    rsx! {
        nav { class: "navbar",
            Logo {}
            ThemeToggle {}
            components::icons::Hamburger{}
            ul { class : "nav-links",
                li {
                    a {
                        class: "nav-link",
                        href: "https://www.fastapayments.com/resources",
                        "data-title": "Explore guides and tutorials on how to use Fastapayments apis",
                        "Resources"
                    }
                }
                li {
                    a {
                        class: "nav-link",
                        href: "https://www.fastapayments.com/support",
                        "data-title": "Get help and support",
                        "Support"
                    }
                }
                li {
                    a {
                        class: "sign-in",
                        href: "https://auth.fastapayments.com/",
                        "data-title": "Sign into your fastapayments account",
                        "Sign In"
                    }
                }
            }
        }
    }
}
