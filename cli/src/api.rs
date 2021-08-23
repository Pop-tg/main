use crate::{Error, Req, Response, Result};

use log::debug;
use reqwest::{Client as ReqClient, StatusCode};
use url::Url;

pub struct Client {
    api_url: Url,
    client: ReqClient,
}

impl Client {
    const API: &'static str = "https://pop.tg/api/v2/";

    /// Initialize a new [`Client`].
    /// Will panic if [`Request::Client`] initialize failed
    pub fn new() -> Self {
        let api_url = Url::parse(Self::API).unwrap();
        let client = ReqClient::new();
        Self { api_url, client }
    }

    pub fn new_with_url(api_url: Url) -> Self {
        let client = ReqClient::new();
        Self { api_url, client }
    }

    pub async fn request<'a, R: Req>(self, req: R) -> Result<Response<R::Response>> {
        let url = self.api_url.join(R::METHOD_NAME).unwrap();
        let req_body = req.to_string()?;
        debug!("Calling <{}> with: {}", R::METHOD_NAME, req_body);
        let req = self.client.post(url).body(req_body);
        let ret = self
            .client
            .execute(req.build().map_err(Error::BadRequest)?)
            .await?;

        let status = ret.status();

        if status != StatusCode::OK && status != StatusCode::BAD_REQUEST {
            return Err(Error::RequestFailed(status, ret.text().await?));
        }

        let content = ret.text().await?;

        debug!("Got response from api: {}", content);

        serde_json::from_str(&content).map_err(Error::Parse)
    }
}

#[tokio::test]
async fn test() {
    use crate::model::*;
    use log::info;
    use simple_logger::init;

    init().unwrap();

    let client = Client::new();
    let res = client
        .request(GetRequest { key: "abc".into() })
        .await
        .unwrap();
    info!("{:?}", res);
}
