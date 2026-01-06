/**
 * src/main.rs
 */
mod tests;
use std::fmt;
use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(
    name = "Clap CLI",
    version = "1.0",
    author = "Mark Fehrenbacher",
    about = "A simple task list / todo CLI application using Clap"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task to tasks.json
    Add { task: String },
    /// List all tasks
    List,
    /// List only completed tasks
    ListCompleted,
    /// Mark a task as completed, but its ID
    Complete { id: usize },
    /// Remove a task from the task list, by its ID
    Remove { id: usize },
    // Note: Doc comments for subcommands must be placed on the corresponding
    // enum variant ^^
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Debug)]
enum TaskError {
    FileError(String),
    TaskNotFound(usize),
    InvalidInput(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::FileError(msg) => write!(f, "File Error: {}", msg),
            TaskError::TaskNotFound(id) => write!(f, "Task with ID {} not found", id),
            TaskError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl std::error::Error for TaskError {}

#[derive(Serialize, Deserialize, Debug)]
/// TaskList struct
///
/// # Fields
///
/// - `tasks` (`Vec<Task>`) - A vector of task structs
/// - `next_id` (`usize`) - The next "unique" id in the queue
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let s = TaskList {
///     tasks: value,
///     next_id: value,
/// };
/// ```
struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TaskList {
    fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// Load task list from file
    ///
    /// # Arguments
    ///
    /// - `filename` (`&str`)
    ///
    /// # Returns
    ///
    /// - `Result<Self, Box<dyn std::error::Error>>`
    ///
    /// # Errors
    ///
    /// Describe possible errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = load_from_file();
    /// ```
    fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if std::path::Path::new(filename).exists() {
            let contents = fs::read_to_string(filename)?;
            let task_list = serde_json::from_str(&contents)?;
            Ok(task_list)
        } else {
            Ok(TaskList::new())
        }
    }

    /// Save task list to a file
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`)
    /// - `filename` (`&str`)
    ///
    /// # Returns
    ///
    /// - `Result<(), Box<dyn std::error::Error>>` - Describe the return value.
    ///
    /// # Errors
    ///
    /// Describe possible errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = save_to_file();
    /// ```
    fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }

    /// Add a task to the task list
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`)
    /// - `description` (`String`) - Description of the task
    ///
    /// # Returns
    ///
    /// - `Result<(), TaskError>`
    ///
    /// # Errors
    ///
    /// Describe possible errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = add_task();
    /// ```
    fn add_task(&mut self, description: String) -> Result<(), TaskError> {
        if description.trim().is_empty() {
            return Err(TaskError::InvalidInput(
                "Task description cannot be empty".to_string(),
            ));
        }

        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };

        self.tasks.push(task);
        self.next_id += 1;

        Ok(())
    }

    /// Mark a task as completed
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`)
    /// - `id` (`usize`) - Numeric ID
    ///
    /// # Returns
    ///
    /// - `Result<(), TaskError>`
    ///
    /// # Errors
    ///
    /// Describe possible errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = complete_task();
    /// ```
    fn complete_task(&mut self, id: usize) -> Result<(), TaskError> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.completed = true;
                Ok(())
            }
            None => Err(TaskError::TaskNotFound(id)),
        }
    }

    /// List out all tasks
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`)
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = list_tasks();
    /// ```
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "{}. [{}] - {}",
                task.id,
                if task.completed { "x" } else { " " },
                task.description
            );
        }
    }

    /// List out only completed tasks
    ///
    /// # Arguments
    ///
    /// - `&self` (`undefined`)
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = list_completed_tasks();
    /// ```
    fn list_completed_tasks(&self) {
        for task in &self.tasks {
            if task.completed {
                println!(
                    "{}. [{}] - {}",
                    task.id,
                    if task.completed { "x" } else { " " },
                    task.description
                );
            }
        }
    }

    /// Remove a task by its ID
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`)
    /// - `id` (`usize`)
    ///
    /// # Returns
    ///
    /// - `Result<(), TaskError>`
    ///
    /// # Errors
    ///
    /// Describe possible errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = remove_task();
    /// ```
    fn remove_task(&mut self, id: usize) -> Result<(), TaskError> {
        let index = self.tasks.iter().position(|task| task.id == id);
        match index {
            Some(i) => {
                self.tasks.remove(i);
                Ok(())
            }
            None => Err(TaskError::TaskNotFound(id)),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_command(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Command runner
///
/// # Arguments
///
/// - `cli` (`Cli`)
///
/// # Returns
///
/// - `Result<(), Box<dyn std::error::Error>>` - Describe the return value.
///
/// # Errors
///
/// Describe possible errors.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let _ = run_command();
/// ```
fn run_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let filename = "tasks.json";
    let mut task_list =
        TaskList::load_from_file(filename).map_err(|e| TaskError::FileError(e.to_string()))?;

    match cli.command {
        Commands::Add { task } => {
            println!("Adding task: {}", task);
            task_list.add_task(task)?;
            task_list.save_to_file(filename)?;
        }
        Commands::List => {
            println!("Listing all tasks");
            task_list.list_tasks()
        }
        Commands::ListCompleted => {
            println!("Listing all completed tasks");
            task_list.list_completed_tasks();
        }
        Commands::Complete { id } => {
            println!("Completing task with ID: {}", id);
            task_list.complete_task(id)?;
            task_list.save_to_file(filename)?;
        }
        Commands::Remove { id } => {
            println!("Removing task with ID: {}", id);
            task_list.remove_task(2)?;
            task_list.save_to_file(filename)?;
        }
    }
    Ok(())
}
