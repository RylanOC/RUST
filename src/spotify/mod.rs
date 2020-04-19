use crate::auth::token_response::Tokens;
use actix_web::client::{Client, ClientRequest};
use actix_web::http::Uri;
use serde::de::DeserializeOwned;

// re-export parameter class and timerange
pub mod params;
pub mod charts;
pub use params::*;

const PERSONALIZATION_ENDPOINT: &'static str = "https://api.spotify.com/v1/me/top/";


#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum PersonalizationData {
    Artists,
    Tracks,
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
        format!("{}{}", PERSONALIZATION_ENDPOINT, self.get_endpoint_path())
            .parse()
            .unwrap()
    }

    /// Make a request to Spotify to get data.
    pub fn make_req(self, tokens: &Tokens, params: &PersonalizationParams) -> ClientRequest {
        let client = Client::default();
        client
            .get(self.get_endpoint())
            .bearer_auth(&tokens.access_token)
            .query(params)
            .unwrap()
    }

    /// Get a spotify data as deserialized json.
    pub async fn get_data<T: DeserializeOwned>(
        self,
        tokens: &Tokens,
        params: &PersonalizationParams,
    ) -> Result<T, String> {
        self.make_req(tokens, params)
            .send()
            .await
            .map_err(|err| err.to_string())?
            .json::<T>()
            .await
            .map_err(|e| e.to_string())
    }
}
