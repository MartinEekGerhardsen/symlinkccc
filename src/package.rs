use std::fmt::Display;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Package(pub(crate) String);

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(package) = self;
        write!(f, "{package}")
    }
}
