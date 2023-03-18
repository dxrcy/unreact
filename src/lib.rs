mod macros;
mod server;

use std::{collections::HashMap, fs, path::Path};

use handlebars::Handlebars;

pub use serde_json::Value;

pub type FileMap = HashMap<String, String>;

pub type Object = serde_json::Map<String, Value>;

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
    pub dev_warning: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            build: "build".to_string(),
            templates: "templates".to_string(),
            public: "public".to_string(),
            styles: "styles".to_string(),
            strict: false,
            dev_warning: true,
            minify: true,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct App<'a> {
    config: Config,

    pages: FileMap,

    registry: Handlebars<'a>,

    templates: FileMap,
    styles: FileMap,

    globals: Object,
    url: String,
    is_dev: bool,
}

type Result<T = ()> = std::result::Result<T, String>;

pub const DEV_BUILD_DIR: &str = ".devbuild";

impl<'a> App<'a> {
    pub fn new(mut config: Config, is_dev: bool, mut url: &str) -> Result<Self> {
        if is_dev {
            config.build = DEV_BUILD_DIR.to_string();
        }

        if is_dev {
            url = const_str::concat!("http://", server::ADDRESS);
        }

        Self::check_dirs(&config)?;

        let templates = Self::load_whole_folder(&config.templates)?;
        let styles = Self::load_whole_folder(&config.styles)?;

        let mut registry = Handlebars::new();

        if config.strict {
            registry.set_strict_mode(true);
        }

        for (name, template) in &templates {
            if let Err(err) = registry.register_partial(name, template) {
                throw!(
                    "Handlebars error! Registering partial '{}', `{:?}`",
                    name,
                    err
                );
            }
        }

        let inbuilt_templates: &[(&str, &str)] = &[
            // Base url for site
            ("URL", url),
            // Script for development
            // Is not registered if `dev_warning` in config is false
            (
                "DEV_SCRIPT",
                if is_dev && config.dev_warning {
                    server::DEV_SCRIPT
                } else {
                    ""
                },
            ),
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

        Ok(Self {
            config,

            pages: FileMap::new(),

            registry,

            templates,
            styles,

            globals: Object::new(),

            url: url.to_string(),
            is_dev,
        })
    }

    fn load_whole_folder(folder: &str) -> Result<FileMap> {
        let mut filemap = FileMap::new();
        load_filemap(&mut filemap, folder, "")?;
        Ok(filemap)
    }

    // ? pub ?
    fn check_dirs(config: &Config) -> Result {
        let src_folders = [&config.templates, &config.public, &config.styles];
        for folder in src_folders {
            if !Path::new(&folder).is_dir() {
                throw!("Directory not exist! '{}'", folder);
            }
        }

        Ok(())
    }

    pub fn set_globals(&mut self, data: Object) -> &mut Self {
        self.globals = data;
        self
    }

    //TODO Rename
    pub fn render_empty(&self, name: &str) -> Result<String> {
        self.render(name, object! {})
    }

    pub fn render(&self, name: &str, mut data: Object) -> Result<String> {
        // Add global variables
        // let mut data = data.clone();
        data.insert("GLOBAL".to_string(), Value::Object(self.globals.clone()));

        // Render template
        match self.registry.render(name, &data) {
            Ok(rendered) => Ok(rendered),

            Err(err) => throw!("Handlebars failed! Rendering '{}' `{:?}`", name, err),
        }
    }

    //TODO Rename
    pub fn page_plain(&mut self, path: &str, content: String) -> &mut Self {
        self.pages.insert(path.to_string(), content);
        self
    }

    pub fn page(&mut self, path: &str, template: &str, data: Object) -> Result<&mut Self> {
        self.page_plain(path, self.render(template, data)?);
        Ok(self)
    }

    pub fn index(&mut self, template: &str, data: Object) -> Result<&mut Self> {
        self.page_plain("", self.render(template, data)?);
        Ok(self)
    }

    //TODO
    // pub fn page_empty()

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

    pub fn finish(self) -> Result {
        self.clean_build_dir()?;

        for (page, content) in self.pages {
            let path = format!("./{}/{}", self.config.build, page);
            if let Err(err) = fs::create_dir_all(&path) {
                throw!(
                    "IO Error! Could not create a folder in the build directory '{}', `{:?}`",
                    path,
                    err
                );
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

        for (page, scss) in self.styles {
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

        if self.is_dev {
            server::listen();
        }

        Ok(())
    }
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
