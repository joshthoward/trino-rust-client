pub mod response;

use reqwest::{Client as ReqwestClient, Response};
use response::*;
use serde::de::DeserializeOwned;

pub struct Client {
    base_url: String,
    port: u32,
    user: String,
    http_client: ReqwestClient,
}

impl Client {
    pub fn new(base_url: &str, port: u32, user: &str) -> Client {
        Client {
            base_url: base_url.to_string(),
            port,
            user: user.to_string(),
            http_client: ReqwestClient::new(),
        }
    }

    // TODO:
    //  - Implement query cancellation i.e. DELETE to nextUri
    //  - Implement own errors
    //  - Implement paging
    //  - Add client builder
    //  - Can we remove `.clone()` below?
    pub async fn query<T>(&self, query_str: &str) -> Result<Vec<T>, reqwest::Error>
    where
        T: DeserializeOwned,
    {
        let mut response = self.initial_request(query_str).await?;
        let mut response_body: QueryResults = response.json().await?;

        let mut data = Vec::new();
        while let Some(next_uri) = response_body.next_uri {
            response = self.next_request(&next_uri).await?;
            response_body = response.json().await?;
            if let Some(rows) = response_body.data {
                data.append(
                    &mut rows
                        .iter()
                        .map(|x| serde_json::from_value(x.clone()).unwrap())
                        .collect(),
                );
            }
        }
        Ok(data)
    }

    async fn initial_request(&self, query_str: &str) -> Result<Response, reqwest::Error> {
        let conn_str = format!("{}:{}/v1/statement", &self.base_url, &self.port);
        self.http_client
            .post(conn_str)
            .header("X-Trino-User", &self.user)
            .body(query_str.to_string())
            .send()
            .await
    }

    async fn next_request(&self, next_uri: &str) -> Result<Response, reqwest::Error> {
        self.http_client.get(next_uri).send().await
    }
}
