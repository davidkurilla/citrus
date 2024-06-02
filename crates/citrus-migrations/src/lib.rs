use std::string::String;
use toml::Value;
use diesel::{pg::PgConnection, Connection};
use diesel_migrations::{FileBasedMigrations, HarnessWithOutput, MigrationHarness};

pub struct InputData {
    username: String,
    password: String,
    dbname: String,
    host: String,
    port: u16
}

fn get_str_or_default(toml: &Value, key: &str) -> String {
    let value = toml.get(key);

    let str =
    if value.is_none() {
        eprintln!("Could not read toml field: \"{}\"", key);
        ""
    } else {
        value.unwrap().as_str().unwrap_or_default()
    };

    String::from(str)
}
fn get_int_or_default(toml: &Value, key: &str) -> i64 {
    let value = toml.get(key);

    if value.is_none() {
        eprintln!("Could not read toml field: \"{}\"", key);
        0
    } else {
        value.unwrap().as_integer().unwrap_or_default()
    }
}

impl InputData {
    pub fn read(toml: &Value) -> InputData {
        InputData {
            username: get_str_or_default(toml, "username"),
            password: get_str_or_default(toml, "password"),
            dbname: get_str_or_default(toml, "dbname"),
            host: get_str_or_default(toml, "host"),
            port: get_int_or_default(toml, "port") as u16
        }
    }

    pub fn postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.dbname
        )
    }
}

pub fn run_migration(toml_file_path: std::path::PathBuf, toml_table: String) {

    let toml_file = toml_file_path;
    let table_name = toml_table;

    let toml_contents = match std::fs::read_to_string(toml_file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading TOML file: {}", e);
            return;
        }
    };

    let toml_data: Value = match toml_contents.parse() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing TOML file: {}", e);
            return;
        }
    };

    let table = get_toml_table(&table_name, &toml_data);

    let input = InputData::read(table);

    let db_url = input.postgres_url();

    println!("Attempting to connect to {}", db_url);

    let mut conn = PgConnection::establish(&db_url)
    .expect("Unable to connect");

    let migrations = FileBasedMigrations::find_migrations_directory()
        .expect("Could not read migrations directory");

    let mut harness = HarnessWithOutput::write_to_stdout(&mut conn);

    harness.run_pending_migrations(migrations).expect("Couldn't run migrations");

    println!("Successfully ran migrations")
}


pub fn get_toml_table<'a>(table_name: &'a str, toml_data: &'a Value) -> &'a Value{
    if !table_name.is_empty() {
        let table = toml_data.get(&table_name);
        if table.is_none() {
            eprintln!("Unable to find toml table: \"{}\"", &table_name);
            std::process::abort()
        }
        
        table.unwrap()
    } else {
        &toml_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
