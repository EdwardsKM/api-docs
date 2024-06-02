
#![allow(non_snake_case)]

use dioxus::prelude::*;

// A Controlled form text-input component
#[inline_props]
pub fn TextInputComponent<'a>(
    cx: Scope<'a>,
    placeholder: &'a str,
    styling: &'a str,
    input_type: &'a str,
    label: &'a str,
    label_visible: bool,
    label_position: &'a str,
    value: &'a UseState<String>,
    on_input: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let toggle_visibility = match label_visible {
        true => "",
        false => "visuallyhidden",
    };

    let label_style = r"
            margin-top: 1em;
            font-size : 14px;
            font-weight : 550;
           
            box-shadow: 0 0 5px rgba(81, 203, 238, 1);
            ";

    let input_style = r"
            border: none;
            border-radius: 3px;
            margin-top: 1em;
            height: 1.5em;
            max-width: 15em;
            display : flex;
            text-align: right;
            padding: 0.5em;
            box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px, rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
                                ";

    cx.render(rsx! {
        // The input container.
        div { style: "display : flex;
                       align-items : center; 
                       gap : 6em;
                       justify-content : space-between; 
                       margin-right : 2em;",

            // The label for the formInput
            label {
                class: "{toggle_visibility}",
                style: "{label_style}",
                r#for: "{label}",
                "{label}"
            }
            // The input component itself
            input {
                class: "{styling}",
                style: "{input_style}",
                id: "{label}",
                title: "{label}",
                tabindex: 0,
                aria_label: "{label}",
                placeholder: "{placeholder}",
                r#type: "{input_type}",
                value: "{value}",
                oninput: move |evt| on_input.call(evt)
            }
        }
    })
}
