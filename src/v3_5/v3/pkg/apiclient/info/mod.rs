pub struct Client<'a>{
    config: &'a super::Config,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a super::Config) -> Self {
        Client { config }
    }

    pub fn get_info(&self) -> () {
        let uri = format!("{}/api/v1/info", self.config.base_path());
        let mut req_builder = self.config
            .http_client()
            .request(reqwest::Method::GET, uri.as_str());

        /* if let Some(ref user_agent) = self.config.user_agent {
            println!("setting user_agent: {}", user_agent);
            req_builder = req_builder
                .header(reqwest::header::USER_AGENT, user_agent.clone());
        } */

        req_builder = req_builder
            .header(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static("application/json"))
            .header("Authorization", self.config.bearer_token().unwrap());

        let req = req_builder.build().unwrap();
        let res = self.config.http_client().execute(req);
        match res {
            Ok(r) => println!("SUCCESS:\n{:#?}", r),
            Err(e) => println!("ERROR:\n{:#?}", e),
        }

        // let status = res.status();
        // let content = res.text();

        // println!("status: {}\ncontent: {}", status, content);
    }
}

