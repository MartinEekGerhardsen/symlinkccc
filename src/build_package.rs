use std::collections::HashMap;

use walkdir::WalkDir;

use crate::{
    build::get_build_path,
    package::Package,
    ros_paths::{Build, BuildPackage, Workspace},
};

pub fn get_all_build_package_paths(workspace: &Workspace) -> HashMap<Package, BuildPackage> {
    log::info!("Getting all built packages in the build folder");
    let Build(build_path) = get_build_path(workspace);

    log::debug!("Got build path: {}", build_path.display());
    // TODO: ok what the hell

    WalkDir::new(build_path.clone())
        .max_depth(1)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.path().exists() && e.path().is_dir())
        .filter_map(|e| e.file_name().to_str().map(std::string::ToString::to_string))
        .filter(|s| !crate::config::BUILD_IGNORE_DIRS.contains(&s.as_str()))
        .map(|s| (Package(s.clone()), BuildPackage(build_path.join(s))))
        .collect()
}
