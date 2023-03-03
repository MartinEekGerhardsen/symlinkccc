use std::collections::HashMap;

use walkdir::{DirEntry, WalkDir};

use crate::{
    package::Package,
    paths::{
        origin_file::OriginFile,
        structs::{Source, SourcePackage, SourcePackageCMakeLists, SourcePackageXML, Workspace},
    },
};

fn extract_package_name(document: &str, pattern: &regex::Regex) -> Option<Package> {
    log::debug!("Extracting package name from document: \n{document}");
    let package_name_captures = pattern.captures(document)?;

    log::debug!("Captures from document: {package_name_captures:?}");

    let package_name_match = package_name_captures.get(1)?;

    let package_name = Package(package_name_match.as_str().to_string());

    Some(package_name)
}

fn get_file_package_name(path: &std::path::PathBuf, pattern: &regex::Regex) -> Option<Package> {
    std::fs::read(path).map_or_else(
        |_| {
            log::warn!("Couldn't read document path, ignoring this path");
            None
        },
        |data| {
            log::debug!("Successfully read {}", path.display());
            std::str::from_utf8(&data).map_or_else(
                |_| {
                    log::warn!("Couldn't convert from utf8 to string, ignoring this path");
                    None
                },
                |doc| {
                    log::debug!("Successfully converted data from utf8 to str");
                    extract_package_name(doc, pattern)
                },
            )
        },
    )
}

fn get_package_name(source: &SourcePackage) -> Option<Package> {
    log::info!("Getting package name for potential package: {source:?}");

    let xml = SourcePackageXML::from(source);
    let xml = xml.canonicalize().ok()?;

    let cmakelists = SourcePackageCMakeLists::from(source);
    let cmakelists = cmakelists.canonicalize().ok()?;

    if !xml.exists() {
        log::info!("Assuming {source} is not a package as it has no package.xml file");
        log::debug!("Can't find package.xml file: {xml}");

        return None;
    }

    if !cmakelists.exists() {
        log::info!("Assuming {source} is not a package as it has no CMakeLists.txt file");
        log::debug!("Can't find CMakeLists.txt file: {cmakelists}");
        return None;
    }

    lazy_static::lazy_static! {
        static ref XML_REGEX: regex::Regex = regex::Regex::new(r"\s*<\s*name\s*>\s*(\w*)\s*<\s*/\s*name\s*>\s*").unwrap();
    }
    let SourcePackageXML(xml_path) = xml;
    let xml = get_file_package_name(&xml_path, &XML_REGEX)?;
    log::debug!("Package name from package.xml: {xml}");

    lazy_static::lazy_static! {
        static ref CMAKELISTS_REGEX: regex::Regex = regex::Regex::new(r"\s*project\s*\(\s*(\w*)\s*\)\s*").unwrap();
    }
    let SourcePackageCMakeLists(cmakelists_path) = cmakelists;
    let cmakelists = get_file_package_name(&cmakelists_path, &CMAKELISTS_REGEX)?;
    log::debug!("Package name from CMakeLists.txt: {cmakelists}");

    if xml == cmakelists {
        log::info!("Found package name: {xml}");
        Some(xml)
    } else {
        log::warn!("Package name from package.xml and CMakeLists.txt are different!");
        None
    }
}

fn get_package_name_from_entry(entry: &walkdir::DirEntry) -> Option<(Package, SourcePackage)> {
    let path = entry.path();
    if !path.exists() || !path.is_dir() {
        log::info!("Path is not a valid package {}", path.display());
        return None;
    }

    let package = SourcePackage(path.to_path_buf());
    let package_name = get_package_name(&package)?;

    Some((package_name, package))
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

pub fn get_all_source_package_paths(workspace: &Workspace) -> HashMap<Package, SourcePackage> {
    log::info!("Getting all source packages in the source folder");
    let Source(source_path) = Source::from(workspace);

    log::debug!("Got source path: {}", source_path.display());

    let walker = WalkDir::new(source_path).follow_links(true);
    walker
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(std::result::Result::ok)
        .filter_map(|e| get_package_name_from_entry(&e))
        .collect()
}
