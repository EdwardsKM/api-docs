#![allow(non_snake_case)]

use dioxus::prelude::*;
#[component]
pub(crate) fn FieldComponent(
    parent: String,
    data: crate::models::service::ObjectField,
    styles: String,
) -> Element {

    rsx!(
        div { class: "attribute",
            span { class: "details",
                {
                    let path = format!("{}", &parent);
                        rsx!(
                            span {class : "field-name", "{path}"})
                },
                { match data.optional {
                    true => rsx!(span { class : "chip", "nullable"}),
                    false => rsx!({}),
                }
                },
                { match data.r#type {
                    crate::models::service::ItemDocs::Enum(_) => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of enums"}),
                            false => rsx!(span { class : "chip", "enum"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Object(_) => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of dictionaries"}),
                            false => rsx!(span { class : "chip", "dictionary"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Primitive(_) => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of strings"}),
                            false => rsx!(span { class : "chip", "string"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Timestamp => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of timestamps"}),
                            false => rsx!(span { class : "chip", "timestamp"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Number => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of numbers"}),
                            false => rsx!(span { class : "chip", "number"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::String => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of strings"}),
                            false => rsx!(span { class : "chip", "string"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Boolean => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of booleans"}),
                            false => rsx!(span { class : "chip", "boolean"}),
                        }
                        }
                    },
                    crate::models::service::ItemDocs::Binary => {
                        { match data.array {
                            true => rsx!(span { class : "chip", "array of byte objects"}),
                            false => rsx!(span { class : "chip", "bytes"}),
                        }
                        }
                    }
                }
                },

                { match data.expandable {
                    true => rsx!(span { class : "chip expandable", "expandable"}),
                    false => rsx!({}),
                }
                }
            }

            p { class: "description", "{data.description?}" }
        }
    )
}
