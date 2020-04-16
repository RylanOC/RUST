use crate::model::artists::ArtistsVec;
use crate::model::tracks::TracksVec;
use handlebars::{Handlebars, RenderError};

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ResultsPage {
    pub page_title: String,
    pub artists: ArtistsVec,
    pub tracks: TracksVec,
}

impl ResultsPage {
    /// Constructs a new ResultsPage, default the page title to results.
    pub fn new(artist_data: ArtistsVec, track_data: TracksVec) -> Self {
        Self {
            page_title: "Results".to_owned(),
            artists: artist_data,
            tracks: track_data,
        }
    }

    /// Builder pattern function to set title.
    pub fn title(mut self, new_title: impl Into<String>) -> Self {
        self.page_title = new_title.into();
        self
    }

    /// Render the results webpage.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("results", self)
    }
}
