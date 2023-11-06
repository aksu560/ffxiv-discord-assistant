use std::env;
use cached::proc_macro::{cached};
use serde::Deserialize;
use reqwest::{get};

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

#[derive(Deserialize, Default, Clone)]
#[allow(dead_code)]
struct ItemData {
    #[serde(rename="Name_en")]
    name_en: String
}

impl ItemData {
    pub fn get_name(&self) -> String {
        self.name_en.clone()
    }
}

#[cached(time=86400, sync_writes=true)]
async fn get_item_data(id: String) -> ItemData {
    let response = get(request_url_builder("https://xivapi.com/item/", id)).await;
    match response {
        Ok(r) => r.json::<ItemData>().await.unwrap(),
        Err(..) => ItemData::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item_data() {
        assert_eq!(
            get_item_data("5503".to_string()).await.get_name(),
            "Animal Glue"
        );
    }
}