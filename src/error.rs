use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Fail: {0}")]
    IoFail(IoError),

    #[error("Source directory does not exist '{0}'")]
    SourceDirectoryNotExist(String),

    #[error("Failed to convert SCSS file '{0}' to CSS `{1:?}`")]
    ScssConvert(String, grass::Error),

    #[error("Failed to minify CSS file '{0}' `{1:?}`")]
    CssMinify(String, String),

    #[error("Failed to render handlebars template '{0}' `{1:?}`")]
    RenderTemplate(String, handlebars::RenderError),

    #[error("Failed to register handlebars template '{0}' `{1:?}`")]
    RegisterTemplate(String, handlebars::TemplateError),

    #[error("Failed to register *inbuilt* handlebars template '{0}' `{1:?}`")]
    RegisterInbuiltTemplate(String, handlebars::TemplateError),

    #[error("Generic error! `{0}`")]
    Generic(String),
}

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Reading file '{0}' `{1}`")]
    ReadFile(String, io::Error),

    #[error("Reading directory '{0}' `{1}`")]
    ReadDir(String, io::Error),

    #[error("Removing directory '{0}' `{1}`")]
    RemoveDir(String, io::Error),

    #[error("Creating directory '{0}' `{1}`")]
    CreateDir(String, io::Error),

    #[error("Copying directory '{0}' `{1}`")]
    CopyDir(String, io::Error),

    #[error("Writing file '{0}' `{1}`")]
    WriteFile(String, io::Error),
}
