use serde::{Deserialize, Serialize};

use k8s_openapi::api::core;

/// ArtifactoryArtifact is the location of an artifactory artifact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactoryArtifact {
    #[serde(rename = "passwordSecret", skip_serializing_if = "Option::is_none")]
    pub password_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// URL of the artifact
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "usernameSecret", skip_serializing_if = "Option::is_none")]
    pub username_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl ArtifactoryArtifact {
    /// ArtifactoryArtifact is the location of an artifactory artifact
    pub fn new(url: String) -> ArtifactoryArtifact {
        ArtifactoryArtifact {
            password_secret: None,
            url,
            username_secret: None,
        }
    }
}
