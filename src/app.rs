use std::fs;

use handlebars::Handlebars;

use crate::{
    convert::{register_inbuilt_templates, register_templates, render_page, scss_to_css},
    files::{check_src_folders, clean_build_dir, load_folder_recurse},
    object,
    server::{self, watch},
    throw, Config, Object, Page, Pages, Result, Unreact, DEV_BUILD_DIR,
};

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
            url,
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

    pub fn compile(&self) -> Result {
        clean_build_dir(&self.config)?;

        let mut registry = Handlebars::new();

        if self.config.strict {
            registry.set_strict_mode(true);
        }

        register_inbuilt_templates(&mut registry, &self.url)?;

        let templates = load_folder_recurse(&self.config.templates)?;
        register_templates(&mut registry, templates)?;

        for (name, page) in &self.pages {
            let path = format!("./{}/{}", self.config.build, name);
            if let Err(err) = fs::create_dir_all(&path) {
                throw!(
                    "IO Error! Could not create a folder in the build directory '{}', `{:?}`",
                    path,
                    err
                );
            }

            let content =
                render_page(&mut registry, name, page, self.globals.clone(), self.is_dev)?;

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
        for (name, scss) in styles {
            let parent = format!("{}/{}/{}", self.config.build, self.config.styles, name);
            if let Err(err) = fs::create_dir_all(&parent) {
                throw!(
                    "IO Error! Could not create folder in styles build directory '{}' `{:?}`",
                    parent,
                    err
                );
            }

            let css = scss_to_css(&name, &scss)?;

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
