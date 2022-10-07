use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize}; 

use crate::catkin; 

pub const COMPILE_COMMANDS_FILENAME: &str = "compile_commands.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageLink {
    pub package: String,
    build_path: std::path::PathBuf, 
    source_path: std::path::PathBuf, 
    pub build_cc_path: std::path::PathBuf, 
    pub source_cc_path: std::path::PathBuf,
}

impl PackageLink {
    pub fn new(package: &str, 
               build_path: std::path::PathBuf, 
               source_path: std::path::PathBuf) 
        -> PackageLink {
        let package = package.to_string();
        let build_cc_path = build_path.join(COMPILE_COMMANDS_FILENAME); 
        let source_cc_path = source_path.join(COMPILE_COMMANDS_FILENAME); 

        PackageLink {
            package,
            build_path,
            source_path,
            build_cc_path,
            source_cc_path,
        }
    }

    pub fn update(&mut self, link: &Self) {
        self.package = link.package; 
        self.build_path = link.build_path; 
        self.source_path = link.source_path; 
        self.build_cc_path = link.build_cc_path; 
        self.source_cc_path = link.source_cc_path; 
    }

    pub fn build_path_is_valid(&self) -> bool {
        self.build_cc_path.is_file()
    }

    pub fn source_path_is_valid(&self) -> bool {
        catkin::is_package(&self.source_path)
    }

    pub fn is_linked(&self) -> bool {
        let source_path = std::fs::canonicalize(&self.source_cc_path); 
        let build_path = std::fs::canonicalize(&self.build_cc_path); 

        match (source_path, build_path) {
            (Ok(source_path), Ok(build_path)) => source_path == build_path,
            (_, _) => false,
        }
    }

    pub fn remove_link(&self) -> Result<(), std::io::Error> {
        std::fs::remove_file(&self.source_cc_path)
    }

    pub fn create_link(&self) -> std::result::Result<(), std::io::Error> {
        std::os::unix::fs::symlink(
            &self.build_cc_path, &self.source_cc_path
        )
    }
}

fn create_package_link(package: &str, build_path: &PathBuf, p2sp: &HashMap<String, PathBuf>) -> Option<PackageLink> {
    match p2sp.get(package) {
        Some(source_path) => Some(
            PackageLink::new(package, build_path.clone(), source_path.clone())
        ), 
        None => None
    }
}

pub fn get_package_links(base_path: &std::path::PathBuf) -> Option<std::vec::Vec<PackageLink>> {
    let source_dir = catkin::get_src_dir(base_path); 
    let build_dir = catkin::get_build_dir(base_path);

    let packages = catkin::get_packages(base_path);
    let package_source_paths = catkin::get_all_package_paths(base_path); 
    
    match (source_dir, build_dir, packages, package_source_paths) {
        (Ok(_source_dir), Ok(build_dir), Ok(packages), Ok(package_source_paths)) => {
            let build_dir = std::path::Path::new(&build_dir); 
            let build_dir = build_dir.to_path_buf(); 

            Some(
                packages.iter().filter_map(
                    |package| create_package_link(
                        package, &build_dir, &package_source_paths
                    )
                ).collect()
            )
        }, 
        (_, _, _, _) => None,
    }
}

pub fn symlink_package_links(links: &std::vec::Vec<PackageLink>) {
    for link in links {
        link.create_link();
    }
}
