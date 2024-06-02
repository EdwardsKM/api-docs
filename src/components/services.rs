#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{components::code::syntax_highlight, ApiDocumentation};

// Represents an ApiService in the router
#[component]
pub(crate) fn Service(name: String) -> Element {
    let docs: Vec<crate::models::service::ServiceDocumentation> = use_context::<ApiDocumentation>().services;

    // tracing::info!("{}", format!("{name}"));

    let ser = docs
        .iter()
        .find(|service| service.default_object_url == format!("{}/default", name))
        .unwrap_or_else(|| docs.iter().find(|service| service.name == name).unwrap());

    rsx!(ServiceComponent { data: ser.clone() })
}

#[component]
pub(crate) fn EndpointNested(service: String, endpoint: String, nested: String) -> Element {
    rsx!(Endpoint {
        service,
        endpoint: format!("/{endpoint}/{nested}")
    })
}

// Represents an Api route in the router
#[component]
pub(crate) fn Endpoint(service: String, endpoint: String) -> Element {
    let docs = use_context::<ApiDocumentation>();
    let path_segments = endpoint.split("/").collect::<Vec<&str>>().len();
    match path_segments > 1 {
        true => match endpoint.contains("default") {
            true => {
                tracing::info!("{}", format!("/{service}{endpoint}"));
                let ser = docs
                    .services
                    .iter()
                    .find(|ser| ser.default_object_url == format!("/{service}{endpoint}"))?;

                rsx!(DefaultServiceObjectComponent {
                    data: ser.default.to_owned()?,
                    url: ser.default_object_url.to_owned(),
                })
            }
            false => {
                tracing::info!("/{}{}", &service, &endpoint);
                let mid = endpoint.split("/").collect::<Vec<&str>>()[1];
                tracing::info!("{}", format!("/{service}/{mid}/default") );
                let ser = docs
                    .services
                    .iter()
                    .find(|ser| {
                        ser.default_object_url
                            == format!(
                                "/{service}/{}/default",
                                endpoint.split("/").collect::<Vec<&str>>()[1]
                            )
                    })
                    .unwrap();

                tracing::info!("{}", &endpoint);
                tracing::info!("{}", &ser.name);

                let ep = ser
                    .endpoints
                    .iter()
                    .find(|ep| ep.url == format!("/{service}{endpoint}"))
                    .unwrap();

                let route = ser.routes.iter().find(|route| route.name == ep.name)?;
                tracing::info!("{}", &route.method);
                if route.parameters.is_some() {
                    rsx!(ServiceRouteComponent {
                        route: route.clone(),
                        url: format!("/{service}/{endpoint}")
                    })
                } else {
                    rsx!(ServiceRouteComponentNoParams {
                        route: route.clone(),
                        url: format!("/{service}/{endpoint}")
                    })
                }
            }
        },
        false => {
            tracing::info!("{}", endpoint);
            let ser = docs
                .services
                .iter()
                .find(|ser| ser.default_object_url == format!("/{}/default", service))
                .unwrap();
            tracing::info!("{}", &ser.default_object_url);

            match endpoint.contains("default") {
                true => {
                    rsx!(DefaultServiceObjectComponent {
                        data: ser.default.to_owned()?,
                        url: ser.default_object_url.to_owned(),
                    })
                }
                false => {
                    let ep = ser
                        .endpoints
                        .iter()
                        .find(|ep| ep.url == format!("/{service}/{endpoint}"))
                        .unwrap();
                    let route = ser.routes.iter().find(|route| route.name == ep.name)?;
                    tracing::info!("{}", &route.method);
                    if route.parameters.is_some() {
                        rsx!(ServiceRouteComponent {
                            route: route.clone(),
                            url: format!("/{service}/{endpoint}")
                        })
                    } else {
                        rsx!(ServiceRouteComponentNoParams {
                            route: route.clone(),
                            url: format!("/{service}/{endpoint}")
                        })
                    }
                }
            }
        }
    }
}

