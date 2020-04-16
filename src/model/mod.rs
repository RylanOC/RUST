use serde::{Serialize, Deserialize};

pub mod artists;
pub mod tracks;


/// See [paging object docs](https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object)
/// This is intended to be used via `#[serde(flatten)]` inside another struct.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PagingObject {
    pub href: String,
    pub limit: u32,
    pub next: Option<String>,
    pub offset: u32,
    pub previous: Option<String>,
    pub total: u32
}