use std::collections::HashMap;

use crate::package_name::PackageName;

use super::{
    package_container::PackageContainer,
    structs::{Build, BuildPackage},
};

fn path_to_string<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
    path.as_ref()
        .file_name()
        .and_then(|file_name| file_name.to_str().map(std::string::ToString::to_string))
}

impl PackageContainer for Build {
    type PackageType = BuildPackage;

    fn get_all_package_paths(&self) -> HashMap<PackageName, Self::PackageType> {
        log::info!("Getting all built packages in the build folder");
        let Self(build_path) = self;

        log::debug!("Got build path: {}", build_path.display());

        std::fs::read_dir(build_path).map_or_else(
            |_| HashMap::new(),
            |paths| {
                paths
                    .filter_map(std::result::Result::ok)
                    .map(|e| e.path())
                    .filter(|p| p.exists() && p.is_dir())
                    .filter_map(path_to_string)
                    .filter(|s| !crate::config::BUILD_IGNORE_DIRS.contains(&s.as_str()))
                    .map(|s| (PackageName(s.clone()), BuildPackage(build_path.join(s))))
                    .collect()
            },
        )
    }
}
