# citrus - a simple task manager

<p align="center">
  <img src="citrus.png" />
</p>

citrus is a simple task manager to assist your projects. With citrus, you can set up tasks containing lists of jobs and commands. 
You can then tell citrus to execute these functions via the command line with citrus-cli. Citrus is easy to install using `cargo`.

## Installing citrus-cli
To install citrus-cli, simply use `cargo`.

```shell
cargo install citrus-cli
```

## Using citrus-cli
Using citrus-cli is easy. Simply invoke the program using the command `citrus-cli`.

### Configuring citrus-cli
Before you can utilize `citrus-cli`'s task management features, you must configure it by creating a `citrus-config.toml` file.
Start by creating a file in your project root directory called `citrus-config.toml`.
Complete the file so that it looks like this:
```toml
[config]
task_directory = "path/to/desired/directory"
```
Save the file and now you may use `citrus-cli`.

### Creating a task
Create a task by setting up a YAML file. Tasks are a list of jobs that can be represented in the following format
```yaml
- name: JobName
  command: "command"
```
You may list as many of these as you want in your file. You may name the YAML file anything you want.

To create the task simply run the following command
```shell
citrus-cli create [task name] [file path to YAML file]
```
TaskName can be whatever you wish to name the task.
task.yaml is the file path to the YAML file you will use to set up the task

You will notice citrus-cli output a JSON file. This file is important as it contains all of the task information.

### Running a task
To run a task simply use the following command
```shell
citrus-cli run [task name]
```

### List tasks
To list existing tasks run the following command
```shell
citrus-cli list
```

### Deleting a task
To delete existing tasks run the following command
```shell
citrus-cli delete [task name]
```

### Updating a task
To update a task by replacing it with a new YAML configuration run the following command
```shell
citrus-cli update [task name] [file path to YAML file]
```
