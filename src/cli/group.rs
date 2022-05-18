use clap::Arg;
use clap::ArgMatches;
use clap::Command;

pub struct GroupContext {
    pub id: String,
    pub elements: bool,
    pub conjugacy_classes: bool,
    pub center: bool
}

impl From<ArgMatches> for GroupContext {
    fn from(m: ArgMatches) -> Self {
        GroupContext {
            id: match m.value_of("id") {
                Some(id) => id.to_owned(),
                None => "".to_owned()
            },
            elements: m.is_present("elements"),
            conjugacy_classes: m.is_present("conjugacy-classes"),
            center: m.is_present("center")
        }
    }
}

pub fn group<'a>() -> Command<'a> {
    return Command::new("group")
        .about("Computes information about a group.")
        .arg(
            Arg::new("id")
                .help("Group identifier: A<n>, D<n>, Q8, S<n>, or Z<n>.")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::new("elements")
                .help("List all elements of a group.")
                .short('e')
                .long("elements")
                .takes_value(false)
        )
        .arg(
            Arg::new("conjugacy-classes")
                .help("List the conjugacy classes of a group.")
                .short('c')
                .long("conjugacy-classes")
                .takes_value(false)
        )
        .arg(
            Arg::new("center")
            .help("List elements that are in conjugacy classes of size 1.")
                .short('C')
                .long("center")
                .takes_value(false)
        );
}