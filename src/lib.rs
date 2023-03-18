mod macros;
mod server;

use std::{collections::HashMap, fs, path::Path};

use handlebars::Handlebars;

pub use serde_json::Value;

pub type FileMap = HashMap<String, String>;

pub type Object = serde_json::Map<String, Value>;

pub use server::watch;

macro_rules! throw {
    ( $lit: literal $(, $arg: expr )* ) => {
        return Err(format!($lit, $( $arg ),*))
    };
}

#[derive(Debug)]
pub struct Config {
    pub build: String,
    pub templates: String,
    pub styles: String,
    pub public: String,

    pub strict: bool,
    pub minify: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            build: "build".to_string(),
            templates: "templates".to_string(),
            public: "public".to_string(),
            styles: "styles".to_string(),
            strict: false,
            minify: true,
        }
    }
}

type Pages = HashMap<String, Page>;

#[derive(Debug)]
enum Page {
    Raw(String),
    Template { template: String, data: Object },
}

#[derive(Debug)]
pub struct Unreact {
    config: Config,
    pages: Pages,
    globals: Object,
    url: String,
    is_dev: bool,
}

type Result<T = ()> = std::result::Result<T, String>;

pub const DEV_BUILD_DIR: &str = ".devbuild";

impl Unreact {
    pub fn new(mut config: Config, is_dev: bool, url: &str) -> Result<Self> {
        if is_dev {
            config.build = DEV_BUILD_DIR.to_string();
        }

        let url = if is_dev {
            format!("http://{}", server::SERVER_ADDRESS)
        } else {
            url.to_string()
        };

        check_src_folders(&config)?;

        Ok(Self {
            config,
            pages: Pages::new(),
            globals: Object::new(),
            url: url.to_string(),
            is_dev,
        })
    }

    pub fn globalize(&mut self, data: Object) -> &mut Self {
        self.globals = data;
        self
    }

    pub fn route(&mut self, path: &str, template: &str, data: Object) -> &mut Self {
        self.pages.insert(
            path.to_string(),
            Page::Template {
                template: template.to_string(),
                data,
            },
        );
        self
    }

    pub fn route_exact(&mut self, path: &str, content: String) -> &mut Self {
        self.pages.insert(path.to_string(), Page::Raw(content));
        self
    }

    pub fn route_bare(&mut self, path: &str, template: &str) -> &mut Self {
        self.route(path, template, object! {})
    }

    pub fn index(&mut self, template: &str, data: Object) -> &mut Self {
        self.route("", template, data)
    }

    pub fn not_found(&mut self, template: &str, data: Object) -> &mut Self {
        self.route("404", template, data)
    }

    fn clean_build_dir(&self) -> Result {
        let build_folder = format!("./{}", self.config.build);
        if Path::new(&build_folder).exists() {
            if let Err(err) = fs::remove_dir_all(&build_folder) {
                throw!(
                    "IO Error! Could not remove build directory '{}' `{:?}`",
                    build_folder,
                    err
                );
            }
        }

        let out_folders = ["", "styles", "public"];
        for folder in out_folders {
            let path = format!("./{}/{}", self.config.build, folder);
            if let Err(err) = fs::create_dir_all(&path) {
                throw!(
                    "IO Error! Could not create directory in build folder '{}' `{:?}`",
                    path,
                    err
                );
            }
        }

        let path = format!("./{}", self.config.public);
        if let Err(err) = dircpy::copy_dir(&path, format!("./{}/public", self.config.build)) {
            throw!(
                "IO Error! Could not copy public directory '{}' `{:?}`",
                path,
                err
            );
        };

        Ok(())
    }

