use crate::paths::path::Path;

#[macro_export]
macro_rules! generate_paths {
    ( $( $x:ident ), * ) => {
        $(
            #[derive(Debug)]
            pub struct $x(pub(crate) std::path::PathBuf);

            impl Path for $x {
                fn path(&self) -> std::path::PathBuf {
                    let $x(path) = self;
                    path.to_path_buf()
                }
            }

            impl<P: AsRef<std::path::Path>> From<P> for $x {
                fn from(path: P) -> Self {
                    $x(path.as_ref().to_path_buf())
                }
            }

            impl std::fmt::Display for $x {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{self:?}")
                }
            }

        )*
    };
}

generate_paths![
    Workspace,
    Build,
    BuildPackage,
    BuildPackageCompileCommands,
    Source,
    SourcePackage,
    SourcePackageCompileCommands,
    SourcePackageXML,
    SourcePackageCMakeLists
];

impl super::package::Package for SourcePackage {}

impl super::package::Package for BuildPackage {}

impl super::origin_file::OriginFile for BuildPackageCompileCommands {}

impl super::origin_file::OriginFile for SourcePackageXML {}

impl super::origin_file::OriginFile for SourcePackageCMakeLists {}

impl From<&Workspace> for Build {
    fn from(Workspace(path): &Workspace) -> Self {
        Self(path.join("build"))
    }
}

impl From<&Workspace> for Source {
    fn from(Workspace(path): &Workspace) -> Self {
        Self(path.join("src"))
    }
}

impl From<&SourcePackage> for SourcePackageXML {
    fn from(SourcePackage(path): &SourcePackage) -> Self {
        Self(path.join("package.xml"))
    }
}

impl From<&SourcePackage> for SourcePackageCMakeLists {
    fn from(SourcePackage(path): &SourcePackage) -> Self {
        Self(path.join("CMakeLists.txt"))
    }
}
