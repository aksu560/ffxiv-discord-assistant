use std::env;
use cached::proc_macro::{cached};
use serde::Deserialize;
use reqwest::get;

#[allow(dead_code)]
struct XivConfig {
    api_key: String
}
impl Default for XivConfig {
    fn default() -> Self {
        Self {
            api_key: get_api_key(),
        }
    }
}

#[cached(time=86400, sync_writes=true)]
fn get_api_key() -> String {
    env::var("XIV_TOKEN").expect("Missing XIV Token")
}

fn request_url_builder(endpoint: &str, item: String) -> String {
    endpoint.to_string() + &*item + "?private_key=" + &*get_api_key()
}

#[derive(Deserialize)]
struct ItemData {
    #[serde(rename="Name_en")]
    name_en: String
}

#[cached(time=86400, sync_writes=true)]
async fn id2name(id: String) -> String {
    let response = get(request_url_builder("https://xivapi.com/item/", id)).await;
    match response {
        Ok(r) => r.json::<ItemData>().await.unwrap().name_en,
        Err(..) => "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_item2id() {
        assert_eq!(
            id2name("5503".to_string()).await,
            "Animal Glue"
        );
    }
    #[tokio::test]
    async fn test_item2id_again() {
        assert_eq!(
            id2name("5503".to_string()).await,
            "Animal Glue"
        );
    }

}