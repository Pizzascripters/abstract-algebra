use clap::Command;

pub fn demo<'a>() -> Command<'a> {
    return Command::new("demo")
        .about("Give a project demonstration.");
}