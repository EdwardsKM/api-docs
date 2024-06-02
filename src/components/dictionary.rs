#![allow(non_snake_case)]

use dioxus::prelude::*;
#[component]
pub(crate) fn DictionaryDocsComponent(
    optional: bool,
    array: bool,
    parent: String,
    obj: crate::models::service::LibraryStructDocs,
) -> Element {
    let chunks = parent.split(".").collect::<Vec<&str>>();

    let styling = match chunks.len() {
        x if x == 0 => "".to_string(),
        x if x == 1 => "margin-left: 2.2em; margin-block: .35em; padding-block: 5px;".to_string(),
        x if x == 2 => "margin-top: .5em; margin-left: 2em;".to_string(),
        _ => "margin-left: 1.65em;".to_string(),
    };

    let field_styles = match chunks.len() {
        x if x == 0 => "margin-left : 3.5em;".to_string(),
        x if x == 1 => "border-top : 1px solid var(--surface4)".to_string(),
        x if x == 2 => "border-top : 1px solid var(--surface4); margin-left: 2.4em;".to_string(),
        _ => "margin-top : .5em; border-top : 1px solid var(--surface4)".to_string(),
    };

    rsx!(
        details { style: "{styling}", class: "dictionary-component",
            summary {
                {
                    rsx!(
                        span { class : "nested", "{parent}"}
                        {
                            match optional {
                                true => rsx!(span {class : "chip", "optional"} span {class : "chip", "dictionary"}),
                                false => match array {
                                    true => rsx!(span {class : "chip","array of dictionaries"}),
                                    false => rsx!(span {class : "chip","dictionary"})
                                }
                            }
                        }
                    )
                }
            }
            p { "{obj.docs?}" }
            {
                obj.fields.iter().map(|field| {
                    match field.r#type.clone() {
                        crate::models::service::ItemDocs::Enum(en) => rsx!(crate::components::enums::EnumDocsComponent{
                            optional : field.optional, array : field.array, parent : format!("{}.{}", &parent, &field.name), en : en}) ,
                        crate::models::service::ItemDocs::Object(obj) => {
                            rsx!(DictionaryDocsComponent{optional : field.optional, array: field.array, parent : format!("{}.{}", &parent, &field.name) , obj : obj})
                        },
                        _ => {
                            let attribute = crate::models::service::ObjectField {
                                name: field.name.clone(),
                                top_level: match &field.docs {
                                    Some(d) => {
                                        if d.contains("top_level") {
                                            Some(true)
                                        } else {
                                            None
                                        }
                                    }
                                    None => None,
                                },
                                description: field.docs.clone(),
                                optional: field.optional,
                                expandable: match &field.docs {
                                    Some(d) => {
                                        if d.contains("expandable") {
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                    None => false,
                                },
                                array: field.array,
                                r#type: field.r#type.clone() };
                            rsx!(crate::components::field::FieldComponent {parent : format!("{}.{}", &parent, &field.name), data : attribute, styles : "{field_styles}"})
                        }
            }
            })
            }
        }
    )
}
