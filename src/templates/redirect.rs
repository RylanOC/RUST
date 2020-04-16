use handlebars::{Handlebars, RenderError};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Redirect {
    destination: String,
}

impl Redirect {
    /// Construct a new redirect template.
    pub fn new(dest: impl Into<String>) -> Self {
        Self {
            destination: dest.into(),
        }
    }

    /// Render a curtain to a webpage.
    pub fn render(&self, registry: &Handlebars) -> Result<String, RenderError> {
        registry.render("redirect", self)
    }
}
