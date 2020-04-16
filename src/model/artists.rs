use crate::model::PagingObject;
use rspotify::model::artist::FullArtist;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ArtistsVec {
    #[serde(flatten)]
    pub paging_info: PagingObject,
    pub items: Vec<FullArtist>,
}
