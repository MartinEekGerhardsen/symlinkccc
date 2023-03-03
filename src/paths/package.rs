pub trait Package: super::path::Path {
    fn compile_commands_filename() -> &'static str {
        crate::config::COMPILE_COMMANDS_NAME
    }

    fn compile_commands(&self) -> std::path::PathBuf {
        self.path().join(Self::compile_commands_filename())
    }
}
