mod demo;
pub use demo::demo;

mod group_command;
pub use group_command::center::center;
pub use group_command::conjugacy_classes::conjugacy_classes;
pub use group_command::members::members;

mod help;
pub use help::help;
pub use help::bad_usage;