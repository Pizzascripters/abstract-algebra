mod action;

mod cli;
use cli::Context;

mod command;

mod group;

mod util;

fn main() {
    match cli::get_context() {
        Context::Demo => {
            command::demo()
        },
        Context::Group(group_context) => {
            command::group(group_context)
        },
        _ => println!("No subcommand specified. Use abstract-algebra --help for a list of subcommands.")
    }
}