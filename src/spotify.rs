use actix_web::http::Uri;

const SPOTIFY_ENDPOINT: &'static str = "https://api.spotify.com/v1/me/top/";

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PersonalizationData {
    Artists,
    Tracks
}

impl PersonalizationData {
    fn get_endpoint_path(self) -> &'static str {
        use PersonalizationData::*;
        match self {
            Artists => "artists",
            Tracks => "tracks",
        }
    }

    /// Get the endpoint of Spotify's API.
    pub fn get_endpoint(self) -> Uri {
        format!("{}{}", SPOTIFY_ENDPOINT, self.get_endpoint_path()).parse().unwrap()
    }
}