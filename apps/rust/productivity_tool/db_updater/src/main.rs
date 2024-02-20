use env_loader::load::load;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

use es_driver::ElasticClient;
use event_entity::EventEntity;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use std::env;

#[derive(Serialize, Debug, Deserialize)]
struct MyDocument {
    title: String,
    body: String,
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
impl MyDocument {
    pub fn new() -> Self {
        Self {
            body: "this is body".to_string(),
            title: "this is title".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    load(".env").expect("failed to load .env file");
    let url = env::var("ELASTIC_FULL_URL").unwrap();
    let config = set_kafka_config();
    let kafka_client = create_kafka_client(&config);
    let es_client = ElasticClient::new(&url);
    loop {
        kafka_client.subscribe(&["event"]);
        match kafka_client.get_message().await {
            None => {}
            Some(msg) => match EventEntity::from_bytes(&msg) {
                Ok(event) => es_client.index("test", &event).await.expect("Do something"),
                Err(_) => {}
            },
        }
    }
    // let client = Client::new();
    // let resp = client.get(&url).send().await;
    // if let Ok(r) = resp {
    //     println!("{:?}", r);
    //     println!("{:?}", r.text().await);
    // }
    // match index_document(&client, "test", &MyDocument::new(), &url).await {
    //     Ok(_) => {}
    //     Err(err) => println!("{err}"),
    // }
    // match search_documents(&client, "test", "*", &url).await {
    //     Ok(docs) => {
    //         println!("{:?}", docs);
    //     }
    //     Err(err) => println!("{:?}", err),
    // }
}

fn set_kafka_config() -> KafkaClientConfig {
    let bootstrap = env::var("KAFKA_URL").unwrap();
    let user = env::var("KAFKA_USER").unwrap();
    let pass = env::var("KAFKA_PASS").unwrap();
    let group_id = env::var("GROUP_ID").unwrap();
    KafkaClientConfig::new(bootstrap, user, pass, group_id)
}

fn create_kafka_client(config: &KafkaClientConfig) -> KafkaConsumerClient {
    KafkaConsumerClient::connect(config)
}
async fn index_document(
    client: &Client,
    index: &str,
    document: &MyDocument,
    base_url: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/{}/_doc", base_url, index);
    client
        .post(url)
        .json(document)
        .send()
        .await?
        .error_for_status()?; // Checks if the response status is an error.
    Ok(())
}

async fn search_documents(
    client: &Client,
    index: &str,
    query: &str,
    base_url: &str,
) -> Result<Vec<MyDocument>, reqwest::Error> {
    let url = format!("{}/{}/_search", base_url, index);
    let response = client
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
        .json::<SearchResult<MyDocument>>()
        .await?;

    Ok(response
        .hits
        .hits
        .into_iter()
        .map(|hit| hit._source)
        .collect())
}
