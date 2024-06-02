#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub(crate) fn Sidebar() -> Element {
    let data = use_context::<crate::ApiDocumentation>().sidebar;
    rsx!(
    aside { class: "sidebar",
              nav { style : "width: inherit;",
                  ul {
                      {data.services.iter().map(|sidebar_item|   match sidebar_item {
                          crate::models::sidebar::SidebarItem::Group(group) => rsx! {
                          ul { class : "sidebar-service-group", "{group.name}"
                              {
                                  group.services.iter().map(|service| {
                                      rsx!(li { class : "sidebar-service-group-item",
                                          SidebarServiceGroup {data : service.clone()}
                                      })
                                  })
                              }
                          }
                          },
                          crate::models::sidebar::SidebarItem::Service(service) => rsx! {
                              li { class : "sidebar-service-item",
                                  SidebarService {data : service.clone()}
                              }
                          },
                      })
                      }
                  }
              }
          }
    )
}

/// A component representing an api endpoint group in the sidebar.
#[component]
fn SidebarService(data: crate::models::sidebar::Service) -> Element {
    let mut open = use_signal(|| false);

    let styling = match open() {
        true => "active",
        false => "",
    };

    let toggle_dropdown = match open() {
        true => false,
        false => true,
    };

    rsx!(
        button {
            class: "sidebar-service {styling}",
            onclick: move |_| { open.set(toggle_dropdown) },
            "{data.name}"
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M6.29289 8.79289C6.68342 8.40237 7.31658 8.40237 7.70711 8.79289L12 13.0858L16.2929 8.79289C16.6834 8.40237 17.3166 8.40237 17.7071 8.79289C18.0976 9.18342 18.0976 9.81658 17.7071 10.2071L12.7071 15.2071C12.3166 15.5976 11.6834 15.5976 11.2929 15.2071L6.29289 10.2071C5.90237 9.81658 5.90237 9.18342 6.29289 8.79289Z"
                }
            }
        }
        div { class: "dropdown-container",
            {data.endpoints.iter().map(|e| {
                let d = e.url.split("/").collect::<Vec<&str>>();
                rsx!(
                    Link {
                        prevent_default : true,
                        onclick : move |event : MouseEvent| {
                            
                        },
                        new_tab : false,
                        class : "sidebar-endpoint",
                        active_class : "active",
                        id: "link_id",
                        to: crate::Route::Endpoint {
                            service : d[1].replace(" ", "").to_owned(),
                            endpoint : d[2..].join("/")
                        },
                        "{e.name}"}
                )
            })}
        }
    )
}

/// A component representing an api endpoint group in the sidebar.
#[component]
fn SidebarServiceGroup(data: crate::models::sidebar::Service) -> Element {
    let mut open = use_signal(|| false);

    let styling = match open.read().clone() {
        true => "active",
        false => "",
    };

    let toggle_dropdown = match open.read().clone() {
        true => false,
        false => true,
    };

    // This should probably be wrapped in a <section></section> element
    rsx!(
        button {
            class: "dropdown-btn {styling}",
            onclick: move |_| { open.set(toggle_dropdown) },
            "{data.name}"
            svg {
                width: "100%",
                height: "100%",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M6.29289 8.79289C6.68342 8.40237 7.31658 8.40237 7.70711 8.79289L12 13.0858L16.2929 8.79289C16.6834 8.40237 17.3166 8.40237 17.7071 8.79289C18.0976 9.18342 18.0976 9.81658 17.7071 10.2071L12.7071 15.2071C12.3166 15.5976 11.6834 15.5976 11.2929 15.2071L6.29289 10.2071C5.90237 9.81658 5.90237 9.18342 6.29289 8.79289Z"
                }
            }
        }
        div { class: "dropdown-container",
            {data.endpoints.iter().map(|e| {
                let d = e.url.split("/").collect::<Vec<&str>>();
                rsx!(
                    Link {
                        prevent_default : true,
                        onclick : move |event : MouseEvent| {
                            
                        },
                        new_tab : false,
                        class : "sidebar-endpoint",
                        active_class : "active",
                        id: "link_id",
                        to: crate::Route::Endpoint {
                            service : d[1].replace(" ", "").to_owned(),
                            endpoint :d[2..].join("/")
                        },
                    "{e.name}"}
                )
            })}
        }
    )
}
