use std::collections::HashMap;
use std::io::{self, Write};

fn read_input(msg: &str) -> Option<String> {
    print!("{}: ", msg);
    io::stdout().flush().expect("Error flushing");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input.trim().to_owned()),
        Err(_) => None,
    }
}

fn new_line() {
    println!("");
}

struct State {
    users: HashMap<String, Vec<String>>,
}

impl State {
    fn new() -> State {
        State {
            users: HashMap::new(),
        }
    }

    fn add_user(&mut self, user: String, department: String) {
        self.users.entry(department).or_insert(vec![]).push(user);
    }

    fn list_department(&self, department: String) {
        match self.users.get(&department) {
            Some(department) => println!("{:?}", department),
            None => println!("The department does not exist."),
        }
    }
}

#[derive(Debug)]
enum Command {
    ADD,
    LIST,
}

impl Command {
    fn execute(&self, state: &mut State) {
        match self {
            Command::ADD => {
                let user =
                    read_input("input the user's name").expect("failed to read the user's name");

                let department = read_input("input the department's name")
                    .expect("failed to read the department's name");

                new_line();
                println!("added user {} to department {}.", user, department);

                state.add_user(user, department);
            }
            Command::LIST => {
                let department = read_input("input the department's name")
                    .expect("failed to read the department's name");

                new_line();
                state.list_department(department);
            }
        }
    }
}

fn main() {
    let mut state = State::new();

    new_line();

    loop {
        println!("Welcome!");
        println!("\t1. Add a user to a department");
        println!("\t2. List users in a department");
        new_line();

        let command = read_input("choose an option").expect("failed to read an option");

        let command: Command = match command.trim() {
            "1" => Some(Command::ADD),
            "2" => Some(Command::LIST),
            _ => None,
        }
        .expect("failed to parse the option");

        new_line();

        command.execute(&mut state);

        new_line();
    }
}
