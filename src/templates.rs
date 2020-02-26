use handlebars::Handlebars;

use lazy_static::lazy_static;

/// Template name and path.
pub type Template = (&'static str, &'static str);

/// Top-level document template. includes HTMML declaration, language, and
/// charset definition. Fields include `title`, `head`, and `doc`.
pub const DOCUMENT_TEMPLATE: Template = ("document", "templates/document.hbs");

lazy_static!{
    pub static ref HANDLEBARS: Handlebars<'static> = {
        let mut h = Handlebars::new();
        h.set_strict_mode(true);
        h.register_template_file(DOCUMENT_TEMPLATE.0, DOCUMENT_TEMPLATE.1);
        h
    };
}
