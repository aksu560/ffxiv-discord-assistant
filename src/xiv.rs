use cached::proc_macro::cached;
use reqwest::get;
use serde::Deserialize;
use std::env;

#[allow(dead_code)]
struct XivConfig {
    api_key: String,
}
#[allow(dead_code)]
impl Default for XivConfig {
    fn default() -> Self {
        Self {
            api_key: get_api_key(),
        }
    }
}

#[cached(time = 86400, sync_writes = true)]
fn get_api_key() -> String {
    env::var("XIV_TOKEN").expect("Please set your XIV_TOKEN environment variable")
}

fn request_url_builder(endpoint: &str, item: String) -> String {
    endpoint.to_string() + &*item + "?private_key=" + &*get_api_key()
}

#[derive(Deserialize, Default, Clone)]
#[allow(dead_code)]
struct ItemData {
    #[serde(rename = "Name_en")]
    name_en: String,
}
#[allow(dead_code)]
impl ItemData {
    pub async fn new(id: String) -> Self {
        get_item_data(id).await
    }
    pub fn get_name(&self) -> String {
        self.name_en.clone()
    }
}

#[cached(time = 86400, sync_writes = true)]
async fn get_item_data(id: String) -> ItemData {
    let response = get(request_url_builder("https://xivapi.com/item/", id)).await;
    match response {
        Ok(r) => r.json::<ItemData>().await.unwrap(),
        Err(..) => ItemData::default(),
    }
}
#[derive(Deserialize, Default, Clone)]
#[allow(dead_code)]
struct WorldName {
    #[serde(rename = "Name")]
    name: String
}

#[allow(dead_code)]
#[cached(time = 86400, sync_writes = true)]
async fn get_world_name(id: String) -> String {
    get(request_url_builder("https://xivapi.com/World/", id)).await.unwrap().json::<WorldName>().await.unwrap().name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item_data() {
        assert_eq!(
            ItemData::new("5503".to_string()).await.get_name(),
            "Animal Glue"
        );
    }

    #[tokio::test]
    async fn test_world_name() {
        assert_eq!(
            get_world_name("55".to_string()).await,
            "Lamia"
        )
    }
}
