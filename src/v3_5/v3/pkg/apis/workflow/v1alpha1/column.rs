use serde::{Deserialize, Serialize};

// Column is a custom column that will be exposed in the Workflow List View.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Column {
    /// The key of the label or annotation, e.g., \"workflows.argoproj.io/completed\".
    #[serde(rename = "key")]
    pub key: String,

    /// The name of this column, e.g., \"Workflow Completed\".
    #[serde(rename = "name")]
    pub name: String,

    /// The type of this column, \"label\" or \"annotation\".
    #[serde(rename = "type")]
    pub r#type: String,
}
