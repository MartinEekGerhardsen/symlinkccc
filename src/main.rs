mod build;
mod build_package;
mod config;
mod package;
mod ros_paths;
mod source_package;
mod symlink;
mod workspace;

use symlink::link_all_compile_commands;

use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Foo
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
}

fn main() {
    log::trace!("Starting symlinkccc");
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    match link_all_compile_commands() {
        Ok(()) => {
            log::info!("Linking completed successfully");
        }
        Err(err) => {
            log::error!("Cannot link compile commands:\n{err:?}");
        }
    }
}
