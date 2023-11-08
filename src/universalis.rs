use cached::proc_macro::cached;
use reqwest::get;
use serde::{Deserialize, Serialize};
use crate::universalis::UniversalisError::UniversalisConnectionFails;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum UniversalisError {
    InvalidWorldId,
    UniversalisConnectionFails
}
#[allow(dead_code)]
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Region {
    Japan,
    #[serde(rename = "North-America")]
    NorthAmerica,
    Europe,
    Oceania,
    // Thanks Yoshi-P for cool trilingual API... Why Japan is in english, is a mystery to this day.
    #[serde(rename = "中国")]
    China,
    #[serde(rename = "한국")]
    Korea
}
#[derive(Clone, Serialize, Deserialize)]
struct DatacenterMapping {
    name: String,
    region: Region,
    worlds: Vec<usize>
}
fn request_url_builder(endpoint: &str) -> String {
    "https://universalis.app/api/v2/".to_string() + endpoint
}
#[derive(Clone, Serialize, Deserialize)]
struct WorldMapping {
    id: usize,
    name: String
}
#[cached(time = 86400, sync_writes = true)]
async fn get_world_mappings() -> Result<Vec<WorldMapping>, UniversalisError> {
    let response = get(request_url_builder("worlds/")).await;
    match response {
        Ok(res) => Ok(res.json::<Vec<WorldMapping>>().await.unwrap()),
        Err(_) => Err(UniversalisConnectionFails)
    }
}
#[allow(dead_code)]
#[cached(time = 86400)]
async fn get_world_name(id: usize) -> Result<String, UniversalisError> {
    let map = get_world_mappings().await.unwrap();

    for world in map {
        if world.id == id {
            return Ok(world.name)
        }
    }
    Err(UniversalisError::InvalidWorldId)
}
#[allow(dead_code)]
#[cached(time = 86400)]
async fn get_world_id(name: String) -> Result<usize, UniversalisError> {
    let map = get_world_mappings().await.unwrap();

    for world in map {
        if world.name == name {
            return Ok(world.id)
        }
    }
    Err(UniversalisError::InvalidWorldId)
}
#[cached(time = 86400, sync_writes = true)]
async fn get_datacenter_mappings() -> Result<Vec<DatacenterMapping>, UniversalisError> {
    let response = get(request_url_builder("data-centers/")).await;
    match response {
        Ok(res) => Ok(res.json::<Vec<DatacenterMapping>>().await.unwrap()),
        Err(_) => Err(UniversalisConnectionFails)
    }
}

#[allow(dead_code)]
#[cached(time = 86400, sync_writes = true)]
async fn get_marketable_items() -> Result<Vec<usize>, UniversalisError> {
    let response = get(request_url_builder("marketable/")).await;
    match response {
        Ok(res) => Ok(res.json::<Vec<usize>>().await.unwrap()),
        Err(_) => Err(UniversalisConnectionFails)
    }
}
#[allow(dead_code)]
#[cached(time = 86400)]
async fn get_region_worlds(region: Region) -> Vec<String> {
    let datacenters = get_datacenter_mappings().await.unwrap();
    let mut output: Vec<String> = vec!();
    for dc in datacenters {
        if dc.region == region {
            for world in dc.worlds {
                output.push(get_world_name(world).await.unwrap())
            }
        }
    }
    output.sort();
    output
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_world_name() {
        assert_eq!(
            get_world_name(55).await.expect("get_world_name() failed!"),
            "Lamia"
        );
    }
    #[tokio::test]
    async fn test_get_world_id() {
        assert_eq!(
            get_world_id("Mateus".to_string()).await.expect("get_world_id() failed!"),
            37
        );
    }
    #[tokio::test]
    async fn test_get_marketable_items() {
        // I dunno, theres a lot of marketable items, so 10000 seemed like a big enough number.
        // The exact number changes borderline every patch anyways, this just tests that the
        // function actually works, and that the endpoint still exists.
        assert!(get_marketable_items()
            .await.expect("get_marketable_items() failed!").len() > 10000);
    }
    #[tokio::test]
    async fn test_get_region_worlds() {
        let mut correct = vec![
            "Cerberus".to_string(),
            "Louisoix".to_string(),
            "Moogle".to_string(),
            "Omega".to_string(),
            "Phantom".to_string(),
            "Ragnarok".to_string(),
            "Sagittarius".to_string(),
            "Spriggan".to_string(),
            "Alpha".to_string(),
            "Lich".to_string(),
            "Odin".to_string(),
            "Phoenix".to_string(),
            "Raiden".to_string(),
            "Shiva".to_string(),
            "Twintania".to_string(),
            "Zodiark".to_string()
        ];
        correct.sort();
        assert_eq!(get_region_worlds(Region::Europe).await, correct);
    }

}
