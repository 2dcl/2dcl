#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
use dcl_common::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Xml {
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

pub async fn find_2d_scenes_str() -> Result<String> {
    match find_2d_scenes().await {
        Ok(scenes) => {
            let mut output = "Scene\t|\tParcel\t|\tLast Update\n\n".to_string();
            for scene in scenes {
                let splitted_link: Vec<&str> = scene.link.split('/').collect();

                let parcel = match splitted_link.len() >= 2 {
                    true => format!(
                        "{} , {}",
                        splitted_link[splitted_link.len() - 2],
                        splitted_link.last().unwrap()
                    ),
                    false => String::default(),
                };
                output += &format!("{}\t|\t{}\t|\t{}\n", scene.title, parcel, scene.pub_date);
            }
            Ok(output)
        }
        Err(err) => Ok(format!("{}", err)),
    }
}

pub async fn find_2d_scenes() -> Result<Vec<SceneDiscoveryData>> {
    let response = reqwest::get("https://2dcl.org/scenes.rss")
        .await?
        .text()
        .await?;
    let xml: Xml = serde_xml_rs::from_str(&response)?;
    Ok(xml.channel.scenes)
}
