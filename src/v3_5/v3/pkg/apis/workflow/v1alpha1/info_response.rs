use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InfoResponse {
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<super::Column>>,

    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<super::Link>>,

    #[serde(rename = "managedNamespace", skip_serializing_if = "Option::is_none")]
    pub managed_namespace: Option<String>,

    #[serde(rename = "modals", skip_serializing_if = "Option::is_none")]
    pub modals: Option<HashMap<String, bool>>,

    #[serde(rename = "navColor", skip_serializing_if = "Option::is_none")]
    pub nav_color: Option<String>,
}
