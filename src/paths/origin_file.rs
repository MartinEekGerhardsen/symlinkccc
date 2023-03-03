pub trait OriginFile: super::path::Path + From<std::path::PathBuf> {
    fn exists(&self) -> bool {
        let path = self.path();
        path.exists() && path.is_file()
    }

    fn canonicalize(self) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let path = self.path();
        let path = path.canonicalize()?;
        Ok(Self::from(path))
    }
}
