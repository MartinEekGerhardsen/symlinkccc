use std::path::PathBuf;

#[derive(Debug)]
pub struct Workspace(pub(crate) PathBuf);
#[derive(Debug)]
pub struct Profile(pub(crate) PathBuf);
#[derive(Debug)]
pub struct Build(pub(crate) PathBuf);
#[derive(Debug)]
pub struct Source(pub(crate) PathBuf);
#[derive(Debug)]
pub struct SourcePackage(pub(crate) PathBuf);
#[derive(Debug)]
pub struct BuildPackage(pub(crate) PathBuf);
