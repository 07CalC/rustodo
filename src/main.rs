use tabled::{Table, Tabled};
use crossterm::{execute, terminal::{Clear, ClearType}};
use std::{io::stdout, path::PathBuf};
use clap::{Parser};

#[allow(non_snake_case)]
#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    isCompleted: bool,
}

#[derive(Tabled)]
struct TaskRow {
    id: u32,
    name: String,
    status: String,
}

#[derive(Parser)]
#[clap(name = "todo")]
#[clap(about = "A simple terminal-based todo list in Rust", long_about = None)]
struct cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    List,
    Add { name: String },
    Complete { id: u32 },
    Uncomplete { id: u32 },
    Delete { id: u32 },
}

fn getTasks() -> Vec<Task> {
    let connection = sqlite::open("../database/tasks.db").unwrap();
    let mut tasks: Vec<Task> = vec![];
    connection.iterate("SELECT * FROM tasks", |task| {
        tasks.push(Task {
            id: task[0].1.unwrap().parse().unwrap(),
            name: task[1].1.unwrap().to_string(),
            isCompleted: match task[2].1.unwrap() {
                "0" => false,
                "1" => true,
                _ => false,
                
            },
        });
        true
    }).unwrap();
    return tasks;
}

fn addTask(name: &str) {
    let connection = sqlite::open("../database/tasks.db").unwrap();
    let query = format!("INSERT INTO tasks (name, isCompleted) VALUES ('{}', 0)", name);
    connection.execute(&query).unwrap();
}

fn completeTask(id: u32){
    let connection = sqlite::open("../database/tasks.db").unwrap();
    let query = format!("UPDATE tasks SET isCompleted = 1 WHERE id = {}", id);
    match connection.execute(&query) {
        Ok(_) => println!("Task completed successfully"),
        Err(error) => println!("{}", error),

    }
    
}

fn uncompleteTask(id: u32){
    let connection = sqlite::open("../database/tasks.db").unwrap();
    let query = format!("UPDATE tasks SET isCompleted = 0 WHERE id = {}", id);
    match connection.execute(&query) {
        Ok(_) => println!("Task uncompleted successfully"),
        Err(error) => println!("{:?}", error),
    }
}

fn deleteTask(id: u32){
    let connection = sqlite::open("../database/tasks.db").unwrap();
    let query = format!("DELETE FROM tasks WHERE id = {};", id);
    match connection.execute(query) {
        Ok(_) => println!("Task deleted successfully"),
        Err(error) => println!("{:?}", error),
    }
    
    
}

fn listTasks(){
    let tasks: Vec<Task> = getTasks();
    let mut rows = Vec::new();
    for task in tasks.iter(){
        let status = if task.isCompleted { "✓ Completed".to_string() } else { "⏳ Pending".to_string() };
        rows.push(TaskRow {
            id: task.id,
            name: task.name.to_string(),
            status: status.to_string(),
        });

    }

    let table = Table::new(rows);
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("{}", table);
}

fn getDbPath() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not find home directory");
    path.push(".rustodo/tasks.db");
    path
}

fn main() {
    let cli = cli::parse();
    let dbPath = getDbPath();
    std::fs::create_dir_all(dbPath.parent().unwrap()).unwrap();
    let connection = sqlite::open(dbPath).unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            isCompleted BOOLEAN NOT NULL
        )",
        )
        .unwrap();
    match &cli.command{
        Commands::List => listTasks(),
        Commands::Add { name } => {addTask(name); listTasks()},
        Commands::Complete { id } => {completeTask(*id); listTasks()},
        Commands::Uncomplete { id } => {uncompleteTask(*id); listTasks()},
        Commands::Delete { id } => {deleteTask(*id); listTasks()},
    }

}