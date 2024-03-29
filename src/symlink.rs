use crate::{
    paths::{
        package::Package,
        structs::{Build, BuildPackage, Source, SourcePackage},
    },
    workspace::find_enclosing,
};

#[derive(Debug, Clone)]
pub struct NoCompileCommandsError {}

impl std::fmt::Display for NoCompileCommandsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot find expected compile commands file")
    }
}

impl std::error::Error for NoCompileCommandsError {}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Workspace(crate::workspace::Error),
    NoCompileCommands(NoCompileCommandsError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<crate::workspace::Error> for Error {
    fn from(value: crate::workspace::Error) -> Self {
        Self::Workspace(value)
    }
}

impl From<NoCompileCommandsError> for Error {
    fn from(value: NoCompileCommandsError) -> Self {
        Self::NoCompileCommands(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn link_compile_commands(
    build_package_path: &BuildPackage,
    source_package_path: &SourcePackage,
) -> Result<()> {
    log::info!("Linking compile commands");

    let build_package_compile_commands_path = build_package_path.compile_commands();
    log::debug!(
        "Built compile commands file at: {}",
        build_package_compile_commands_path.display()
    );

    let source_package_compile_commands_path = source_package_path.compile_commands();
    log::debug!(
        "Source path for linking compile commands at: {}",
        source_package_compile_commands_path.display()
    );

    if source_package_compile_commands_path.exists()
        && source_package_compile_commands_path.is_file()
    {
        // TODO: Find some better way of handling especially this.
        // Potentially look into the catkin profile.
        // Or remove only if build folder is removed.
        // Or look into cargo watch
        log::debug!(
            "Removing existing symlink to: {}",
            source_package_compile_commands_path.display()
        );
        std::fs::remove_file(&source_package_compile_commands_path)?;
    }

    if !build_package_compile_commands_path.exists() {
        log::error!(
            "Cannot find built compile commands file: {}",
            build_package_compile_commands_path.display()
        );
        return Err(Error::NoCompileCommands(NoCompileCommandsError {}));
    }

    std::os::unix::fs::symlink(
        build_package_compile_commands_path,
        source_package_compile_commands_path,
    )?;
    log::info!("Linked compile commands");

    Ok(())
}
use crate::paths::package_container::PackageContainer;

pub fn link_all_compile_commands() -> Result<()> {
    let current_working_directory = std::env::current_dir()?;
    log::debug!(
        "Current working directory: {}",
        current_working_directory.display()
    );

    let workspace = find_enclosing(current_working_directory)?;
    log::debug!("Current workspace: {workspace}");

    let source = Source::from(&workspace);
    let source_packages = source.get_all_package_paths();
    log::debug!("All package names to package sources\n: {source_packages:?}");

    let build = Build::from(&workspace);
    let build_packages = build.get_all_package_paths();
    log::debug!("All package names to built packages\n: {build_packages:?}");

    for (package_name, build_package) in build_packages {
        if let Some(source_package) = source_packages.get(&package_name) {
            log::debug!("Linking {package_name}");
            log::debug!("From {build_package:?}");
            log::debug!("To {source_package:?}");
            link_compile_commands(&build_package, source_package)?;
        } else {
            log::warn!("Built package '{package_name}' cannot be found among the source packages.");
            log::info!("This might be because this 'package' is only an umbrella for other packages, and therefore doens't show up in `rospack list`.");
        }
    }

    Ok(())
}
