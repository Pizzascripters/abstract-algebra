mod action;
mod command;
mod group;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        command::help(None);
        return;
    };
    let primary_arg = args[1].as_str();
    match primary_arg {
        "cc" => {
            if args.len() < 3 {
                command::bad_usage(primary_arg.to_string())
            } else {
                command::conjugacy_classes(args[2].to_string())
            }
        },
        "center" => {
            if args.len() < 3 {
                command::bad_usage(primary_arg.to_string())
            } else {
                command::center(args[2].to_string())
            }
        },
        "demo" => command::demo(),
        "help" => command::help(None),
        "members" => {
            if args.len() < 3 {
                command::bad_usage(primary_arg.to_string())
            } else {
                command::members(args[2].to_string())
            }
        },
        _ => command::help(Some(primary_arg.to_string()))
    }
}