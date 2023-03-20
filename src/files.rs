use std::{fs, path::Path};

use crate::{Config, FileMap, Result};

pub fn get_filename(full_name: &str) -> Option<&str> {
    full_name.split('.').next()
}

pub fn check_src_folders(config: &Config) -> Result {
    let src_folders = [&config.templates, &config.public, &config.styles];
    for folder in src_folders {
        if !Path::new(&folder).is_dir() {
            throw!("Directory not exist! '{}'", folder);
        }
    }

    Ok(())
}

pub fn load_folder_recurse(folder: &str) -> Result<FileMap> {
    let mut filemap = FileMap::new();
    load_filemap(&mut filemap, folder, "")?;
    Ok(filemap)
}

fn load_filemap(map: &mut FileMap, root: &str, parent: &str) -> Result {
    let full_path = format!("./{root}/{parent}/");

    let children = try_unwrap!(
        fs::read_dir(&full_path),

        else Err(err) => throw!(
            "IO Error! Could not read director '{}' `{:?}`",
            full_path,
            err
        ),
    );

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

        let content = try_unwrap!(
            fs::read_to_string(&path),
            else Err(err) => throw!("IO Error! Could not read file '{}' `{:?}`", path, err),
        );

        map.insert(format!("{parent}{name}"), content);
    }

    Ok(())
}

pub fn clean_build_dir(config: &Config) -> Result {
    let build_folder = format!("./{}", config.build);
    if Path::new(&build_folder).exists() {
        try_unwrap!(
            fs::remove_dir_all(&build_folder),

            else Err(err) => throw!(
                "IO Error! Could not remove build directory '{}' `{:?}`",
                build_folder,
                err
            )
        );
    }

    let out_folders = ["", "styles", "public"];
    for folder in out_folders {
        let path = format!("./{}/{}", config.build, folder);

        try_unwrap!(
            fs::create_dir_all(&path),

            else Err(err) => throw!(
                "IO Error! Could not create directory in build folder '{}' `{:?}`",
                path,
                err
            )
        );
    }

    let path = format!("./{}", config.public);
    try_unwrap!(
        dircpy::copy_dir(&path, format!("./{}/public", config.build)),

        else Err(err) => throw!(
            "IO Error! Could not copy public directory '{}' `{:?}`",
            path,
            err
        )
    );

    Ok(())
}