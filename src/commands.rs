pub mod demo;
pub mod help;

pub fn parse(args: Vec<String>) {
    if args.len() < 2 {
        help::help(None);
        return;
    };
    let primary_arg = args[1].as_str();
    match primary_arg {
        "demo" => demo::demo(),
        "help" => help::help(None),
        _ => help::help(Some(primary_arg.to_string()))
    }
}