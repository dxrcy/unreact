mod css;
pub use css::scss_to_css;

use handlebars::Handlebars;

use crate::{Error, FileMap, Object, Page, Value};

/// Render a page, using either a Handlebars template or a raw string, and minify
pub(crate) fn render_page(
    registry: &mut Handlebars,
    page: &Page,
    globals: Object,
    #[allow(unused_variables)] is_dev: bool,
    minify: bool,
) -> Result<String, Error> {
    let mut rendered = match page {
        Page::Raw(page) => page.to_string(),

        Page::Template { template, data } => {
            // Add global variables
            let mut data = data.clone();
            data.insert("GLOBAL".to_string(), Value::Object(globals));

            // Render template
            try_unwrap!(
                registry.render(template, &data),
                else Err(err) => return fail!(RenderTemplate, template.to_string(), Box::new(err)),
            )
        }
    };

    // Minify before adding dev script
    if minify {
        let config = minify_html::Cfg {
            do_not_minify_doctype: true,
            keep_comments: true,
            keep_html_and_head_opening_tags: true,
            keep_closing_tags: true,
            minify_css: true,
            minify_js: true,
            ..minify_html::Cfg::default()
        };

        rendered =
            String::from_utf8_lossy(&minify_html::minify(rendered.as_bytes(), &config)).to_string()
    }

    // Add dev script to file
    #[cfg(feature = "dev")]
    if is_dev {
        rendered += "\n\n";
        rendered += crate::server::DEV_SCRIPT;
    }

    Ok(rendered)
}

/// Register custom Handlebars templates onto registry
pub fn register_templates(registry: &mut Handlebars, templates: FileMap) -> Result<(), Error> {
    for (name, template) in templates {
        try_unwrap!(
            registry.register_partial(&name, template),
            else Err(err) => return fail!(RegisterTemplate, name, Box::new(err)),
        );
    }

    Ok(())
}

/// Inbuilt templates (partials)
const PARTIALS: &[(&str, &str)] = &[
    // Local link
    ("LINK", include_str!("partials/LINK.hbs")),
    // Local css style tag
    ("CSS", include_str!("partials/CSS.hbs")),
    // Local image icon
    ("ICON", include_str!("partials/ICON.hbs")),
    // Boilerplate meta tags
    ("META", include_str!("partials/META.hbs")),
];

/// Register inbuilt Handlebars templates (partials) onto registry
pub fn register_partials(registry: &mut Handlebars, url: &str) -> Result<(), Error> {
    // Url partial (not const)
    try_unwrap!(
        registry.register_partial("URL", url),
        else Err(err) => return fail!(RegisterInbuiltTemplate, "URL".to_string(), Box::new(err)),
    );

    // Rest of inbuilt partials (const)
    for (name, template) in PARTIALS {
        try_unwrap!(
            registry.register_partial(name, template),
            else Err(err) => return fail!(RegisterInbuiltTemplate, name.to_string(), Box::new(err)),
        );
    }

    Ok(())
}
