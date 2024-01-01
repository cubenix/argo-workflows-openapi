use reqwest::blocking::Client;

use host::Host;
mod host;

type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
struct APIKey {
    key: String,
    prefix: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    host: Host,
    api_key: Option<APIKey>,
    basic_auth: Option<BasicAuth>,
    bearer_token: Option<String>,
    client: Client,
    oauth_access_token: Option<String>,
    user_agent: Option<String>,
}

impl Config {
    pub fn builder(&mut self) -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn base_path(&self) -> String {
        self.host.as_string()
    }

    pub fn bearer_token(&self) -> Option<String> {
        self.bearer_token.clone()
    }

    pub fn http_client(&self) -> &Client {
        &self.client
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: Host::default(),
            api_key: None,
            basic_auth: None,
            bearer_token: None, 
            client: Client::new(),
            oauth_access_token: None,
            user_agent: None,
        } 
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            config: Config::default(),
        }
    }

    pub fn build(self) -> Config {
       self.config 
    }

    pub fn bearer_access_token(&mut self, token: String) -> &mut ConfigBuilder {
        self.config.bearer_token = Some(token);
        self
    }

    pub fn allow_insecure(&mut self) -> &ConfigBuilder {
        self.config.host.allow_insecure();
        self
    }

    pub fn host_address(&mut self, addr: String) -> &ConfigBuilder {
        self.config.host.address(addr);
        self
    }

    pub fn host_port(&mut self, port: u32) -> &ConfigBuilder {
       self.config.host.port(port);
       self
    }
}
