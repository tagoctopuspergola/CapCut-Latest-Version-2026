use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const TODO_FILE: &str = "todos.txt";

#[derive(Debug)]
struct Todo {
    id: usize,
    text: String,
    done: bool,
}

impl Todo {
    fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() != 3 {
            return None;
        }

        let id = parts[0].parse().ok()?;
        let done = parts[1] == "1";
        let text = parts[2].to_string();

        Some(Todo { id, text, done })
    }

    fn to_line(&self) -> String {
        format!(
            "{}|{}|{}",
            self.id,
            if self.done { "1" } else { "0" },
            self.text
        )
    }
}

fn load_todos() -> io::Result<Vec<Todo>> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(TODO_FILE)?;
    let todos = content.lines().filter_map(Todo::from_line).collect();

    Ok(todos)
}

fn save_todos(todos: &[Todo]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(TODO_FILE)?;

    for todo in todos {
        writeln!(file, "{}", todo.to_line())?;
    }

    Ok(())
}

fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("Список задач пуст.");
        return;
    }

    println!("\n--- Список задач ---");
    for todo in todos {
        let status = if todo.done { "[x]" } else { "[ ]" };
        println!("{} {} {}", status, todo.id, todo.text);
    }
    println!("--------------------\n");
}

fn add_todo(todos: &mut Vec<Todo>, text: String) {
    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    todos.push(Todo {
        id: next_id,
        text,
        done: false,
    });
    println!("Задача добавлена (id: {})", next_id);
}

fn mark_done(todos: &mut Vec<Todo>, id: usize) {
    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            todo.done = true;
            println!("Задача {} отмечена как выполненная.", id);
        }
        None => println!("Задача с id {} не найдена.", id),
    }
}

fn remove_todo(todos: &mut Vec<Todo>, id: usize) {
    let len_before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < len_before {
        println!("Задача {} удалена.", id);
    } else {
        println!("Задача с id {} не найдена.", id);
    }
}

fn print_help() {
    println!(
        r#"
Использование:
  Initial list                  - показать все задачи
  Initial add "текст задачи"    - добавить задачу
  Initial done <id>             - отметить задачу выполненной
  Initial remove <id>           - удалить задачу
  Initial help                  - показать эту справку
"#
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = args[1].as_str();

    let mut todos = match load_todos() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Ошибка чтения файла: {}", e);
            return;
        }
    };

    match command {
        "list" => {
            list_todos(&todos);
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Ошибка: укажите текст задачи.");
                print_help();
                return;
            }
            let text = args[2..].join(" ");
            add_todo(&mut todos, text);
            if let Err(e) = save_todos(&todos) {
                eprintln!("Ошибка сохранения: {}", e);
            }
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Ошибка: укажите id задачи.");
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => {
                    mark_done(&mut todos, id);
                    if let Err(e) = save_todos(&todos) {
                        eprintln!("Ошибка сохранения: {}", e);
                    }
                }
                Err(_) => eprintln!("Ошибка: id должен быть числом."),
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Ошибка: укажите id задачи.");
                return;
            }
            match args[2].parse::<usize>() {
                Ok(id) => {
                    remove_todo(&mut todos, id);
                    if let Err(e) = save_todos(&todos) {
                        eprintln!("Ошибка сохранения: {}", e);
                    }
                }
                Err(_) => eprintln!("Ошибка: id должен быть числом."),
            }
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            eprintln!("Неизвестная команда: {}", command);
            print_help();
        }
    }
}
