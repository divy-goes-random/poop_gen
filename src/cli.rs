use clap::{App, SubCommand};

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// Generates the clap argument parser
pub fn build_cli() -> App<'static, 'static> {
    App::new("Poop generator")
        .version(VERSION.unwrap_or("unknown"))
        .author("Divy Srivastava <dj.srivastava23@gmail.com>")
        .about("Skeleton project manager")
        .args_from_usage("-l, --lang=<LANG>  'Set language configuration'")
        .subcommand(SubCommand::with_name("new")
                        .about("create new project")
                        .arg_from_usage("<NAME>   'The project name'"))
        .subcommand(SubCommand::with_name("init").about("initialize existing project"))
}