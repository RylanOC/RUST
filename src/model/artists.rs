use rspotify::model::artist::FullArtist;
use crate::model::PagingObject;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ArtistsVec {
    #[serde(flatten)]
    pub paging_info: PagingObject,
    pub items: Vec<FullArtist>
}