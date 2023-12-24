use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HTTPAuth {
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<Box<super::BasicAuth>>,

    #[serde(rename = "clientCert", skip_serializing_if = "Option::is_none")]
    pub client_cert: Option<Box<super::ClientCertAuth>>,

    #[serde(rename = "oauth2", skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<Box<super::OAuth2Auth>>,
}

impl HTTPAuth {
    pub fn new() -> HTTPAuth {
        HTTPAuth {
            basic_auth: None,
            client_cert: None,
            oauth2: None,
        }
    }
}
