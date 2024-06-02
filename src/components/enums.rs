#![allow(non_snake_case)]

use dioxus::prelude::*;
#[component]
pub(crate) fn EnumDocsComponent(
    optional: bool,
    array: bool,
    parent: String,
    en: crate::models::service::LibraryEnumDocs,
) -> Element {
    rsx!(
        details { class: "enum-component",
            summary {
                {
                    let path = format!("{}", &parent);
                        rsx!(
                            span {class : "nested", "{path}"},
                            {
                                match optional {
                                    true => rsx!(span {class : "chip", "optional enum"}),
                                    false => match array {
                                        true => rsx!(span {class : "chip","array of enums"}),
                                        false => rsx!(span {class : "chip","enum"})
                                    }
                                }
                            }
                        )
                    }
            }
            p { "{en.docs?}" }

            ul { class: "variant-list",
                {
                    en.variants.iter().map(|variant| {
                            rsx!(
                                li { class : "enum-list-item",
                                    {
                                        let path = format!("{}", variant.name);
                                        rsx!(span{ class : "enum-field", "{path}"})
                                    },
                                    p { class : "field-description",
                                        "{variant.clone().docs?}"
                                    }
                                }
                            )
                    } )
                    }
            }
        }
    )
}
