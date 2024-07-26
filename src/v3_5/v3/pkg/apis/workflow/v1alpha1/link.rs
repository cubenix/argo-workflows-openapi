use serde::{Deserialize, Serialize};

/// A link to another app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    /// The name of the link, E.g. \"Workflow Logs\" or \"Pod Logs\"
    #[serde(rename = "name")]
    pub name: String,

    /// \"workflow\", \"pod\", \"pod-logs\", \"event-source-logs\", \"sensor-logs\", \"workflow-list\" or \"chat\"
    #[serde(rename = "scope")]
    pub scope: String,

    /// The URL. Can contain \"${metadata.namespace}\", \"${metadata.name}\",
    /// \"${status.startedAt}\", \"${status.finishedAt}\" or any other element
    /// in workflow yaml, e.g.
    /// \"${io.argoproj.workflow.v1alpha1.metadata.annotations.userDefinedKey}\"
    #[serde(rename = "url")]
    pub url: String,
}

impl Link {
    pub fn new(name: String, scope: String, url: String) -> Self {
        Link { name, scope, url }
    }
}
