use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Task Automation Tool")
        .version("1.0")
        .author("David Kurilla")
        .about("A simple task automation tool")
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a task")
        )
	.subcommand(
	    SubCommand::with_name("create")
		.about("Create a task")
	)
        .get_matches();

    match matches.subcommand_name() {
        Some("run") => {
            println!("Running task...");
        }
	Some("create") => {
	    println!("Creating task...");
	}
        _ => {
            println!("Welcome to Citrus!");
        }
    }
}

