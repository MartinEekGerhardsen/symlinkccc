#[macro_export]
macro_rules! generate_path_structs {
    ( $( $x:ident ), * ) => {
        $(
            pub struct $x(pub(crate) std::path::PathBuf);

            impl super::path::Path for $x {
                fn path(&self) -> std::path::PathBuf {
                    let $x(path) = self;
                    path.to_path_buf()
                }
            }
        )*
    };
}

generate_path_structs![Workspace, Build, Source, SourcePackage, BuildPackage];

impl super::package::Package for SourcePackage {}

impl super::package::Package for BuildPackage {}
