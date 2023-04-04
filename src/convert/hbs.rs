use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

use crate::{Error, FileMap, Object, Page, Port, Value};

/// Registry all [`Handlebars`](handlebars) partials, and helpers
///
/// NOT Templates
pub fn register_inbuilt(registry: &mut Handlebars, url: &str) -> Result<(), Error> {
    // Register inbuilt templates (partials)
    register_partials(registry)?;
    // Register inbuilt helpers
    register_helpers(registry, url);
    Ok(())
}

/// Render a page, using either a [`Handlebars`](handlebars) template or a raw string, and minify
pub(crate) fn render_page(
    registry: &mut Handlebars,
    page: &Page,
    globals: Object,
    minify: bool,
    // Only for "dev" feature
    #[allow(unused_variables)] is_dev: bool,
    #[allow(unused_variables)] port_ws: Port,
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
        rendered += &crate::server::dev_script(port_ws);
    }

    Ok(rendered)
}

/// Register custom [`Handlebars`](handlebars) templates onto registry
pub(crate) fn register_templates(
    registry: &mut Handlebars,
    templates: FileMap,
) -> Result<(), Error> {
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
    // More meta tags!
    ("META_EXTRA", include_str!("partials/META_EXTRA.hbs")),
];

/// Register const inbuilt [`Handlebars`](handlebars) templates (partials) onto registry
fn register_partials(registry: &mut Handlebars) -> Result<(), Error> {
    for (name, template) in PARTIALS {
        try_unwrap!(
            registry.register_partial(name, template),
            else Err(err) => return fail!(RegisterInbuiltTemplate, name.to_string(), Box::new(err)),
        );
    }

    Ok(())
}

/// Registers some as [`Handlebars`](handlebars) helpers, including `url`
fn register_helpers(registry: &mut Handlebars, url: &str) {
    // Url helper, returns url given
    let url = url.to_string();
    let closure = move |_: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
          -> HelperResult {
        out.write(&url)?;
        Ok(())
    };
    registry.register_helper("URL", Box::new(closure));

    // Concat helper, concatenates strings
    let closure = |helper: &Helper,
                   _: &Handlebars,
                   _: &Context,
                   _: &mut RenderContext,
                   out: &mut dyn Output|
     -> HelperResult {
        let params = helper.params();

        for param in params {
            out.write(param.value().render().as_ref())?;
        }

        Ok(())
    };
    registry.register_helper("concat", Box::new(closure));
}
