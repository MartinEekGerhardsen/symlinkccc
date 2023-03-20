use std::collections::HashMap;

use crate::package_name::PackageName;

pub trait PackageContainer {
    type PackageType;

    fn get_all_package_paths(&self) -> HashMap<PackageName, Self::PackageType>;
}
