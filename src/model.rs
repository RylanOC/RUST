use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum ImageData {
    String,
    u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub external_urls: HashMap<String, String>,
    pub followers: HashMap<String, Option<u32>>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<HashMap<String, ImageData>>,
    pub name: String,
    pub popularity: u32,
    //pub type_str: String,
    pub uri: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    pub items: Vec<Artist>
}

//pub mod artist;
//pub mod track;
