use std::string::FromUtf8Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap; 
use std::vec::Vec; 

#[derive(Debug)]
pub enum CommandAndUTF8Error {
    Command(std::io::Error), 
    UTF8(FromUtf8Error),
}

const CATKIN_PATH: &str = "/usr/bin/catkin";
const ROSPACK_PATH: &str = "/opt/ros/melodic/bin/rospack";

/* get_packages
 *
 * List all packages by name in path
 *
 */
pub fn get_packages<P: AsRef<Path>>(path: P) -> Result<Vec<String>, CommandAndUTF8Error> {
    let catkin_list_unformatted = Command::new(CATKIN_PATH)
        .args(["list", "--unformatted"])
        .current_dir(path)
        .output(); 
    
    match catkin_list_unformatted {
        Ok(catkin_list_unformatted) =>{ 
            match String::from_utf8(catkin_list_unformatted.stdout) {
                Ok(packages) => Ok(packages.split('\n')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()),
                Err(err) => Err(CommandAndUTF8Error::UTF8(err)),
            }
        }, 
        Err(err) => Err(CommandAndUTF8Error::Command(err)),
    }
}

/*
 * get_package_path
 *
 * Find path of specified package given 
 * a path. Assumes the package name is 
 * valid, as the returning String is not 
 * checked as a path. 
 */
pub fn get_package_path<P: AsRef<Path>>(path: P, package: &str) -> Result<String, CommandAndUTF8Error> {
    let rospack_find_package = Command::new(ROSPACK_PATH)
        .args(["find", package])
        .current_dir(path)
        .output();
    
    match rospack_find_package {
        Ok(rospack_find_package) => {
            let data = String::from_utf8(rospack_find_package.stdout); 
            match data {
                Ok(data) => Ok(data), 
                Err(err) => Err(CommandAndUTF8Error::UTF8(err)),
            }
        },
        Err(err) => Err(CommandAndUTF8Error::Command(err)),
    }
}

/*
 * split_package_to_path
 *
 * Helper-function to split a &str of the shape 
 * "package package_path" into their respective variables. 
 * Returns an option to simplify get_all_package_paths, 
 * as both invalid package names and invalid paths are ignored.
 */
fn split_package_to_path(package_to_path: &str) -> Option<(String, PathBuf)> {
    if package_to_path.trim().is_empty() {
        None
    } else {
        let package_and_path: Vec<&str> = package_to_path
            .splitn(2, " ")
            .collect(); 
        
        match (package_and_path.get(0), package_and_path.get(1)) {
            (Some(package), Some(path)) => {
                Some((
                    package.trim().to_string(), 
                    Path::new(path.trim()).to_path_buf(),
                ))
            }, 
            (_, _) => None,
        }
    }
}

/* 
 * get_all_package_paths
 *
 * Returns a HashMap of all available 
 * package names and their corresponding 
 * paths. This includes installed packages, 
 * not only those in the src directory of a
 * catkin 
 */
pub fn get_all_package_paths<P: AsRef<Path>>(path: P) -> Result<HashMap<String, PathBuf>, CommandAndUTF8Error> {
    let rospack_list = Command::new(ROSPACK_PATH)
        .args(["list"])
        .current_dir(path)
        .output();

    match rospack_list {
        Ok(rospack_list) => {
            let rospack_list = String::from_utf8(rospack_list.stdout); 

            match rospack_list {
                Ok(rospack_list) => {
                    Ok(rospack_list.split("\n").filter_map(
                        |pack_to_path| split_package_to_path(pack_to_path)
                    ).collect())
                }, 
                Err(err) => Err(CommandAndUTF8Error::UTF8(err)),
            }
        }, 
        Err(err) => Err(CommandAndUTF8Error::Command(err)),
    }
}

/*
 * get_dir
 *
 * Parses result of `catkin locate`. 
 * This gives information on the current 
 * directory based on which flag is given. 
 *
 * "-b" returns the build directory for the 
 * current workspace
 *
 * "-s" returns the source directory for the 
 * current workspace 
 */
fn get_dir<P: AsRef<Path>>(path: P, flag: &str) -> Result<String, CommandAndUTF8Error> {
    let catkin_locate_flag = Command::new(CATKIN_PATH)
        .args(["locate", flag])
        .current_dir(path)
        .output();

    match catkin_locate_flag {
        Ok(catkin_locate_flag) => {
            let catkin_locate_flag = String::from_utf8(catkin_locate_flag.stdout); 

            match catkin_locate_flag {
                Ok(catkin_locate_flag) => Ok(
                    catkin_locate_flag.chars().filter(|c| !c.is_whitespace()).collect()
                ),
                Err(err) => Err(CommandAndUTF8Error::UTF8(err)),
            }
        }, 
        Err(err) => Err(CommandAndUTF8Error::Command(err)),
    }
}

pub fn get_build_dir<P: AsRef<std::path::Path>>(path: P) -> Result<String, CommandAndUTF8Error> {
    get_dir(path, "-b")
}

pub fn get_src_dir<P: AsRef<std::path::Path>>(path: P) -> Result<String, CommandAndUTF8Error> {
    get_dir(path, "-s")
}

pub fn get_workspace_dir<P: AsRef<std::path::Path>>(path: P) -> Result<String, CommandAndUTF8Error> {
    get_dir(path, "")
}

pub fn is_package<S: AsRef<OsStr> + ?Sized>(path: &S) -> bool {
    let cmakelists = std::path::Path::new(&path).join("CMakeLists.txt"); 
    let package = std::path::Path::new(&path).join("package.xml"); 

    return cmakelists.is_file() && package.is_file(); 
}

