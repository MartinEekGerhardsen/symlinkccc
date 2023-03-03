use core::fmt::{self, Display};
use std::path::Path;

use crate::paths::structs::Workspace;

#[derive(Clone, Debug)]
pub struct InvalidWorkspaceError;

impl std::error::Error for InvalidWorkspaceError {}

impl Display for InvalidWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid workspace. Cannot find valid ROS workspace.")
    }
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Workspace(InvalidWorkspaceError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<InvalidWorkspaceError> for Error {
    fn from(value: InvalidWorkspaceError) -> Self {
        Self::Workspace(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn find_enclosing<P: AsRef<Path>>(search_start_path: P) -> Result<Workspace> {
    let mut search_next_path = search_start_path.as_ref().to_path_buf();
    log::debug!(
        "Start looking for workspace from: {}",
        search_next_path.display()
    );

    loop {
        let potential_path = search_next_path.join(crate::config::METADATA_DIR_NAME);
        log::debug!("Looking for: {}", potential_path.display());

        if potential_path.exists() && potential_path.is_dir() {
            let search_next_path = search_next_path.canonicalize()?;
            log::info!("Found workspace path: {}", search_next_path.display());
            return Ok(Workspace(search_next_path));
        } else if let Some(parent_path) = search_next_path.parent() {
            search_next_path = parent_path.to_path_buf();
        } else {
            log::error!(
                "Cannot find ROS workspace looking up from {}",
                search_start_path.as_ref().display()
            );
            return Err(Error::Workspace(InvalidWorkspaceError {}));
        }
    }
}
