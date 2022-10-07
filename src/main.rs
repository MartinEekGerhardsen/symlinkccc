mod catkin; 
mod symlinkdata;
mod packagelink; 

use std::vec::Vec; 
use std::path::{Path, PathBuf}; 
use std::time::Instant; 

use clap::Parser; 

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    remove_old_symlink: bool,
    #[arg(short, long)]
    debug: bool,
}


fn main() {
    let path = std::env::current_dir().expect("Cannot find current directory");

    let args = Args::parse(); 
    println!("{:?}", args); 

    let build_path = catkin::get_build_dir(&path).expect("Cannot find current build directory.");
    let build_path = Path::new(&build_path);

    let data_path = build_path.join(symlinkdata::SYMLINK_DATA_FILENAME); 
    let data = if data_path.is_file() {
        symlinkdata::SymlinkData::load(&data_path)
            .expect("Cannot load default path to symlink data")
    } else {
        symlinkdata::SymlinkData::new(&path)
            .expect("Cannot create new symlink data")
    };

    if data.all_links_valid() {
        println!("All links are valid"); 
    } else {
        
    }

    /* 
    let mut package_info_vec: Vec<packagelink::PackageLink> = Vec::new(); 

    let packages = catkin::get_packages(&path); 

    let build_dir = unwrap!(
        catkin::get_build_dir(&path), 
        "Cannot get catkin build dir"
    ); 
    let build_path = std::path::Path::new(build_dir.trim());

    let findtime = Instant::now(); 
    let package_to_path = catkin::get_all_package_paths(&path);

    for package in packages {
        if let Some(src_path) = package_to_path.get(&package) {
            let inf = packagelink::PackageLink {
                package: package.to_string(),
                build_path: build_path.join(&package), 
                source_path: src_path.to_path_buf(),
            }; 
            
            //pinfo.push(inf); 
            package_info_vec.push(inf);
        }
    }
    let findelapsed = findtime.elapsed();
    println!("rospack find took: {:.2?}", findelapsed);

    for info in package_info_vec {
        if verbose {
            println!("Working on {}", info.package);
        }
        let compile_commands_path = info.build_path.join("compile_commands.json"); 

        if compile_commands_path.is_file() && info.source_path.is_dir() {
            let canonicalized_compile_commands_path = std::fs::canonicalize(
                &compile_commands_path
             );

            let canonicalized_compile_commands_path = unwrap!(
                canonicalized_compile_commands_path, 
                "Cannot canonicalize original compile commands:\n{}", 
                info.compile_commands_path.display()
            );

            let symlinked_compile_commands_path = info
                .source_path
                .join("compile_commands.json");

            if symlinked_compile_commands_path.is_file() {
                let unsymlinked_compile_commands_path = std::fs::canonicalize(
                    &symlinked_compile_commands_path
                );

                let unsymlinked_compile_commands_path = unwrap!(
                    unsymlinked_compile_commands_path, 
                    "Cannot canonicalize symlinked compile commands:\n{}",
                    symlinked_compile_commands_path.display()
                );

                let broken_symlink = unsymlinked_compile_commands_path != compile_commands_path;

                if remove_old_compile_commands || broken_symlink {
                    if remove_old_compile_commands && verbose {
                        println!(
                            "Removing symlink\n:{}",
                            symlinked_compile_commands_path.display()
                        );
                    }
                    if broken_symlink && verbose {
                        println!(
                            "Removing broken symlink\n:{}",
                            symlinked_compile_commands_path.display()
                        );
                    }

                    unwrap!(
                        std::fs::remove_file(&symlinked_compile_commands_path), 
                        "Cannot remove symlinked compile commands:\n{}", 
                        symlinked_compile_commands_path.display()
                    );

                    if create_non_existent_symlinks {
                        if verbose {
                            println!(
                                "Symlinking new compile commands:\n{}\nto new src:\n{}", 
                                info.compile_commands_path.display(), 
                                symlinked_compile_commands_path.display()
                            );
                        }

                        unwrap!(
                            std::os::unix::fs::symlink(
                                &info.compile_commands_path, 
                                &symlinked_compile_commands_path
                            ),
                            "Cannot symlink new compile commands:\n{}\nto new src:\n{}", 
                            info.compile_commands_path.display(), 
                            symlinked_compile_commands_path.display()
                        );
                    }
                }
            } else {

                if create_non_existent_symlinks {
                    if verbose {
                        println!(
                            "Symlinking new compile commands:\n{}\nto new src:\n{}", 
                            info.compile_commands_path.display(), 
                            symlinked_compile_commands_path.display()
                        );
                    }
                    
                    unwrap!(
                        std::os::unix::fs::symlink(
                            &info.compile_commands_path, 
                            &symlinked_compile_commands_path
                        ),
                        "Cannot symlink new compile commands:\n{}\nto new src:\n{}", 
                        info.compile_commands_path.display(), 
                        symlinked_compile_commands_path.display()
                    );
                }
            }
        }
    }
    */
    /*
    if let Ok(p) = path {
        println!("{}", p.display());
        for entry in WalkDir::new(&p) {
            let entry = entry.unwrap(); 
            // println!("{}", entry.path().display()); 
            if catkin::is_package(entry.path()) {
                println!("{}", entry.path().display())
            }
        }

        let packages = catkin::get_packages(&p); 

        let build_base = std::path::Path::new(
            catkin::get_build_dir(&p)
            .expect("Cannot find build directory")
            .trim()
        );
        let build_directories = packages.iter()
            .map(|package| build_base.join(package))
            .collect(); 

        if let Ok(build) = build {
            println!("build path: {}", build); 
            if std::path::Path::new(build.trim()).is_dir() {
                assert!(std::env::set_current_dir(build.trim()).is_ok()); 
                println!("whoop");
            }
            else {
                println!("noop");
            }
        }
    }
    */
}
