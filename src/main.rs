use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, exit};

#[derive(Debug, Serialize, Deserialize)]
struct Job {
    name: String,
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    jobs: Vec<Job>,
}

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
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            run_task(name);
        }
        Some(("create", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let yaml_path = sub_m.value_of("yaml").unwrap();
            create_task(name, yaml_path);
        }
        Some(("list", _)) => {
            list_tasks();
        }
        Some(("delete", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            delete_task(name);
        }
        Some(("update", sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            let yaml_path = sub_m.value_of("yaml").unwrap();
            update_task(name, yaml_path);
        }
        _ => {
            println!("Welcome to citrus!");
        }
    }
}

fn run_task(name: &str) {
    // Load the task from the JSON file
    let file_path = format!("{}.json", name);
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: Unable to open task file '{}'. Error: {}", file_path, err);
            exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read task file");
    let task: Task = serde_json::from_str(&contents).expect("Unable to parse task file");

    for job in task.jobs {
        println!("Executing job: {}", job.name);
        let mut parts = job.command.split_whitespace();
        let command = parts.next().expect("Invalid command");
        let args: Vec<&str> = parts.collect();

        let status = Command::new(command)
            .args(&args)
            .status()
            .expect("Failed to execute command");

        if !status.success() {
            eprintln!("Error: Job '{}' failed with exit code {}", job.name, status);
            exit(1);
        }
    }
    println!("Task '{}' completed successfully.", task.name);
}

fn create_task(name: &str, yaml_path: &str) {
    let mut file = File::open(yaml_path).expect("Unable to open YAML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read YAML file");
    let jobs: Vec<Job> = serde_yaml::from_str(&contents).expect("Unable to parse YAML file");

    let task = Task {
        name: name.to_string(),
        jobs,
    };

    let file_path = format!("{}.json", name);
    save_task(&file_path, &task);

    println!("Created task: {:?}", task);
}

fn save_task(file_path: &str, task: &Task) {
    let content = serde_json::to_string_pretty(task).expect("Unable to serialize task");
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write file");
}

fn delete_task(name: &str) {
    let file_path = format!("{}.json", name);

    if Path::new(&file_path).exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => println!("Task '{}' deleted successfully.", name),
            Err(err) => eprintln!("Error: Unable to delete task file '{}'. Error: {}", file_path, err),
        }
    } else {
        eprintln!("Error: Task '{}' does not exist.", name);
    }
}

fn list_tasks() {
    let paths = fs::read_dir(".").expect("Unable to read directory");

    let mut tasks = Vec::new();
    for path in paths {
        let path = path.expect("Unable to read path").path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(task_name) = path.file_stem().and_then(|s| s.to_str()) {
                tasks.push(task_name.to_string());
            }
        }
    }

    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        println!("Tasks:");
        for task in tasks {
            println!("- {}", task);
        }
    }
}

fn update_task(name: &str, yaml_path: &str) {
    let file_path = format!("{}.json", name);
    if !Path::new(&file_path).exists() {
        eprintln!("Error: Task '{}' does not exist.", name);
        exit(1);
    }

    let mut file = File::open(yaml_path).expect("Unable to open YAML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read YAML file");
    let jobs: Vec<Job> = serde_yaml::from_str(&contents).expect("Unable to parse YAML file");

    let task = Task {
        name: name.to_string(),
        jobs,
    };

    save_task(&file_path, &task);

    println!("Updated task: {:?}", task);
}
