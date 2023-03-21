use std::fs;

use handlebars::Handlebars;

use crate::{
    convert::{register_inbuilt_templates, register_templates, render_page, scss_to_css},
    files::{check_source_folders, clean_build_dir, read_folder_recurse},
    object, Config, Object, Page, Pages, Result, Unreact, DEV_BUILD_DIR,
};

#[cfg(feature = "dev")]
use crate::server;

impl Unreact {
    pub fn new(mut config: Config, is_dev: bool, url: &str) -> Result<Self> {
        if is_dev {
            config.build = DEV_BUILD_DIR.to_string();
        }

        check_source_folders(&config)?;

        Ok(Self {
            config,
            pages: Pages::new(),
            globals: Object::new(),
            url: get_url(url, is_dev),
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

        let templates = read_folder_recurse(&self.config.templates)?;
        register_templates(&mut registry, templates)?;

        for (name, page) in &self.pages {
            let path = format!("./{}/{}", self.config.build, name);

            try_unwrap!(
                fs::create_dir_all(&path),
                else Err(err) => throw!(
                    "IO Error! Could not create a folder in the build directory '{}', `{:?}`",
                    path,
                    err
                )
            );

            let content = render_page(
                &mut registry,
                name,
                page,
                self.globals.clone(),
                self.is_dev,
                self.config.minify,
            )?;

            let path = format!("{path}/index.html");
            try_unwrap!(
                fs::write(&path, content),
                else Err(err) => throw!(
                    "IO Error! Could not write file in build directory '{}' `{:?}`",
                    path,
                    err
                )
            );
        }

        let styles = read_folder_recurse(&self.config.styles)?;
        for (name, scss) in styles {
            let parent = format!("{}/{}/{}", self.config.build, self.config.styles, name);
            try_unwrap!(
                fs::create_dir_all(&parent),
                else Err(err) => throw!(
                    "IO Error! Could not create folder in styles build directory '{}' `{:?}`",
                    parent,
                    err
                )
            );

            let css = scss_to_css(&name, &scss, self.config.minify)?;

            let path = format!("{}/style.css", parent);
            try_unwrap!(
                fs::write(&path, css),
                else Err(err) => throw!("IO Error! Could not write css file '{}' `{:?}`", path, err)
            );
        }

        Ok(())
    }

    #[cfg(not(feature = "dev"))]
    pub fn run(&self) -> Result {
        self.compile()
    }

    //TODO Rename
    #[cfg(feature = "dev")]
    pub fn run(&self) -> Result {
        if !self.is_dev {
            return self.compile();
        }

        let compile = || {
            try_unwrap!(
                self.compile(),
                else Err(err) => eprintln!("Failed to build in dev mode!\n{:?}", err),
            );
        };

        //TODO Make ~*pretty*~
        println!("Listening on http://localhost:{}", server::SERVER_PORT);

        compile();

        cfg_if!( if #[cfg(feature = "watch")] {
            std::thread::spawn(server::listen);

            server::watch(compile);
        } else {
            server::listen();
        });

        Ok(())
    }
}

fn get_url(url: &str, #[allow(unused_variables)] is_dev: bool) -> String {
    // If `watch` feature is used, and `is_dev`
    cfg_if!( if #[cfg(feature = "dev")] {
        if is_dev {
            return format!("http://localhost:{}", server::SERVER_PORT);
        }
    });

    // Default
    url.to_string()
}
