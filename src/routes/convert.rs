// use serde_json::json;

use crate::{
    error::MyResult,
    routes::{Fragment, Route},
    Object,
};

//TODO rename ?
#[derive(Debug, PartialEq)]
pub struct TemplateToRender {
    pub route: Route,
    pub paths: Vec<PathToRender>,
}

//TODO rename ?
#[derive(Clone, Debug, PartialEq)]
pub struct PathToRender {
    pub filepath: String,
    pub values: Object,
}

impl PathToRender {
    pub fn new() -> Self {
        Self {
            filepath: String::new(),
            values: Object::new(),
        }
    }
}

//TODO rename
//TODO docs
pub fn convert_routes(routes: Vec<Route>, global: &Object) -> MyResult<Vec<TemplateToRender>> {
    let mut templates: Vec<TemplateToRender> = Vec::new();

    // For each route
    for route in routes {
        // Start with one path
        let mut paths = vec![PathToRender::new()];

        // For each fragment in route path
        for fragment in &route.path.0 {
            match fragment {
                // Literal
                Fragment::Literal(literal) => {
                    for PathToRender { filepath, .. } in &mut paths {
                        // Add literal fragment to filepath string
                        filepath.push('/');
                        filepath.push_str(literal);
                    }
                }

                // Dynamic value
                Fragment::Value(name) => {
                    // Get value as object
                    let Some(map) = global.get(name) else {
                        throw!("Value is not given with name '{}'", name);
                    };
                    let Some(map) = map.as_object() else {
                        throw!("Value given is not an object map, named '{}'", name);
                    };

                    // Move paths into temporary variable, reset paths
                    let old_paths = paths;
                    paths = Vec::new();

                    // For each old path
                    for old_path in old_paths {
                        // For each key/value in value map
                        for (key, value) in map {
                            // Clone path
                            let mut path = old_path.clone();

                            // Add key to filepath
                            path.filepath.push('/');
                            path.filepath.push_str(key);

                            // Add value to object
                            path.values.insert(name.clone(), value.clone());

                            // Add cloned path to new paths
                            paths.push(path);
                        }
                    }
                }
            }
        }

        // Add 'index.html' to end of every path
        for path in &mut paths {
            if !path.filepath.ends_with('/') {
                path.filepath.push('/');
            }
            path.filepath.push_str("index.html");
        }

        // Push template
        templates.push(TemplateToRender { route, paths });
    }

    Ok(templates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::RoutePath;
    use Fragment::*;

    #[test]
    fn convert_routes_works() {
        let values = object! {
            item: object!{
                bar: "Value bar",
                foo: "Value foo",
            },
            other: object!{
                foo2: "Value foo 2",
                bar2: "Value bar 2",
            }
        };

        let original = vec![
            Route {
                path: RoutePath(vec![Literal("".to_string())]),
                template: "INDEX".to_string(),
            },
            Route {
                path: RoutePath(vec![Literal("404".to_string())]),
                template: "NOT-FOUND".to_string(),
            },
            Route {
                path: RoutePath(vec![
                    Literal("values".to_string()),
                    Value("item".to_string()),
                ]),
                template: "VALUES".to_string(),
            },
            Route {
                path: RoutePath(vec![
                    Value("item".to_string()),
                    Value("other".to_string()),
                    Literal("last".to_string()),
                ]),
                template: "NESTED-VALUES".to_string(),
            },
        ];

        let templates = convert_routes(original, &values).unwrap();

        let expected = vec![
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![Literal("".to_string())]),
                    template: "INDEX".to_string(),
                },
                paths: vec![PathToRender {
                    filepath: "/index.html".to_string(),
                    values: object! {},
                }],
            },
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![Literal("404".to_string())]),
                    template: "NOT-FOUND".to_string(),
                },
                paths: vec![PathToRender {
                    filepath: "/404/index.html".to_string(),
                    values: object! {},
                }],
            },
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![
                        Literal("values".to_string()),
                        Value("item".to_string()),
                    ]),
                    template: "VALUES".to_string(),
                },
                //* must be correct order (alphabetical??)
                paths: vec![
                    PathToRender {
                        filepath: "/values/bar/index.html".to_string(),
                        values: object! {item:"Value bar"},
                    },
                    PathToRender {
                        filepath: "/values/foo/index.html".to_string(),
                        values: object! {item:"Value foo"},
                    },
                ],
            },
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![
                        Value("item".to_string()),
                        Value("other".to_string()),
                        Literal("last".to_string()),
                    ]),
                    template: "NESTED-VALUES".to_string(),
                },
                //* must be correct order (alphabetical??)
                paths: vec![
                    PathToRender {
                        filepath: "/bar/bar2/last/index.html".to_string(),
                        values: object! { item:"Value bar", other: "Value bar 2" },
                    },
                    PathToRender {
                        filepath: "/bar/foo2/last/index.html".to_string(),
                        values: object! { item:"Value bar", other: "Value foo 2" },
                    },
                    PathToRender {
                        filepath: "/foo/bar2/last/index.html".to_string(),
                        values: object! { item:"Value foo", other: "Value bar 2" },
                    },
                    PathToRender {
                        filepath: "/foo/foo2/last/index.html".to_string(),
                        values: object! { item:"Value foo", other: "Value foo 2" },
                    },
                ],
            },
        ];

        // backtrace
        for (i, template) in templates.iter().enumerate() {
            let expected = expected.get(i).expect("More templates that expected");

            println!("\n===========================");
            println!("\nLEFT (template)\n{:#?}", template);
            println!("\nRIGHT (expected)\n{:#?}", expected);

            assert_eq!(
                template.route, expected.route,
                "Template ROUTE (index {}) did not match",
                i
            );

            for (j, template_path) in template.paths.iter().enumerate() {
                let expected_path = expected
                    .paths
                    .get(j)
                    .expect("More template paths that expected");

                println!("\n===========================");
                println!("\nLEFT (template PATH)\n{:#?}", template_path);
                println!("\nRIGHT (expected PATH)\n{:#?}", expected_path);

                assert_eq!(
                    template_path, expected_path,
                    "Template (index {}) PATH (index {}) did not match",
                    i, j
                );
            }

            assert_eq!(template, expected, "Template (index {}) did not match", i);
        }

        assert_eq!(templates, expected, "(Final check)");
    }
}
