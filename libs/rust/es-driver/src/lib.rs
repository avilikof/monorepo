use reqwest::{Client, Error, Response};
use serde::Deserialize;
use serde_json::Value;

pub struct ElasticClient {
    client: Client,
    url: String,
}

#[derive(Deserialize, Debug)]
struct SearchResult<T> {
    hits: Hits<T>,
}

#[derive(Deserialize, Debug)]
struct Hits<T> {
    hits: Vec<Hit<T>>,
}

#[derive(Deserialize, Debug)]
struct Hit<T> {
    _source: T,
}

impl ElasticClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: url.to_string(),
        }
    }
    pub async fn index(&self, index: &str, doc: Vec<u8>) -> Result<Response, Error> {
        let ful_url = format!("{}/{}/_doc", self.url, index);
        self.client
            .post(ful_url)
            .header("Content-Type", "application/json")
            .body(doc)
            .send()
            .await
    }

    pub async fn search(&self, index: &str, query: &str) -> Result<Vec<Value>, reqwest::Error> {
        let url = format!("{}/{}/_search", self.url, index);
        let response = self
            .client
            .post(url)
            .json(&serde_json::json!({
                "query": {
                    "query_string": {
                        "query": query
                    }
                }
            }))
            .send()
            .await?
            .error_for_status()?
            .json::<SearchResult<Value>>()
            .await?;

        Ok(response
            .hits
            .hits
            .into_iter()
            .map(|hit| hit._source)
            .collect())
    }
}
