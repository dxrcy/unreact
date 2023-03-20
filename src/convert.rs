use css_minify::optimizations as css_minify;
use handlebars::Handlebars;

use crate::{FileMap, Object, Page, Result, Value};

pub fn scss_to_css(name: &str, scss: &str, minify: bool) -> Result<String> {
    // Convert scss to css
    let css = try_unwrap!(
        grass::from_string(scss, &Default::default()),

        else Err(err) => {
            throw!(
                "SCSS to CSS Error! Problem with scss file '{}' `{:?}`",
                name,
                err
            )
        }
    );

    // Minify
    if minify {
        return Ok(try_unwrap!(
            css_minify::Minifier::default().minify(&css, css_minify::Level::Two),

            else Err(err) => {
                throw!(
                    "CSS minify error! Problem with scss file '{}' `{:?}`",
                    name,
                    err
                )
            }
        ));
    }

    // Don't minify
    return Ok(css);
}

pub(crate) fn render_page(
    registry: &mut Handlebars,
    name: &str,
    page: &Page,
    globals: Object,
    #[allow(unused_variables)] is_dev: bool,
    minify: bool,
) -> Result<String> {
    let mut rendered = match page {
        Page::Raw(page) => page.to_string(),

        Page::Template { template, data } => {
            // Add global variables
            let mut data = data.clone();
            data.insert("GLOBAL".to_string(), Value::Object(globals));

            // Render template
            try_unwrap!(
                registry.render(template, &data),
                else Err(err) => throw!("Handlebars failed! Rendering '{}' `{:?}`", name, err),
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
            ..minify_html::Cfg::default()
        };

        rendered =
            String::from_utf8_lossy(&minify_html::minify(rendered.as_bytes(), &config)).to_string()
    }

    // Add dev script to file
    #[cfg(feature = "watch")]
    if is_dev {
        rendered += "\n\n";
        rendered += crate::server::DEV_SCRIPT;
    }

    Ok(rendered)
}

pub fn register_templates(registry: &mut Handlebars, templates: FileMap) -> Result {
    for (name, template) in templates {
        try_unwrap!(
            registry.register_partial(&name, template),
            else Err(err) => throw!(
                "Handlebars error! Registering template '{}', `{:?}`",
                name,
                err
            )
        );
    }

    Ok(())
}

pub fn register_inbuilt_templates(registry: &mut Handlebars, url: &str) -> Result {
    let inbuilt_templates: &[(&str, &str)] = &[
        // Base url for site
        ("URL", url),
        // Simple style tag
        (
            "CSS",
            r#"<link rel="stylesheet" href="{{>URL}}/styles/{{name}}/style.css" />"#,
        ),
        // Simple link
        (
            "LINK",
            r#"<a href="{{>URL}}/{{to}}"> {{>@partial-block}} </a>"#,
        ),
    ];

    for (name, template) in inbuilt_templates {
        try_unwrap!(
            registry.register_partial(name, template),
            else Err(err) => throw!(
                "Handlebars error! Registering inbuilt template '{}', `{:?}`",
                name,
                err
            )
        );
    }

    Ok(())
}
