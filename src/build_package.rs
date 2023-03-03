use std::collections::HashMap;

use crate::{
    package::Package,
    paths::structs::{Build, BuildPackage, Workspace},
};

fn valid_path_buf_to_str(path: std::path::PathBuf) -> Option<String> {
    if let Some(file_name) = path.file_name() {
        file_name.to_str().map(|file_name| file_name.to_string())
    } else {
        None
    }
}

pub fn get_all_build_package_paths(workspace: &Workspace) -> HashMap<Package, BuildPackage> {
    log::info!("Getting all built packages in the build folder");
    let Build(build_path) = Build::from(workspace);

    log::debug!("Got build path: {}", build_path.display());

    if let Ok(paths) = std::fs::read_dir(&build_path) {
        paths
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.exists() && p.is_dir())
            .filter_map(valid_path_buf_to_str)
            .filter(|s| !crate::config::BUILD_IGNORE_DIRS.contains(&s.as_str()))
            .map(|s| (Package(s.clone()), BuildPackage(build_path.join(s))))
            .collect()
    } else {
        HashMap::new()
    }
}
