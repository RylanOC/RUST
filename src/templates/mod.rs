// to simplify the module structure, all the important structures
// are re-exported

pub mod curtain;
pub use curtain::Curtain;

pub mod redirect;
pub use redirect::Redirect;

pub mod p404;
pub use p404::P404;

pub mod results_page;
pub use results_page::ResultsPage;
