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

pub fn get_packages<P: AsRef<Path>>(path: P) -> Result<Vec<String>, CommandAndUTF8Error> {
    let catkin_list_unformatted = Command::new("catkin")
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

pub fn get_package_path<P: AsRef<Path>>(path: P, package: &str) -> Result<String, CommandAndUTF8Error> {
    let rospack_find_package = Command::new("rospack")
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

pub fn get_all_package_paths<P: AsRef<Path>>(path: P) -> Result<HashMap<String, PathBuf>, CommandAndUTF8Error> {
    let rospack_list = Command::new("rospack")
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

fn get_dir<P: AsRef<Path>>(path: P, flag: &str) -> Result<String, CommandAndUTF8Error> {
    let catkin_locate_flag = Command::new("catkin")
        .args(["locate", flag])
        .current_dir(path)
        .output();

    match catkin_locate_flag {
        Ok(catkin_locate_flag) => {
            let catkin_locate_flag = String::from_utf8(catkin_locate_flag.stdout); 

            match catkin_locate_flag {
                Ok(catkin_locate_flag) => Ok(catkin_locate_flag),
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

pub fn is_package<S: AsRef<OsStr> + ?Sized>(path: &S) -> bool {
    let cmakelists = std::path::Path::new(&path).join("CMakeLists.txt"); 
    let package = std::path::Path::new(&path).join("package.xml"); 

    return cmakelists.is_file() && package.is_file(); 
}

