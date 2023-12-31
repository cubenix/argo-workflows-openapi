use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// GCSArtifact is the location of a GCS artifact.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GCSArtifact {
    /// Bucket is the name of the bucket
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,

    /// Key is the path in the bucket where the artifact resides
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "serviceAccountKeySecret", skip_serializing_if = "Option::is_none")]
    pub service_account_key_secret: Option<Box<core::v1::SecretKeySelector>>,
}

impl GCSArtifact {
    pub fn new(key: String) -> GCSArtifact {
        GCSArtifact {
            bucket: None,
            key,
            service_account_key_secret: None,
        }
    }
}
