use crate::model::PagingObject;
use rspotify::model::track::FullTrack;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TracksVec {
    #[serde(flatten)]
    pub paging_info: PagingObject,
    pub items: Vec<FullTrack>,
}
