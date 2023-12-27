use serde::{Deserialize, Serialize};
use k8s_openapi::api::core;

/// HDFSArtifactRepository defines the controller configuration for an HDFS
/// artifact repository.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HDFSArtifactRepository {
    /// Addresses is accessible addresses of HDFS name nodes.
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,

    /// Force copies a file forcibly even if it exists.
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// HDFSUser is the user to access HDFS file system. It is ignored if
    /// either ccache or keytab is used.
    #[serde(rename = "hdfsUser", skip_serializing_if = "Option::is_none")]
    pub hdfs_user: Option<String>,

    #[serde(rename = "krbCCacheSecret", skip_serializing_if = "Option::is_none")]
    pub krb_c_cache_secret: Option<Box<core::v1::SecretKeySelector>>,

    #[serde(rename = "krbConfigConfigMap", skip_serializing_if = "Option::is_none")]
    pub krb_config_config_map: Option<Box<core::v1::ConfigMapKeySelector>>,

    #[serde(rename = "krbKeytabSecret", skip_serializing_if = "Option::is_none")]
    pub krb_keytab_secret: Option<Box<core::v1::SecretKeySelector>>,

    /// KrbRealm is the Kerberos realm used with Kerberos keytab.
    /// It must be set if keytab is used.
    #[serde(rename = "krbRealm", skip_serializing_if = "Option::is_none")]
    pub krb_realm: Option<String>,
    
    /// KrbServicePrincipalName is the principal name of Kerberos service.
    /// It must be set if either ccache or keytab is used.
    #[serde(rename = "krbServicePrincipalName", skip_serializing_if = "Option::is_none")]
    pub krb_service_principal_name: Option<String>,

    /// KrbUsername is the Kerberos username used with Kerberos keytab.
    /// It must be set if keytab is used.
    #[serde(rename = "krbUsername", skip_serializing_if = "Option::is_none")]
    pub krb_username: Option<String>,

    /// PathFormat is defines the format of path to store a file.
    /// Can reference workflow variables.
    #[serde(rename = "pathFormat", skip_serializing_if = "Option::is_none")]
    pub path_format: Option<String>,
}

impl HDFSArtifactRepository {
    pub fn new() -> HDFSArtifactRepository {
        HDFSArtifactRepository {
            addresses: None,
            force: None,
            hdfs_user: None,
            krb_c_cache_secret: None,
            krb_config_config_map: None,
            krb_keytab_secret: None,
            krb_realm: None,
            krb_service_principal_name: None,
            krb_username: None,
            path_format: None,
        }
    }
}
