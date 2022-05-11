use std::env;
use std::cmp::max;

// For printing usage
const COMMANDS: [Command; 2] = [
    Command {
        name: "demo",
        syntax: "",
        description: "Run for project demonstration"
    },
    Command {
        name: "help",
        syntax: "help [command]",
        description: ""
    }
];

struct Command {
    name: &'static str,
    syntax: &'static str,
    description: &'static str
}

pub enum Mode {
    Help(Option<String>),
    Demo
}

pub fn parse() -> Mode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return Mode::Help(None) };
    let primary_arg = args[1].as_str();
    match primary_arg {
        "demo" => Mode::Demo,
        "help" => Mode::Help(None),
        _ => Mode::Help(Some(primary_arg.to_string()))
    }
}

pub fn help(command: Option<String>) {
    match command {
        Some(command) => println!("Unknown command \"{}\"\nCommands:", command),
        None => println!("Commands:")
    }

    let max_name = COMMANDS.iter().fold(0, |l, c| max(l, c.name.len()));
    let max_syntax = COMMANDS.iter().fold(0, |l, c| max(l, c.syntax.len()));
    for command in COMMANDS {
        println!("   {:name$}   {:syntax$}   {}", command.name, command.syntax, command.description, name=max_name, syntax=max_syntax);
    }
}