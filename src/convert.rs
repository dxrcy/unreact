use handlebars::Handlebars;

use crate::{server, throw, FileMap, Object, Page, Result, Value};

pub fn scss_to_css(name: &str, scss: &str) -> Result<String> {
    match grass::from_string(scss, &Default::default()) {
        Ok(css) => Ok(css),
        Err(err) => throw!(
            "SCSS to CSS Error! Problem with scss file '{}' `{:?}`",
            name,
            err
        ),
    }
}

pub(crate) fn render_page(
    registry: &mut Handlebars,
    name: &str,
    page: &Page,
    globals: Object,
    is_dev: bool,
) -> Result<String> {
    let mut rendered = match page {
        Page::Raw(page) => page.to_string(),

        Page::Template { template, data } => {
            // Add global variables
            let mut data = data.clone();
            data.insert("GLOBAL".to_string(), Value::Object(globals));

            // Render template
            match registry.render(template, &data) {
                Ok(rendered) => rendered,
                Err(err) => throw!("Handlebars failed! Rendering '{}' `{:?}`", name, err),
            }
        }
    };

    // Add dev script to file
    if is_dev {
        rendered += "\n\n";
        rendered += server::DEV_SCRIPT;
    }

    Ok(rendered)
}

pub fn register_templates(registry: &mut Handlebars, templates: FileMap) -> Result {
    for (name, template) in templates {
        if let Err(err) = registry.register_partial(&name, template) {
            throw!(
                "Handlebars error! Registering template '{}', `{:?}`",
                name,
                err
            );
        }
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
        if let Err(err) = registry.register_partial(name, template) {
            throw!(
                "Handlebars error! Registering inbuilt template '{}', `{:?}`",
                name,
                err
            );
        }
    }

    Ok(())
}
