use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("citrus")
        .version("1.0")
        .author("David Kurilla")
        .about("A simple task automation tool")
        .subcommand(
            SubCommand::with_name("run")
                .about("Run a task")
                .arg(Arg::with_name("name")
                    .help("The name of the task to run")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a task")
                .arg(Arg::with_name("name")
                    .help("The name of the task")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("yaml")
                    .help("The YAML file defining the jobs")
                    .required(true)
                    .index(2))
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List tasks")
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete a task")
                .arg(Arg::with_name("name")
                    .help("The name of the task to delete")
                    .required(true)
                    .index(1))
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update a task")
                .arg(Arg::with_name("name")
                    .help("The name of the task to update")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("yaml")
                    .help("The YAML file defining the new jobs")
                    .required(true)
                    .index(2))
        )
        .subcommand(
            SubCommand::with_name("migrations")
                .about("Run diesel migrations")
                .arg(Arg::with_name("table-name")
                    .help("Specify table name in 'citrus-config.toml' file to run migrations. Defaults to 'database'")
                    .required(false)
                    .index(1))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            citrus_core::run_task(name);
        }
        Some(("create", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let yaml_path = sub_m.value_of("yaml").unwrap();
            citrus_core::create_task(name, yaml_path);
        }
        Some(("list", _)) => {
            citrus_core::list_tasks();
        }
        Some(("delete", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            citrus_core::delete_task(name);
        }
        Some(("update", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let yaml_path = sub_m.value_of("yaml").unwrap();
            citrus_core::update_task(name, yaml_path);
        }
        Some(("migrations", sub_m)) => {
            let table_name = sub_m.value_of("table-name").unwrap();
            citrus_migrations::run_migration("citrus-config.toml".into(), table_name.to_string());
        }
        _ => {
            println!("Welcome to citrus!");
        }
    }
}
