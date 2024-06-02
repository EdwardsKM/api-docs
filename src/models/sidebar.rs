use serde::{Deserialize, Serialize};

/// Data for generating a sidebar.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct Sidebar {
    /// The list of services to include in the sidebar.
    pub(crate) services: Vec<SidebarItem>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) enum SidebarItem {
    Group(ServiceGroup),
    Service(Service),
}

/// Represents a group of correlated api services. eg. identity.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct ServiceGroup {
    /// The name of the group.
    pub(crate) name: String,
    /// The list of services inside a sidebar service-group.
    pub(crate) services: Vec<Service>,
}

/// Represents a single api service
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct Service {
    /// The name of the service. eg quotes
    pub(crate) name: String,
    /// Short description of the service and its role.
    pub(crate) docs: Option<String>,
    /// An array of endpoints associated with the servcie.
    pub(crate) endpoints: Vec<ServiceEndpoint>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct ServiceEndpoint {
    /// The name of the endpoint. Eg. Create Invoice.
    pub(crate) name: String,
    /// A short description of the endpoint. Used to create tooltips.
    pub(crate) description: Option<String>,
    /// The url to the endpoint's documentation
    pub(crate) url: String,
}
