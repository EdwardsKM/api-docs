#![allow(non_snake_case)]

use crate::components::pagenotfound::PageNotFound;
use crate::components::services::{Endpoint, Service, EndpointNested};
use components::layout::RootLayout;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::Level;
pub mod components;
pub mod models;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(RootLayout)]
    #[route("/")]
    Home {},
    #[route("/:name")]
    Service { name : String},
    #[route("/:service/:endpoint")]
    Endpoint { service : String, endpoint : String},
    #[route("/:service/:endpoint/:nested")]
    EndpointNested { service : String, endpoint : String, nested : String},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound {route: Vec<String>,},
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    let cfg = server_only!(
        dioxus::fullstack::Config::new().addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8234)))
    );
    let docs = serde_json::from_str::<ApiDocumentation>(include_str!("../api-docs.json")).unwrap();

    LaunchBuilder::fullstack()
        .with_cfg(cfg)
        .with_context(docs)
        .launch(App);
}

#[component]
fn Home() -> Element {
    let service_list = use_context::<ApiDocumentation>()
        .services
        .iter()
        .map(|s| {
            let url = s.default_object_url.split("/").collect::<Vec<&str>>();
            let name = url[0..(url.len() - 1)].join("/");
            name
        })
        .collect::<Vec<String>>();
    rsx! {
        div { class: "main-body",
            {service_list.into_iter().map(|name| rsx! {
                crate::components::services::Service {name}
            })}
        }
    }
}

//   let full_route = use_route::<Route>(cx).unwrap();
// #[derive(PartialEq, Clone, Props)]
// struct RootProps {
//     docs: ApiDocumentation,
// }

// fn main() {
//     let json_file = std::fs::read("api-docs.json").unwrap();

//     let endpoints: ApiDocumentation = serde_json::from_slice(&json_file).unwrap();

//     let mut dom = VirtualDom::new_with_props(
//         Home,
//         HomeProps {
//             cx: RootProps { docs: endpoints },
//         },
//     );

//     dom.rebuild(&mut dioxus_core::NoOpMutations);

//     let text = dioxus_ssr::render(&dom);
// }

// #[component]
// fn Home() -> Element {
//     let mut docs: Signal<ApiDocumentation> = use_signal(|| ApiDocumentation::default());

//     use_future(move || async move {
//         if let Ok(data) = get_api_docs_data().await {
//             info!("Client received endpoint data");
//             docs.set(data.clone());
//         }
//     });
// }

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_api_docs_data() -> Result<ApiDocumentation, ServerFnError> {
    let json_file = std::fs::read("api-docs.json").unwrap();
    let endpoints: ApiDocumentation = serde_json::from_slice(&json_file).unwrap();
    Ok(endpoints)
}

#[derive(Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ApiDocumentation {
    /// The crate version on the gateway object
    pub(crate) gateway_version: String,
    /// The version of the models used to build this documentation.
    /// The models can be updated without updating the gateway endpoints hence the need to track versions.
    pub(crate) models_version: String,
    /// The buld date of the documentation
    pub(crate) build_date: u64,
    /// The services present in this doc release.
    pub(crate) services: Vec<models::service::ServiceDocumentation>,
    /// The service-hierarhcy presented in the sidebar.
    pub(crate) sidebar: crate::models::sidebar::Sidebar,
}
