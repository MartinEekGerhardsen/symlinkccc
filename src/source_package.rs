use std::{collections::HashMap, path::Path, process::Command};

use crate::{
    package::Package,
    ros_paths::{SourcePackage, Workspace},
};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    FromUtf8(std::string::FromUtf8Error),
    Which(which::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<which::Error> for Error {
    fn from(value: which::Error) -> Self {
        Self::Which(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn split_package_to_path(package_to_path: &str) -> Option<(Package, SourcePackage)> {
    if package_to_path.trim().is_empty() {
        None
    } else {
        let package_and_path: Vec<&str> = package_to_path.splitn(2, ' ').collect();

        match (package_and_path.first(), package_and_path.get(1)) {
            (Some(package), Some(path)) => Some((
                Package(package.trim().to_string()),
                SourcePackage(Path::new(path.trim()).to_path_buf()),
            )),
            (_, _) => None,
        }
    }
}

pub fn get_all_source_package_paths(
    Workspace(workspace_path): &Workspace,
) -> Result<HashMap<Package, SourcePackage>> {
    log::info!("Getting all packages and related source paths");

    let rospack = which::which(crate::config::ROSPACK_COMMAND)?;
    log::debug!("Found rospack command: {}", rospack.display());

    let rospack_list = Command::new(rospack)
        .args(["list"])
        .current_dir(workspace_path)
        .output()?;
    log::debug!("Result from `rospack list` command:\n{rospack_list:?}");

    let rospack_list = String::from_utf8(rospack_list.stdout)?;
    log::debug!("Standard output:\n{rospack_list}");

    let name_to_package = rospack_list
        .split('\n')
        .filter_map(split_package_to_path)
        .collect();
    log::debug!("`rospack list` result split:\n{name_to_package:?}");

    Ok(name_to_package)
}
