use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, exit};
use toml::Value;

// Job Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    name: String,
    command: String,
}

// Task Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    name: String,
    jobs: Vec<Job>,
}

// Run Task Function
pub fn run_task(name: &str) {
    
    // Load the task from the JSON file
    let output_directory = match get_config_file() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting config file: {}", e);
            return;
        }
    };

    // Get File path and file
    let file_path = format!("{}/{}.json", output_directory, name);
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error: Unable to open task file '{}'. Error: {}", file_path, err);
            exit(1);
        }
    };

    // Get file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read task file");
    
    // Create Task
    let task: Task = serde_json::from_str(&contents).expect("Unable to parse task file");

    // Create iterate through Task and execute Jobs
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

// Create Task
pub fn create_task(name: &str, yaml_path: &str) {
    let mut file = File::open(yaml_path).expect("Unable to open YAML file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read YAML file");
    let jobs: Vec<Job> = serde_yaml::from_str(&contents).expect("Unable to parse YAML file");

    let task = Task {
        name: name.to_string(),
        jobs,
    };

    let output_directory = match get_config_file() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting config file: {}", e);
            return;
        }
    };

    let file_path = format!("{}/{}.json", output_directory, name);
    save_task(&file_path, &task);

    println!("Created task: {:?}", task);
}

// Save Task
pub fn save_task(file_path: &str, task: &Task) {
    let content = serde_json::to_string_pretty(task).expect("Unable to serialize task");
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(content.as_bytes()).expect("Unable to write file");
}

// Delete Task
pub fn delete_task(name: &str) {
    let output_directory = match get_config_file() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting config file: {}", e);
            return;
        }
    };
    let file_path = format!("{}/{}.json", output_directory, name);

    if Path::new(&file_path).exists() {
        match fs::remove_file(&file_path) {
            Ok(_) => println!("Task '{}' deleted successfully.", name),
            Err(err) => eprintln!("Error: Unable to delete task file '{}'. Error: {}", file_path, err),
        }
    } else {
        eprintln!("Error: Task '{}' does not exist.", name);
    }
}

// List Tasks
pub fn list_tasks() {
    
    // Get Config File
    let output_directory = match get_config_file() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting config file: {}", e);
            return;
        }
    };
    let paths = fs::read_dir(output_directory).expect("Unable to read directory");

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

// Update Task
pub fn update_task(name: &str, yaml_path: &str) {
    let output_directory = match get_config_file() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Error getting config file: {}", e);
            return;
        }
    };
    // Check if Task exists
    let file_path = format!("{}/{}.json", output_directory, name);
    if !Path::new(&file_path).exists() {
        eprintln!("Error: Task '{}' does not exist.", name);
        exit(1);
    }

    // Create and Save new Task
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

// Get Config File
fn get_config_file() -> Result<String, &'static str> {
    let mut config_file = File::open("citrus-config.toml").expect("Could not locate 'citrus-config.toml' in the current directory");
    let mut contents = String::new();
    config_file.read_to_string(&mut contents).expect("Could not read file!");
    let value: Value = toml::from_str(&contents).expect("Could not parse TOML file!");
    if let Some(task_directory) = value.get("config").and_then(|config| config.get("task_directory")).and_then(|v| v.as_str()) {
        Ok(task_directory.to_string())
    } else {
        Err("Could not find 'task_directory' key in TOML file or it is not a string")
    }
}

// Init
pub fn init() {

    let file_data = "[config]\ntask_directory = \".citrus\"";

    let _config_file = match File::open("citrus-config.toml") {
        Ok(_file) => {
            println!("'citrus-config.toml' file found. Project already initialized.");
            return;
        }
        Err(_) => {
            let mut file = File::create("citrus-config.toml").expect("Cannot create 'citrus-config.toml'");
            file.write_all(file_data.as_bytes()).expect("Cannot write to 'citrus-config.toml'");
            let _dir = std::fs::create_dir(".citrus");
            file
        }
    };

    println!("Project successfully initialized!");
}

#[cfg(test)]
mod tests {
    use super::*;
}