    pub fn compile(&self) -> Result {
        self.clean_build_dir()?;

        let mut registry = Handlebars::new();

        if self.config.strict {
            registry.set_strict_mode(true);
        }

        let inbuilt_templates: &[(&str, &str)] = &[
            // Base url for site
            ("URL", &self.url),
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
                    "Handlebars error! Registering inbuilt partial '{}', `{:?}`",
                    name,
                    err
                );
            }
        }

        let templates = load_folder_recurse(&self.config.templates)?;
        for (name, template) in templates {
            if let Err(err) = registry.register_partial(&name, template) {
                throw!(
                    "Handlebars error! Registering partial '{}', `{:?}`",
                    name,
                    err
                );
            }
        }

        for (name, page) in &self.pages {
            let path = format!("./{}/{}", self.config.build, name);
            if let Err(err) = fs::create_dir_all(&path) {
                throw!(
                    "IO Error! Could not create a folder in the build directory '{}', `{:?}`",
                    path,
                    err
                );
            }

            let mut content = match page {
                Page::Raw(page) => page.to_string(),
                Page::Template { template, data } => {
                    // Add global variables
                    let mut data = data.clone();
                    data.insert("GLOBAL".to_string(), Value::Object(self.globals.clone()));

                    // Render template
                    match registry.render(template, &data) {
                        Ok(rendered) => rendered,
                        Err(err) => throw!("Handlebars failed! Rendering '{}' `{:?}`", name, err),
                    }
                }
            };

            // Add dev script to file
            if self.is_dev {
                content += "\n\n";
                content += server::DEV_SCRIPT;
            }

            let path = format!("{path}/index.html");
            if let Err(err) = fs::write(&path, content) {
                throw!(
                    "IO Error! Could not write file in build directory '{}' `{:?}`",
                    path,
                    err
                );
            }
        }

        let styles = load_folder_recurse(&self.config.styles)?;
        for (page, scss) in styles {
            let parent = format!("{}/{}/{}", self.config.build, self.config.styles, page);
            if let Err(err) = fs::create_dir_all(&parent) {
                throw!(
                    "IO Error! Could not create folder in styles build directory '{}' `{:?}`",
                    parent,
                    err
                );
            }

            let css = match grass::from_string(scss, &Default::default()) {
                Ok(css) => css,
                Err(err) => throw!(
                    "SCSS to CSS Error! Problem with scss file '{}' `{:?}`",
                    page,
                    err
                ),
            };

            let path = format!("{}/style.css", parent);
            if let Err(err) = fs::write(&path, css) {
                throw!("IO Error! Could not write css file '{}' `{:?}`", path, err);
            }
        }

        Ok(())
    }

    pub fn run(&self) -> Result {
        if !self.is_dev {
            return self.compile();
        }

        let compile = || {
            if let Err(err) = self.compile() {
                eprintln!("Failed to build in dev mode!\n{:?}", err);
            }
        };

        //TODO Make ~*pretty*~
        println!(
            "Listening on http://localhost:{}\nWatching file changes",
            server::SERVER_PORT
        );

        compile();
        watch(compile);

        Ok(())
    }
}

fn check_src_folders(config: &Config) -> Result {
    let src_folders = [&config.templates, &config.public, &config.styles];
    for folder in src_folders {
        if !Path::new(&folder).is_dir() {
            throw!("Directory not exist! '{}'", folder);
        }
    }

    Ok(())
}

fn load_folder_recurse(folder: &str) -> Result<FileMap> {
    let mut filemap = FileMap::new();
    load_filemap(&mut filemap, folder, "")?;
    Ok(filemap)
}

fn load_filemap(map: &mut FileMap, root: &str, parent: &str) -> Result {
    let full_path = format!("./{root}/{parent}/");

    let children = match fs::read_dir(&full_path) {
        Ok(children) => children,
        Err(err) => throw!(
            "IO Error! Could not read director '{}' `{:?}`",
            full_path,
            err
        ),
    };

    for file in children.flatten() {
        let (path, full_name) = (file.path(), file.file_name());
        let Some((path, name)) = path.to_str().zip(full_name.to_str()) else {
            continue;
        };

        let path = path.replace('\\', "/");

        if Path::new(&path).is_dir() {
            load_filemap(map, root, &format!("{parent}{name}/"))?;
            continue;
        }

        let Some(name) = get_filename(name) else {
            continue;
        };

        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => throw!("IO Error! Could not read file '{}' `{:?}`", path, err),
        };

        map.insert(format!("{parent}{name}"), content);
    }

    Ok(())
}

fn get_filename(full_name: &str) -> Option<&str> {
    full_name.split('.').next()
}
