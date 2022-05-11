mod action;
mod commands;
mod group;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        commands::help(None);
        return;
    };
    let primary_arg = args[1].as_str();
    match primary_arg {
        "cc" => {
            if args.len() < 3 {
                commands::bad_usage(primary_arg.to_string())
            } else {
                commands::conjugacy_classes(args[2].to_string())
            }
        },
        "center" => {
            if args.len() < 3 {
                commands::bad_usage(primary_arg.to_string())
            } else {
                commands::center(args[2].to_string())
            }
        },
        "demo" => commands::demo(),
        "help" => commands::help(None),
        "members" => {
            if args.len() < 3 {
                commands::bad_usage(primary_arg.to_string())
            } else {
                commands::members(args[2].to_string())
            }
        },
        _ => commands::help(Some(primary_arg.to_string()))
    }
}