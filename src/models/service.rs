use serde::{Deserialize, Serialize};

/// Represents the documentation for a publicly-facing service.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct ServiceDocumentation {
    /// The name of the service. eg. Products, Customers. etc.
    pub(crate) name: String,
    /// The description of the service. What it does and other interconnected services.
    pub(crate) description: Option<String>,
    /// The default object for this service.
    pub(crate) default: Option<Object>,
    /// The url of the default object returned by this service
    pub(crate) default_object_url: String,
    /// The routes under this service and thier documentation.
    pub(crate) routes: Vec<ApiRouteDocumentation>,
    /// A list of the routes in this service.
    pub(crate) endpoints: Vec<Endpoint>,
}

/// Represents one route in the service endpoints section
#[derive(serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Default, Debug)]
pub(crate) struct Endpoint {
    /// The name of the endpoint. This is derived from the function names.
    /// e.g. create an invoice.
    pub(crate) name: String,
    /// The api route. Eg. POST /v1/invoices
    pub(crate) route: String,
    /// The url of the endpoint's docs. eg. https://docs.fastapayments.com/api/invoices/create
    pub(crate) url: String,
}

/// Documentation for a Fastapayments public-facing api route.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct ApiRouteDocumentation {
    /// The http method of the route.
    pub(crate) method: String,
    /// Path params
    // pub(crate) path_params: RouteDetails,
    /// The name of the endpoint
    pub(crate) name: String,
    /// The description of the endpoint
    pub(crate) description: Option<String>,
    /// Parameters required by the api endpoint.
    pub(crate) parameters: Option<Object>,
    /// The return object on successful api route calls.
    /// Some calls only return a status code hence this field can be null.
    pub(crate) returns: Option<Object>,
}

/// The default structObject for a service
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Object {
    /// The docs on the default object
    pub(crate) description: Option<String>,
    /// The attributes|fields on th eobject
    pub(crate) attributes: Vec<ObjectField>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct ObjectField {
    /// The name of the struct-field
    pub(crate) name: String,
    /// Whether the attribute is a top-level attr. Used in rendering the default service object.
    pub(crate) top_level: Option<bool>,
    /// The docs on the field
    pub(crate) description: Option<String>,
    /// Whether the field is required. Placed at the top of attributes.
    pub(crate) optional: bool,
    /// Whether the field is expandable and its value can be used to get detailed data about a resource from another api.
    pub(crate) expandable: bool,
    /// Whether the field is an array
    pub(crate) array: bool,
    /// The type of the field's value. Can be a primitive eg bool, i32, string, an object, or an enum.
    pub(crate) r#type: ItemDocs,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RouteDetails {
    /// The method on the route.
    pub(crate) method: String,
    /// The path params on the route. Used to validate docs.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) params: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ItemDocs {
    Enum(LibraryEnumDocs),
    Object(LibraryStructDocs),
    Primitive(String),
    Timestamp,
    Number,
    String,
    Boolean,
    Binary,
}

/// The documentation object for a struct field.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) struct FieldDocs {
    /// The name of the field
    pub(crate) name: String,
    /// The field-level doc comments
    pub(crate) docs: Option<String>,
    /// Whether the field is optional
    pub(crate) optional: bool,
    /// Whether the field is an array
    pub(crate) array: bool,
    /// The type of the field. Can be a struct or enum.
    pub(crate) r#type: ItemDocs,
}

/// The fully fleshed out documentation object for a struct in the context of a crate
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) struct LibraryStructDocs {
    /// The name of the item
    #[serde(skip_deserializing)]
    pub(crate) name: String,
    /// The struct documentation
    pub(crate) docs: Option<String>,
    /// The fieds ids associated with this struct
    pub(crate) fields: Vec<FieldDocs>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) struct LibraryEnumDocs {
    // The name of the item
    #[serde(skip_deserializing)]
    pub(crate) name: String,
    // The enum documentation
    pub(crate) docs: Option<String>,
    // The variants associated with this enum and their documentation
    pub(crate) variants: Vec<VariantDocs>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(rename_all = "lowercase")]
pub(crate) struct VariantDocs {
    // The name of the field
    pub(crate) name: String,
    // The field-level doc comments
    pub(crate) docs: Option<String>,
}
