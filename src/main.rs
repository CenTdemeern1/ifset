extern crate console;
use console::Term;
use std::collections::HashMap;
use std::env;
use std::fs;

// #[derive(Debug)]
#[derive(Clone)]
struct Position {
    linenumber : usize,
    indentation : usize,
}

fn read_file() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("No file name was provided.");
    }
    let filename = &args[1];
    let code = fs::read_to_string(filename)
        .expect("Could not open file.");
    code
}

struct InterpreterState {
    memory: HashMap<String, String>,
    functions: HashMap<String, Position>,
    stack: Vec<Position>,
    current_position: Position,
    term: Term,
}

impl InterpreterState {
    fn sanitize_value(&self, value: String) -> String {
        let mut value = value.clone();
        while value.contains(r"\INPUT") {
            let input = self.term.read_char().expect("Couldn't read input.").to_string();
            value = value.replacen(r"\INPUT", &input, 1);
        }
        value.replace(r"\n", "\n").replace(r"\r", "\r").replace(r"\\", "\\")
    }

    fn run_line(&mut self, line: &String) {
        let stripped_line = line.trim_start_matches("\t");
        let line_indentation = line.len() - stripped_line.len();
        if line_indentation > self.current_position.indentation {
            return;
        }
        if line_indentation < self.current_position.indentation {
            self.current_position.indentation = line_indentation;
        }
        let line = stripped_line.to_string();
        if let Some((key, value)) = match_assignment(&line) {
            let value = self.sanitize_value(value);
            if key == "OUTPUT" {
                print!("{}", value);
                return;
            }
            self.memory.insert(key, value);
            return;
        }
        if let Some((key, value)) =  match_if(&line) {
            let value = self.sanitize_value(value);
            match self.memory.get(&key) {
                Some(mem_value) => {
                    if value == *mem_value {
                        self.current_position.indentation += 1;
                    }
                }
                None => panic!("Variable {:?} is not defined", key),
            };
            return;
        }
        if let Some(functionname) = match_def(&line) {
            let mut functionpos = self.current_position.clone();
            functionpos.indentation += 1;
            self.functions.insert(functionname, functionpos);
            return;
        }
        if line == "RETURN" {
            match self.stack.pop() {
                Some(v) => self.current_position = v.clone(),
                None => panic!("There is nothing on the stack to return to."),
            }
            return;
        }
        if line == "LOOP" {
            if self.stack.len() == 0 {
                return;
            }
            match self.stack.get(self.stack.len()-1) {
                Some(v) => self.current_position = v.clone(),
                None => (),
            }
            return;
        }
        if let Some(pos) = self.functions.get(&line) {
            self.stack.push(self.current_position.clone());
            self.current_position = pos.clone();
            return;
        } else {panic!("Function {:?} not found", line)}
    }
}

fn match_assignment(line: &String) -> Option<(String, String)> {
    match line.find(" = ") {
        Some(i) => {
            if i == 0 {
                return None;
            }
            if i > line.len() - 4 {
                return None;
            }
            Some((line[0..i].to_string(), line[i+3..].to_string()))
        },
        None => None,
    }
}

fn match_if(line: &String) -> Option<(String, String)> {
    if !line.starts_with("IF ") {
        return None;
    }
    match line.find(" == ") {
        Some(i) => {
            if i <= 3 {
                return None;
            }
            if i > line.len() - 5 {
                return None;
            }
            Some((line[3..i].to_string(), line[i+4..].to_string()))
        },
        None => None,
    }
}

fn match_def(line: &String) -> Option<String> {
    if line.starts_with("DEF ") & (line.len() > 4) {
        Some(line[4..].to_string())
    } else {
        None
    }
}

fn main() {
    let script = read_file();
    let mut scriptlines = Vec::<String>::new(); 
    for line in script.lines() {
        scriptlines.push(line.to_string());
    };

    let mut state = InterpreterState{
        memory: HashMap::new(),
        functions: HashMap::new(),
        stack: Vec::<Position>::new(),
        current_position: Position{linenumber: 0, indentation: 0},
        term: Term::buffered_stdout(),
    };

    'mainloop: loop {
        if let Some(line) = scriptlines.get(state.current_position.linenumber) {
            state.run_line(&line);
        } else {
            break 'mainloop;
        }
        state.current_position.linenumber += 1;
    }
}
