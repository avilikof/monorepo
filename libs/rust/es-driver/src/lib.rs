use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

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
    pub async fn index<B: Serialize>(&self, index: &str, doc: &B) -> Result<(), Error> {
        let full_url = format!("{}/{}/_doc", self.url, index);
        match self.client.post(full_url).json(doc).send().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
