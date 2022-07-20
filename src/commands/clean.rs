
use std::fs;


use clap::{Command, ArgMatches};
use log;


// Create clap subcommand arguments
pub fn make_subcommand_clean<'help>() -> Command<'help> {
    Command::new("clean")
        .about("Deletes a built bookshelf")
}

// Clean command implementation
pub fn execute_clean(_args: &ArgMatches) -> Result<(), anyhow::Error> {
    
    //TODO need to be pulling this from a config instead.
    let build_path = std::env::current_dir().unwrap().join("build").join("");

    if build_path.exists() {
        if let Result::Err(err) = fs::remove_dir_all(&build_path) {
            log::error!("Unable to remove the build directory during clean command. Error: {}", err);
        } else {
            log::info!("Build directory removed successfuly.")
        }
    } else {
        log::info!("Build directory does not exist.");
    }

    Ok(())
}
