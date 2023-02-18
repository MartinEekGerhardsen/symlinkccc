use crate::ros_paths::{Build, Workspace};

pub fn get_build_path(Workspace(workspace_path): &Workspace) -> Build {
    Build(workspace_path.join("build"))
}
