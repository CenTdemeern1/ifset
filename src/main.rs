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

struct InterpreterState {
    memory: HashMap<String, String>,
    functions: HashMap<String, Position>,
    stack: Vec<Position>,
    current_position: Position,
    term: Term,
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

fn sanitize_value(value: String, state: &InterpreterState) -> String {
    let mut value = value.clone();
    while value.contains(r"\INPUT") {
        let input = state.term.read_char().expect("Couldn't read input.").to_string();
        value = value.replacen(r"\INPUT", &input, 1);
    }
    value.replace(r"\n", "\n").replace(r"\r", "\r").replace(r"\\", "\\")
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

fn run_line(line: &String, state: &mut InterpreterState) {
    let stripped_line = line.trim_start_matches("\t");
    let line_indentation = line.len() - stripped_line.len();
    if line_indentation > state.current_position.indentation {
        return;
    }
    if line_indentation < state.current_position.indentation {
        state.current_position.indentation = line_indentation;
    }
    let line = stripped_line.to_string();
    match match_assignment(&line) {
        Some((key, value)) => {
            let value = sanitize_value(value, &state);
            if key == "OUTPUT" {
                print!("{}", value);
                return;
            }
            state.memory.insert(key, value);
            return;
        },
        None => (),
    }
    match match_if(&line) {
        Some((key, value)) => {
            let value = sanitize_value(value, &state);
            match state.memory.get(&key) {
                Some(mem_value) => {
                    if value == *mem_value { // Is dereferencing like this safe? I sure hope it is...
                        state.current_position.indentation += 1;
                    }
                }
                None => panic!("Variable {:?} is not defined", key),
            };
            return;
        },
        None => (),
    }
    match match_def(&line) {
        Some(functionname) => {
            let mut functionpos = state.current_position.clone();
            functionpos.indentation += 1;
            state.functions.insert(functionname, functionpos);
            return;
        },
        None => (),
    }
    if line == "RETURN" {
        match state.stack.pop() {
            Some(v) => state.current_position = v.clone(),
            None => panic!("There is nothing on the stack to return to."),
        }
        return
    }
    if line == "LOOP" {
        if state.stack.len() == 0 {
            return
        }
        match state.stack.get(state.stack.len()-1) {
            Some(v) => state.current_position = v.clone(),
            None => (),
        }
        return
    }
    // if line == "debug" {
    //     println!("{} - {}: {} {} {} {} {} {} {}", state.current_position.linenumber, state.memory["cell"], state.memory["cell0"], state.memory["cell1"], state.memory["cell2"], state.memory["cell3"], state.memory["cell4"], state.memory["cell5"], state.memory["cell6"]);
    //     return
    // }
    match state.functions.get(&line) {
        Some(pos) => {
            state.stack.push(state.current_position.clone());
            state.current_position = pos.clone();
            return // Not really needed, but good for consistency
        },
        None => panic!("Function {:?} not found", line)
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
        match scriptlines.get(state.current_position.linenumber) {
            Some(line) => run_line(&line, &mut state),
            None => break 'mainloop,
        }
        state.current_position.linenumber += 1;
    }
}
