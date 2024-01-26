use reqwest::blocking::Client;

use host::Host;
mod host;

#[derive(Debug)]
pub struct Config {
    host: Host,
    bearer_token: Option<String>,
    client: Client,
}

impl Config {
    pub fn builder(self) -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub fn base_path(&self) -> String {
        self.host.as_string()
    }

    pub fn bearer_access_token(&self) -> Option<String> {
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
            bearer_token: None, 
            client: Client::new(),
        } 
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    allow_insecure: bool,
    bearer_token: Option<String>,
    host_address: Option<String>,
    host_port: Option<u32>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn allow_insecure(&mut self) -> &mut Self {
        self.allow_insecure = true;
        self
    }

    pub fn bearer_access_token(&mut self, token: String) -> &mut Self {
        self.bearer_token = Some(token);
        self
    }

    pub fn host_address(&mut self, addr: String) -> &mut Self {
        self.host_address = Some(addr);
        self
    }

    pub fn host_port(&mut self, port: u32) -> &mut Self {
       self.host_port = Some(port);
       self
    }

    pub fn build(&self) -> Config {
        let mut config = Config::default();
        if let Some(ref token) = self.bearer_token {
            config.bearer_token = Some(token.to_owned())
        }

        if let Some(ref addr) = self.host_address {
            config.host.address(addr.to_owned());
        }

        if let Some(port) = self.host_port {
            config.host.port(port);
        }

        if self.allow_insecure {
            config.host.allow_insecure();
        }

        config 
    }
}
