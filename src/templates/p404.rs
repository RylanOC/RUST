use handlebars::{Handlebars, RenderError};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct P404 {
    destination: String
}

impl P404 {
    /// Construct a new 404 error template.
    pub fn new(dest: impl Into<String>) -> Self {
        Self {destination: dest.into()}
    }

    /// Render a curtain to a webpage.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("p404", self)
    }
}