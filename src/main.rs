use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct Task {
    name: String,
    done: bool,
}

impl Task {
    fn from_line(line: &str) -> Task {
        let parts: Vec<&str> = line.splitn(2, ',').collect();
        Task {
            done: parts[0] == "1",
            name: parts[1].to_string(),
        }
    }

    fn to_line(&self) -> String {
        format!("{},{}", if self.done { "1" } else { "0" }, self.name)
    }
}

fn load_tasks() -> Vec<Task> {
    let file = fs::File::open("tasks.txt").unwrap_or_else(|_| OpenOptions::new()
        .write(true).create(true).open("tasks.txt").unwrap());
    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| Task::from_line(&line))
        .collect()
}

fn save_tasks(tasks: &Vec<Task>) {
    let mut file = fs::File::create("tasks.txt").unwrap();
    for task in tasks {
        writeln!(file, "{}", task.to_line()).unwrap();
    }
}

fn main() {
    let mut tasks = load_tasks();

    println!("1. List tasks\n2. Add task\n3. Toggle task\n4. Exit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => {
            for (i, task) in tasks.iter().enumerate() {
                println!("{} [{}] {}", i, if task.done { "x" } else { " " }, task.name);
            }
        }
        "2" => {
            println!("Enter task name:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            tasks.push(Task { name: input.trim().to_string(), done: false });
        }
        "3" => {
            println!("Enter task number:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            if let Ok(i) = input.trim().parse::<usize>() {
                if let Some(task) = tasks.get_mut(i) {
                    task.done = !task.done;
                }
            }
        }
        _ => return,
    }

    save_tasks(&tasks);
}
