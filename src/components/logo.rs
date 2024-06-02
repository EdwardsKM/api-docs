use dioxus::prelude::*;
#[component]
pub(crate) fn Logo() -> Element {
    rsx!(
        a { class: "logo", href: "https://docs.fastapayments.com/api",
            img {
                style: "border-radius : 50%; height : 1.25rem; width : 1.25rem; ",
                src: "logo.png",
                alt: "Fastapayments Logo"
            }
            "Fastapayments"
        }
    )
}