#[component]
pub(crate) fn ServiceComponent(data: crate::models::service::ServiceDocumentation) -> Element {
    let navigator = use_navigator();

    let mut show_more = use_signal(|| false);

    let toggle_service_docs = match show_more.read().clone() {
        true => false,
        false => true,
    };

    let rotate = match show_more.read().clone() {
        true => "open",
        false => "closed",
    };

    rsx!(
        section { class: "service",
            div { class: "service-docs",
                h1 { "{data.name}" }
                p { "{data.description?}" }
                button {
                    class: "show-more",
                    onclick: move |_| {
                        show_more.set(toggle_service_docs);
                        // navigator.push(Route::Service { service: "".to_string() });
                    },
                    "Show More"
                    svg {
                        width: "100%",
                        height: "100%",
                        view_box: "0 0 24 24",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "{rotate}",
                        path {
                            fill_rule: "evenodd",
                            clip_rule: "evenodd",
                            d: "M6.29289 8.79289C6.68342 8.40237 7.31658 8.40237 7.70711 8.79289L12 13.0858L16.2929 8.79289C16.6834 8.40237 17.3166 8.40237 17.7071 8.79289C18.0976 9.18342 18.0976 9.81658 17.7071 10.2071L12.7071 15.2071C12.3166 15.5976 11.6834 15.5976 11.2929 15.2071L6.29289 10.2071C5.90237 9.81658 5.90237 9.18342 6.29289 8.79289Z"
                        }
                    }
                }
            }
            { rsx! {
                ServiceEndpoints {endpoints : data.endpoints.clone()}
            }},

            div { class: "detailed-service-docs",
                match show_more() {
                    true => rsx!(
                        DefaultServiceObjectComponent {data : data.default?, url : data.default_object_url}
                        {data.routes.into_iter().map(|route| {
                            let url = &data.endpoints.iter().find(|e| e.route == route.method).unwrap().url;
                            if route.parameters.is_some() {
                                rsx!(ServiceRouteComponent { route, url})
                            } else {
                                rsx!(ServiceRouteComponentNoParams { route, url })
                            }
                        })}
                    ),
                    false => rsx!({}),
                }
            }
        }
    )
}

