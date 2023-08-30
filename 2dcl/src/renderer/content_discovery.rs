#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
use dcl_common::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct XML {
    channel: Channel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Channel {
    #[serde(rename = "item")]
    scenes: Vec<SceneDiscoveryData>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SceneDiscoveryData {
    pub title: String,
    pub link: String,
    pub pub_date: String,
}

#[tokio::main]
pub async fn find_2d_scenes() -> Result<Vec<SceneDiscoveryData>> {
    let response = reqwest::get("https://2dcl.org/scenes.rss")
        .await?
        .text()
        .await?;
    let xml: XML = serde_xml_rs::from_str(&response).unwrap();

    Ok(xml.channel.scenes)
}
