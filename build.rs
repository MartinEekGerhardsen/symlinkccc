use const_gen::*;
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("const_config.rs");

    let const_declarations = vec![
        const_declaration!(pub METADATA_DIR_NAME = ".catkin_tools"),
        const_declaration!(pub ROSPACK_COMMAND = "rospack"),
        const_declaration!(pub BUILD_IGNORE_DIRS = ["build", "catkin_tools_prebuild"]),
        const_declaration!(pub COMPILE_COMMANDS_NAME = "compile_commands.json"),
    ]
    .join("\n");

    fs::write(dest_path, const_declarations).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
