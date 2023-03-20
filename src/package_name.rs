use std::fmt::Display;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct PackageName(pub(crate) String);

impl Display for PackageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(package) = self;
        write!(f, "{package}")
    }
}
