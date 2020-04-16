// re-export timerange
pub use rspotify::senum::TimeRange;

/// For more info, see [api docs](https://developer.spotify.com/documentation/web-api/reference-beta/#category-personalization)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PersonalizationParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub time_range: Option<TimeRange>,
}

impl PersonalizationParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder pattern method to set the number of items to retrive;
    /// Returns none if limit is not in range 1 to 50 (inclusive).
    pub fn limit(mut self, limit: u32) -> Option<Self> {
        if limit > 50 || limit < 1 {None}
        else {
            self.limit = Some(limit);
            Some(self)
        }
    }

    /// Builder pattern to set the offset to retrieve.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builder patter to specify the time range to retrieve.
    pub fn time_range(mut self, time_range: TimeRange) -> Self {
        self.time_range = Some(time_range);
        self
    }

}