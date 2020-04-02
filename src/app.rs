use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub template_registry: Arc<Handlebars<'static>>,
}

impl AppState {
    pub fn new(registry: Arc<Handlebars<'static>>) -> Self {
        Self {
            template_registry: registry,
        }
    }
}