#[component]
pub(crate) fn ServiceRouteComponent(
    route: crate::models::service::ApiRouteDocumentation,
    url: String,
) -> Element {
    let navigator = use_navigator();
    let p = route.parameters?;
    let p2 = p.clone();

    let ex = r#"
    {
        "id": "di_1M6vk22eZvKYlo2CYMGIhk14",
        "object": "discount",
        "checkout_session": "cs_test_b1mywbZHtQCQW2ncaItVPFqupwmfqNU4IMMdw3lArEBGt0QD0CZDrNQswR",
        "coupon": {
          "id": "wsd",
          "object": "coupon",
          "amount_off": null,
          "created": 1669116350,
          "currency": null,
          "duration": "forever",
          "duration_in_months": null,
          "livemode": false,
          "max_redemptions": null,
          "metadata": {},
          "name": null,
          "percent_off": 10,
          "redeem_by": null,
          "times_redeemed": 1,
          "valid": true
        },
        "customer": "cus_9s6XKzkNRiz8i3",
        "end": null,
        "invoice": null,
        "invoice_item": null,
        "promotion_code": null,
        "start": 1669120702,
        "subscription": null
      }
    "#;
    let pre_formatted = syntax_highlight(ex);

    rsx!(
        section { class: "api-route rad_shadow",

            h2 {
                class: "heading",
                style: "color: var(--text1); font-weight : 600;",
                a {
                    style: "text-decoration : none;",
                    href: "{url}",
                    "data-title": "{route.name}",
                    "{route.name}"
                }
            }

            div { class: "attributes",
                p { class: "route-description", "{route.description?}" }

                h3 { class: "parameters", "Parameters" }

                ul { class: "parameter-list",
                    {

                        p
                        .attributes
                        .into_iter()
                        .filter(|param| param.optional == false)
                        .into_iter()
                        .map(|param| {
                            rsx!(ObjectFieldComponent{field: param, styles : "border-block : 1px solid var(--surface4);".to_string()})
                        })
                    }
                }
                h3 { class: "parameters", "more parameters" }
                ul {
                    {
                        p2
                        .attributes
                        .into_iter()
                        .filter(|param| param.optional == true)
                        .into_iter()
                        .map(|param| {
                            rsx!(ObjectFieldComponent{field: param, styles : "border-block : 1px solid var(--surface4);".to_string()})
                        })
                    }
                }

                h3 { class: "returns", "Returns" }
                p { class : "success", style: "font-size : .8rem; line-height : 1.5; margin-left: 4em; margin-bottom: 5em; margin-top : auto;",
                    "{route.returns?.description?}"
                }
            }

            aside { class: "samples",
                section { class: "samples-section",
                    div { class: "request-example rad_shadow",
                        {
                            let mut first = route.method.split("/").collect::<Vec<&str>>();

                            let styling = match first.first().unwrap().trim().to_lowercase().as_str() {
                                "post" => "method-post",
                                "delete" => "method-delete",
                                "patch" => "method-patch",
                                _ => "method-default",
                            };

                            let method = first.remove(0);
                            let remainder = first.join("/");

                            rsx!(
                                a {
                                    class: "route-endpoint",
                                    style : "",
                                    href: "{url}",
                                    "data-title": "{route.name}",
                                    span { style: "margin-inline: .5em;", class: "{styling}", "{method}" }
                                    "/{remainder}"
                                }
                            )
                        }
                    }
                    pre { class: "response-example rad_shadow",
                        p { "Response" }
                        code {
                            class: "example-response",
                            dangerous_inner_html: pre_formatted
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub(crate) fn ServiceRouteComponentNoParams(
    route: crate::models::service::ApiRouteDocumentation,
    url: String,
) -> Element {
    let ex = r#"
    {
        "id": "di_1M6vk22eZvKYlo2CYMGIhk14",
        "object": "discount",
        "checkout_session": "cs_test_b1mywbZHtQCQW2ncaItVPFqupwmfqNU4IMMdw3lArEBGt0QD0CZDrNQswR",
        "coupon": {
          "id": "wsd",
          "object": "coupon",
          "amount_off": null,
          "created": 1669116350,
          "currency": null,
          "duration": "forever",
          "duration_in_months": null,
          "livemode": false,
          "max_redemptions": null,
          "metadata": {},
          "name": null,
          "percent_off": 10,
          "redeem_by": null,
          "times_redeemed": 1,
          "valid": true
        },
        "customer": "cus_9s6XKzkNRiz8i3",
        "end": null,
        "invoice": null,
        "invoice_item": null,
        "promotion_code": null,
        "start": 1669120702,
        "subscription": null
      }
    "#;
    let pre_formatted = syntax_highlight(ex);
    rsx!(
        section { class: "api-route rad_shadow",

            h2 {
                class: "heading",
                style: "color: var(--text1); font-weight : 600;",
                a {
                    style: "text-decoration : none;",
                    href: "{url}",
                    "data-title": "{route.name}",
                    "{route.name}"
                }
            }

            div { class: "attributes",
                p { class: "route-description", "{route.description?}" }

                h4 { class: "parameters", "Parameters" }

                ul { class: "parameter-list",
                    span { style: "font-size : .7rem; margin-left : 2em;", "No Parameters" }
                }
                h3 { class: "parameters", "more parameters" }
                ul {
                    span { style: "font-size : .7rem; margin-left : 2em;", "No Parameters" }
                }

                h3 { class: "returns", "Returns" }
                p { style: "font-size : .8rem; line-height : 1.5; margin-left: 4em; margin-bottom: 5em; margin-top : auto;",
                    "{route.returns?.description?}"
                }
            }

            aside { class: "samples",
                div { class: "request-example rad_shadow",
                    {
                        let mut first = route.method.split("/").collect::<Vec<&str>>();

                        let styling = match first.first().unwrap().trim().to_lowercase().as_str() {
                            "post" => "method-post",
                            "delete" => "method-delete",
                            "patch" => "method-patch",
                            _ => "method-default",
                        };

                        let method = first.remove(0);
                        let remainder = first.join("/");

                        rsx!(
                            a {
                                class: "route-endpoint",
                                style : "",
                                href: "{url}",
                                "data-title": "{route.name}",
                                span { style: "margin-inline: .5em;", class: "{styling}", "{method}" }
                                "/{remainder}"
                            }
                        )
                    }
                }
                pre { class: "response-example rad_shadow",
                    p { "Response" }
                    code {
                        class: "example-response",
                        dangerous_inner_html: pre_formatted
                    }
                }
            }
        }
    )
}

#[component]
fn ServiceEndpoints(endpoints: Vec<crate::models::service::Endpoint>) -> Element {
    rsx!(
        aside { class: "endpoints rad_shadow",
            h4 { "Endpoints" }
            hr {}
            {
                endpoints
                .iter()
                .map(|point| {
                    {
                        let mut first = point.route.split(" ").collect::<Vec<&str>>();

                        let styling = match first.first().unwrap().to_lowercase().as_str() {
                            "post" => "method-post",
                            "delete" => "method-delete",
                            "patch" => "method-patch",
                            _ => "method-default",
                        };

                        let method = first.remove(0);
                        let remainder = first.join("");

                        rsx!(
                            a {
                                class: "endpoint",
                                href: "{point.url}",
                                "data-title": "{point.name}",
                                span { style: " width : 7ch;", class: "{styling}", "{method}" }
                                "{remainder}"
                            }
                        )
                    }
                    .unwrap()
                })
            }
        }
    )
}

/// Component used to render the default object for each service.
#[component]
fn DefaultServiceObjectComponent(data: crate::models::service::Object, url: String) -> Element {
    let ex = r#"
    {
        "id": "di_1M6vk22eZvKYlo2CYMGIhk14",
        "object": "discount",
        "checkout_session": "cs_test_b1mywbZHtQCQW2ncaItVPFqupwmfqNU4IMMdw3lArEBGt0QD0CZDrNQswR",
        "coupon": {
          "id": "wsd",
          "object": "coupon",
          "amount_off": null,
          "created": 1669116350,
          "currency": null,
          "duration": "forever",
          "duration_in_months": null,
          "livemode": false,
          "max_redemptions": null,
          "metadata": {},
          "name": null,
          "percent_off": 10,
          "redeem_by": null,
          "times_redeemed": 1,
          "valid": true
        },
        "customer": "cus_9s6XKzkNRiz8i3",
        "end": null,
        "invoice": null,
        "invoice_item": null,
        "promotion_code": null,
        "start": 1669120702,
        "subscription": null
      }
    "#;
    let pre_formatted = syntax_highlight(ex);

    rsx!(
        section { class: "default-service-object rad_shadow",
            h2 {
                class: "heading",
                style: "color: var(--text1); font-weight : 600;",
                a { style: "text-decoration : none;", href: "{url}", "{data.description.as_ref()?}" }
            }
            div { class: "top-attributes",
                {data
                    .attributes
                    .iter()
                    .filter(|f| f.top_level == Some(true)).into_iter().map(|field|
                    rsx!(ObjectFieldComponent{field: field.clone(), styles : "margin-left: .4rem; border-top : 1px solid var(--surface4)".to_string()})
                )}
            }
            div { class: "more-attributes",
                h4 { class: "more-attributes-header", "More attributes" }
                {data
                    .attributes
                    .iter()
                    .filter(|f| f.top_level == None).into_iter().map(|field|
                    rsx!(ObjectFieldComponent{field: field.clone(), styles : "margin-left: .4rem; border-top : 1px solid var(--surface4)".to_string()})
                )}
            }

            aside { class: "object-shape rad_shadow",
                span { class: "example-header", "{data.description?}" }
                pre { class: "example-object",
                    code {
                        class: "example-response",
                        dangerous_inner_html: pre_formatted
                    }
                }
            }
        }
    )
}

#[component]
fn ObjectFieldComponent(field: crate::models::service::ObjectField, styles: String) -> Element {
    match field.r#type.clone() {
        crate::models::service::ItemDocs::Enum(en) => {
            rsx!(crate::components::enums::EnumDocsComponent {
                optional: field.optional,
                array: field.array,
                parent: field.name.clone(),
                en
            })
        }
        crate::models::service::ItemDocs::Object(obj) => {
            rsx!(crate::components::dictionary::DictionaryDocsComponent {
                optional: field.optional,
                array: field.array,
                parent: field.name.clone(),
                obj
            })
        }
        _ => rsx!(crate::components::field::FieldComponent {
            parent: field.name.clone(),
            data: field.clone().clone(),
            styles: "{styles}".to_owned()
        }),
    }
}
