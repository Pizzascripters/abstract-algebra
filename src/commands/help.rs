use std::cmp::max;

const COMMANDS: [Command; 3] = [
    Command {
        name: "demo",
        syntax: "",
        description: "Run for project demonstration"
    },
    Command {
        name: "help",
        syntax: "help [command]",
        description: ""
    },
    Command {
        name: "members",
        syntax: "members <group>",
        description: "Lists all the members of a group"
    },
];

struct Command {
    name: &'static str,
    syntax: &'static str,
    description: &'static str
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