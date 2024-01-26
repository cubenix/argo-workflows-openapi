#[derive(Debug)]
enum Protocol {
    HTTP,
    HTTPS,
}

#[derive(Debug)]
pub struct Host {
    address: String,
    port: u32,
    protocol: Protocol,
}

impl Host {
    pub fn address(&mut self, addr: String) -> &mut Host {
        self.address = addr;
        self
    }

    pub fn allow_insecure(&mut self) -> &mut Host {
        self.protocol = Protocol::HTTP;
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Host {
        self.port = port;
        self
    }

    pub fn as_string(&self) -> String {
        format!("{:?}://{}:{}", self.protocol, self.address, self.port)
    }
}

impl Default for Host {
    fn default() -> Self {
        Host {
            address: String::from("localhost"),
            port: 2746,
            protocol: Protocol::HTTPS,
        }
    }
}
