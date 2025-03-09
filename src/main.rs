
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
struct Task {
    id: u64,
    name: String,
    isCompleted: bool,
}

impl Task {
    fn new(name: String) -> Self {
        Task {
            id,
            name,
            isCompleted: false,
        }
    }
}

fn main() {
    let task1 = Task::new("Task 1".to_string());
    println!("{:?}", task1);
}