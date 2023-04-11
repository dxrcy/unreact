use crate::{
    error::MyResult,
    routes::{Fragment, Route},
    Object,
};

#[derive(Debug, PartialEq)]
pub struct TemplateToRender {
    route: Route,
    paths: Vec<PathToRender>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathToRender {
    filepath: String,
    values: Object,
}

impl PathToRender {
    pub fn new() -> Self {
        Self {
            filepath: String::new(),
            values: Object::new(),
        }
    }
}

pub fn convert_routes(routes: Vec<Route>, values: Object) -> MyResult<Vec<TemplateToRender>> {
    let mut templates: Vec<TemplateToRender> = Vec::new();

    for route in routes {
        let mut paths = vec![PathToRender::new()];

        for fragment in route.path.clone() {
            match fragment {
                Fragment::Literal(literal) => {
                    for PathToRender { filepath, .. } in &mut paths {
                        filepath.push('/');
                        filepath.push_str(&literal);
                    }
                }

                //TODO optimize
                Fragment::Value(name) => {
                    // println!("    /{} (Value)", name);

                    let Some(map) = values.get(&name) else {
                        throw!("Value is not given with name '{}'", name);
                    };
                    let Some(map) = map.as_object() else {
                        throw!("Value given is not an object map, named '{}'", name);
                    };

                    let mut new_paths = Vec::new();

                    for path in paths {
                        for (key, value) in map {
                            let mut cloned = path.clone();

                            cloned.filepath.push('/');
                            cloned.filepath.push_str(key);

                            cloned.values.insert(name.clone(), value.to_owned());

                            new_paths.push(cloned);
                        }
                    }

                    paths = new_paths;
                }
            }
        }

        for path in &mut paths {
            if !path.filepath.ends_with('/') {
                path.filepath.push('/');
            }
            path.filepath.push_str("index.html");
        }

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
    fn it_works() {
        let values = object! {
            item: object!{
                bar: "Value bar",
                foo: "Value foo",
            },
            other: object!{
                foo2: "Value foo 2",
                bar2: "Value bar 2",
            },
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

        let templates = convert_routes(original, values).unwrap();

        let expected = vec![
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![Literal("".to_string())]),
                    template: "INDEX".to_string(),
                },
                paths: vec![PathToRender {
                    filepath: "/index.html".to_string(),
                    values: Object::new(),
                }],
            },
            TemplateToRender {
                route: Route {
                    path: RoutePath(vec![Literal("404".to_string())]),
                    template: "NOT-FOUND".to_string(),
                },
                paths: vec![PathToRender {
                    filepath: "/404/index.html".to_string(),
                    values: Object::new(),
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
                        values: object! { item:"Value bar" },
                    },
                    PathToRender {
                        filepath: "/values/foo/index.html".to_string(),
                        values: object! { item:"Value foo" },
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
