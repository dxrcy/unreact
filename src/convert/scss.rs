use css_minify::optimizations as css_minify;

use crate::Error;

/// Convert SCSS file to CSS, and minify
pub fn scss_to_css(name: &str, scss: &str, minify: bool) -> Result<String, Error> {
    // Convert scss to css
    let css = try_unwrap!(
        grass::from_string(scss, &Default::default()),
        else Err(err) => return fail!(ScssConvert, name.to_string(), err),
    );

    // Minify
    if minify {
        return Ok(try_unwrap!(
            css_minify::Minifier::default().minify(&css, css_minify::Level::Two),
            else Err(err) => return fail!(CssMinify, name.to_string(), err.to_string()),
        ));
    }

    // Don't minify
    Ok(css)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scss_to_css_works() {
        let scss = "\
$foo: white;

body {
    background-color: black;

    p {
        color: $foo;
    }
}
";

        let css_large = scss_to_css("no_name", scss, false).expect("Should have parsed");

        assert_eq!(
            css_large,
            "\
body {
  background-color: black;
}
body p {
  color: white;
}
"
        );

        let css_mini = scss_to_css("no_name", scss, true).expect("Should have parsed");

        assert_eq!(css_mini, "body{background:black}body p{color:white}");
    }
}
