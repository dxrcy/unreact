/// Specifically for HBS->HTML template rendering
mod hbs;
/// Specifically for SCSS->CSS conversion
mod scss;

pub(crate) use hbs::{register_all, render_page};
pub(crate) use scss::scss_to_css;
