use std::fmt::Display;

use crate::{error::MyResult, Error};

/// Path for a `Route`
///
/// Vector of `Fragment`'s
#[derive(Debug, PartialEq)]
pub struct RoutePath(pub Vec<Fragment>);

impl Display for RoutePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}",
            self.0
                .iter()
                .map(|fragment| match fragment {
                    Fragment::Literal(literal) => format!("{}", literal),
                    Fragment::Value(value) => format!("<{}>", value),
                })
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

impl TryFrom<&str> for RoutePath {
    type Error = Error;

    fn try_from(filepath: &str) -> Result<Self, Self::Error> {
        route_path_from_filename(filepath)
    }
}

/// Fragment of filepath
#[derive(Debug, PartialEq)]
pub enum Fragment {
    /// Literal path fragment string
    Literal(String),
    /// Variable name
    Value(String),
}

/// Converts filename into `RoutePath`
fn route_path_from_filename(filepath: &str) -> MyResult<RoutePath> {
    let filepath = remove_hbs_extension(filepath)?;
    let filepath = replace_index_ident(filepath)?;
    Ok(RoutePath(split_fragments(filepath)?))
}

/// Removes `.hbs` extension from filepath
fn remove_hbs_extension(filepath: &str) -> MyResult<&str> {
    // Checks if is .hbs
    if !filepath.ends_with(".hbs") {
        throw!("File is not `.hbs` Handlebars file");
    }

    // Remove characters: back from end, until period
    let mut chars = filepath.chars();
    while chars.next_back() != Some('.') {}
    Ok(chars.as_str())
}

/// Replaces *index identifier* (tilde `~`), if at end of filename
fn replace_index_ident(filepath: &str) -> MyResult<&str> {
    // Filepath is ident
    if filepath == "~" {
        return Ok("");
    }

    // Ends with ident
    if filepath.ends_with("/~") {
        // Remove characters: back from end, until slash
        let mut chars = filepath.chars();
        while chars.next_back() != Some('/') {}
        return Ok(chars.as_str());
    }

    // Contains ident, not at end
    if filepath.contains('~') {
        throw!("Cannot use `~` index identifier in this position");
    }

    Ok(filepath)
}

/// Splits filepath into vector of fragments
fn split_fragments(filepath: &str) -> MyResult<Vec<Fragment>> {
    let mut fragments = Vec::new();

    // Split at slash
    for frag in filepath.split('/') {
        // Value or literal
        if frag.starts_with('$') {
            // Remove value ident from start
            let mut chars = frag.chars();
            chars.next();
            fragments.push(Fragment::Value(chars.as_str().to_string()));
        } else {
            // Value ident, not at start of fragment
            if frag.contains('$') {
                throw!("Cannot use `$` value identifier in this position")
            }
            fragments.push(Fragment::Literal(frag.to_string()));
        }
    }

    Ok(fragments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Fragment::*;

    #[test]
    fn route_path_from_filename_works() {
        let zero = vec![
            "~.hbs",
            "404.hbs",
            "hello/~.hbs",
            "hello/there.hbs",
            "news/$article.hbs",
        ];

        let mapped: Vec<_> = zero
            .into_iter()
            .flat_map(route_path_from_filename)
            .collect();

        assert_eq!(
            mapped,
            vec![
                RoutePath(vec![Literal("".to_string())]),
                RoutePath(vec![Literal("404".to_string())]),
                RoutePath(vec![Literal("hello".to_string())]),
                RoutePath(vec![
                    Literal("hello".to_string()),
                    Literal("there".to_string())
                ]),
                RoutePath(vec![
                    Literal("news".to_string()),
                    Value("article".to_string())
                ]),
            ],
        );
    }

    #[test]
    fn remove_hbs_extension_works() {
        let initial = vec![
            "~.hbs",
            "404.hbs",
            "hello/~.hbs",
            "hello/there.hbs",
            "news/$article.hbs",
        ];

        let mapped: Vec<_> = initial.into_iter().flat_map(remove_hbs_extension).collect();

        assert_eq!(
            mapped,
            vec!["~", "404", "hello/~", "hello/there", "news/$article"],
        );
    }

    #[test]
    fn replace_index_ident_works() {
        let initial = vec!["~", "404", "hello/~", "hello/there", "news/$article"];

        let mapped: Vec<_> = initial.into_iter().flat_map(replace_index_ident).collect();

        assert_eq!(
            mapped,
            vec!["", "404", "hello", "hello/there", "news/$article"],
        );
    }

    #[test]
    fn split_fragments_works() {
        let initial = vec!["", "404", "hello", "hello/there", "news/$article"];

        let mapped: Vec<_> = initial.into_iter().flat_map(split_fragments).collect();

        assert_eq!(
            mapped,
            vec![
                vec![Literal("".to_string())],
                vec![Literal("404".to_string())],
                vec![Literal("hello".to_string())],
                vec![Literal("hello".to_string()), Literal("there".to_string())],
                vec![Literal("news".to_string()), Value("article".to_string())],
            ],
        );
    }
}
