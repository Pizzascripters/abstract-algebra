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
            if group_context.elements {
                command::members(group_context.id.to_owned())
            }
            if group_context.conjugacy_classes {
                command::conjugacy_classes(group_context.id.to_owned())
            }
            if group_context.center {
                command::center(group_context.id.to_owned())
            }
        },
        _ => println!("No subcommand specified. Use abstract-algebra --help for a list of subcommands.")
    }
}