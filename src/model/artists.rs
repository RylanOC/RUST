use rspotify::model::artist::FullArtist;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArtistsVec {
    items: Vec<FullArtist>
}