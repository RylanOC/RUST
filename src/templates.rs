use handlebars::{Handlebars, RenderError};

/// Curtain page template structure.
#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Curtain {
    /// Title of the page displayed in the tab bar.
    pub page_title: String,
    /// Large text main title at center of page in big font.
    pub title: String,
    /// Smaller subtitle under the title.
    pub subtitle: String,
}

impl Curtain {
    /// Construct a new Page with empty fields. Can be used in 'builder' pattern.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page title of this curtain page.
    pub fn page_title(mut self, page_title: impl Into<String>) -> Self {
        self.page_title = page_title.into();
        self
    }

    /// Builder pattern function to set the title in this curtain.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Builder pattern function to set the subtitle in this curtain/
    pub fn subtitle(mut self, sub: impl Into<String>) -> Self {
        self.subtitle = sub.into();
        self
    }

    /// Render a curtain to a webpage.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("curtain", self)
    }
}