use serde::{Serialize, Deserialize};
use serde_json;

use crate::catkin;
use crate::packagelink; 

use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::vec::Vec;
use std::path::{Path, PathBuf}; 
use std::fs::File;
use std::collections::HashMap;  

pub const SYMLINK_DATA_FILENAME: &str = "symlinkccc_data.json";

#[derive(Debug)]
pub enum SymlinkDataError {
    JSON(serde_json::Error), 
    IO(std::io::Error),
}  

#[derive(Serialize, Deserialize, Debug)]
pub struct SymlinkData {
    pub packagelinks: Vec<packagelink::PackageLink>, 
}

impl SymlinkData {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, catkin::CommandAndUTF8Error> {
        let _source_dir = catkin::get_src_dir(&path)?; 
        let build_dir = catkin::get_build_dir(&path)?;

        let packages = catkin::get_packages(&path)?;
        let package_source_paths = catkin::get_all_package_paths(&path)?; 
        
        let build_dir = Path::new(&build_dir); 
        let build_dir = build_dir.to_path_buf(); 

        let packagelinks = packages.iter().filter_map(
            |package| create_package_link(
                package, &build_dir, &package_source_paths
            )
        ).collect(); 

        Ok(Self {
            packagelinks
        } )
    }

    pub fn all_links_valid(&self) -> bool {
        self.packagelinks.iter().all(|pl| pl.is_linked())
    }

    pub fn update_invalid_links<P: AsRef<Path>>(&mut self, path: P) 
            -> Result<(), catkin::CommandAndUTF8Error> {
        let build_dir = catkin::get_build_dir(&path)?;
        let build_dir = Path::new(&build_dir).to_path_buf(); 

        for link in self.packagelinks {
            if ! link.is_linked() {
                let new_link = packagelink::PackageLink::new(
                    &link.package, build_dir, link.source_dir
                ); 
                
                link.update(&link); 
            }
        }

        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), SymlinkDataError> {
        let json_data = serde_json::to_string(self);

        match json_data {
            Ok(json_data) => {
                let symlink_data_file = File::create(path); 

                match symlink_data_file {
                    Ok(mut symlink_data_file) => {
                        let data_write = symlink_data_file.write_all(&json_data.as_bytes()); 

                        match data_write {
                            Ok(()) => Ok(()), 
                            Err(err) => Err(SymlinkDataError::IO(err)),
                        }
                    }, 
                    Err(err) => Err(SymlinkDataError::IO(err)),
                }
            }, 
            Err(err) => Err(SymlinkDataError::JSON(err)),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, SymlinkDataError> {
        let data_file = File::open(path);

        match data_file {
            Ok(data_file) => {
                let reader = BufReader::new(data_file);

                match serde_json::from_reader(reader) {
                    Ok(data) => Ok(data), 
                    Err(err) => Err(SymlinkDataError::JSON(err)),
                }
            }, 
            Err(err) => Err(SymlinkDataError::IO(err)),
        }
    }
}

fn create_package_link(package: &str, build_path: &PathBuf, p2sp: &HashMap<String, PathBuf>) 
        -> Option<packagelink::PackageLink> {
    let build_path = build_path.join(package);
    match p2sp.get(package) {
        Some(source_path) => Some(
            packagelink::PackageLink::new(package, build_path.clone(), source_path.clone())
        ), 
        None => None
    }
}
