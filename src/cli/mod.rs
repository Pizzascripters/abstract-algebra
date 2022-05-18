mod demo;

mod group;
pub use group::GroupContext;

use clap::ArgMatches;
use clap::Command;

pub enum Context {
    Demo,
    Group(GroupContext),
    None
}

impl From<ArgMatches> for Context {
    fn from(m: ArgMatches) -> Self {
        match m.subcommand() {
            Some(("demo", _)) => Context::Demo,
            Some(("group", sub_m)) => Context::Group(GroupContext::from(sub_m.clone())),
            _ => Context::None
        }
    }
}

pub fn get_context() -> Context {
    let cmd = Command::new("abstract-algebra")
        .version("1.0")
        .about("An abstract algebra cli written in Rust.")
        .subcommand(demo::demo())
        .subcommand(group::group());
    Context::from(cmd.get_matches())
    // match cmd.get_matches().subcommand() {
    //     Some(("demo", _)) => Context::Demo,
    //     Some(("group", _)) => Context::Group(),
    //     _ => panic!("No subcommand specified. Use abstract-algebra --help for a list of subcommands.")
    // }
}