#[macro_use]
extern crate clap;

use clap::{Command, Arg}; //arg, ArgMatches
use clap_complete::Shell;
use anyhow::anyhow;
use log;


mod book;
mod book_builder;
mod page_builder;
mod commands;


const VERSION: &str = concat!("v", crate_version!());



fn main() {
    //setup logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    //create clap app, loading in the available commands
    fn create_clap_app() -> Command<'static> {
        Command::new(crate_name!())
            .about(crate_description!())
            .author("Domtron Vox <domtron.vox@gmail.com>")
            .version(VERSION)
            .propagate_version(true)
            .arg_required_else_help(true)
            .after_help(
                "For more information about a specific command, try `mdbookshelf <command> --help`\n\
                 ",
            )
            .subcommand(commands::build::make_subcommand_build())
            .subcommand(commands::serve::make_subcommand_serve())
            .subcommand(commands::clean::make_subcommand_clean())
            .subcommand(
                Command::new("completions")
                    .about("Generate shell completions for your shell to stdout")
                    .arg(
                        Arg::new("shell")
                            .takes_value(true)
                            .possible_values(Shell::possible_values())
                            .help("the shell to generate completions for")
                            .value_name("SHELL")
                            .required(true),
                    ),
            )
    }

    let app = create_clap_app();

    // Check which sub-command the user ran.
    let res = match app.get_matches().subcommand() {
        Some(("build", sub_matches)) => commands::build::execute_build(sub_matches),
        Some(("serve", sub_matches)) => commands::serve::execute_serve(sub_matches),
        Some(("clean", sub_matches)) => commands::clean::execute_clean(sub_matches),
        Some(("completions", sub_matches)) => (|| {
            let shell: Shell = sub_matches
                .value_of("shell")
                .ok_or_else(|| anyhow!("Shell name missing."))?
                .parse()
                .map_err(|s| anyhow!("Invalid shell: {}", s))?;

            let mut complete_app = create_clap_app();
            clap_complete::generate(
                shell,
                &mut complete_app,
                "mdbookshelf",
                &mut std::io::stdout().lock(),
            );
            Ok(())
        })(),
        _ => unreachable!(),
    };
    
    
    if let Err(err) = res {
        log::error!("{}", err);
    }    
}


