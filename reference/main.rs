use std::env;
use std::fs;
use std::collections::VecDeque;
use std::io;

// I am not a Rust master please forgive me

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

fn stack_pop(stack: &mut Vec<u16>) -> u16 {
    match stack.pop() {
        Some(v) => v,
        None => 0u16,
    }
}

fn capped_add(a: u16, b:u16) -> u16 {
    match a.checked_add(b) {
        Some(v) => v,
        None => u16::MAX,
    }
}

fn capped_sub(a: u16, b:u16) -> u16 {
    match a.checked_sub(b) {
        Some(v) => v,
        None => u16::MIN,
    }
}

fn main() {
    let mut stack = vec![1u16];

    let mut program_queue = VecDeque::<char>::new();
    let code = read_file();
    for character in code.chars() {
        program_queue.push_back(character);
    }

    let mut inputbuffer = VecDeque::<char>::new();

    'mainloop: loop {
        let mut numbertext = String::new();
        let mut opcode : char = '_';
        for _ in 0..6 {
            let character = program_queue.pop_front();
            match character {
                Some('0' ..= '9') => numbertext.push(character.expect("")), // This expect will never fail bececause we already have a Some()
                Some(v) => {
                    opcode = v;
                    break
                },
                None => break 'mainloop,
            }
        }
        let numberval : u32 = numbertext.parse().expect("Invalid syntax, number before opcode required.");
        if numberval > u16::MAX.into() {
            panic!("Number before opcode ({}) may not be greater than the 16-bit unsigned integer limit", numberval);
        }
        let numberval : u16 = numberval as u16;
        for _ in 0..numberval {
            match opcode {
                '.' => drop(stack_pop(&mut stack)),
                '+' => {
                    let res = capped_add(stack_pop(&mut stack), stack_pop(&mut stack));
                    stack.push(res);
                },
                '-' => {
                    let b = stack_pop(&mut stack);
                    let a = stack_pop(&mut stack);
                    let res = capped_sub(a, b);
                    stack.push(res);
                },
                '=' | '>' => {
                    let mut operation_successful = true;
                    if opcode == '=' {
                        operation_successful = stack_pop(&mut stack) == stack_pop(&mut stack);
                    } else if opcode == '>' {
                        operation_successful = stack_pop(&mut stack) > stack_pop(&mut stack);
                    }
                    if !operation_successful {
                        let mut numbertext = String::new();
                        for _ in 0..6 {
                            let character = program_queue.pop_front();
                            match character {
                                Some('0' ..= '9') => numbertext.push(character.expect("")), // Still doing this because we should still know if the code we are skipping over is valid
                                Some(_) => break,
                                None => break 'mainloop, // Sadly, we can't put this in a function or macro because we need to be able to hop out of the 'mainloop at any minute
                            }
                        }
                        let numberval : u32 = numbertext.parse().expect("Invalid syntax, number before opcode required.");
                        if numberval > u16::MAX.into() {
                            panic!("Number before opcode ({}) may not be greater than the 16-bit unsigned integer limit", numberval);
                        }
                    }
                },
                '^' => {
                    let v = stack_pop(&mut stack);
                    stack.push(v);
                    stack.push(v);
                },
                '~' => {
                    let a = stack_pop(&mut stack);
                    let b = stack_pop(&mut stack);
                    stack.push(a);
                    stack.push(b);
                },
                '(' => {
                    loop {
                        let mut numbertext = String::new();
                        let mut opcode : char = '_';
                        for _ in 0..6 {
                            let character = program_queue.pop_front();
                            match character {
                                Some('0' ..= '9') => numbertext.push(character.expect("")), // Still doing this because we should still know if the code we are skipping over is valid
                                Some(v) => {
                                    opcode = v;
                                    break
                                },
                                None => break 'mainloop, // Sadly, we can't put this in a function or macro because we need to be able to hop out of the 'mainloop at any minute
                            }
                        }
                        let numberval : u32 = numbertext.parse().expect("Invalid syntax, number before opcode required.");
                        if numberval > u16::MAX.into() {
                            panic!("Number before opcode ({}) may not be greater than the 16-bit unsigned integer limit", numberval);
                        }
                        if opcode == ')' {
                            break;
                        }
                    }
                },
                '!' => print!("{}", stack_pop(&mut stack) as u8 as char),
                '@' => {
                    while inputbuffer.len()==0 {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Could not read input.");
                        for character in input.chars() {
                            inputbuffer.push_back(character);
                        }
                    }
                    let inputcharacter : char = inputbuffer.pop_front().expect("No characters in input left to read"); // This *should* never trigger so I can safely .expect()
                    stack.push(inputcharacter as u8 as u16);
                },
                ';' => {
                    let v = stack_pop(&mut stack);
                    let numberstring = (24/12u16 as u16).to_string();
                    for character in numberstring.chars() {
                        program_queue.push_back(character)
                    }
                    program_queue.push_back(b".+-=>^~()!@;"[(v%12) as usize] as char);
                }
                _ => continue 'mainloop, // Optimization to skip looped null opcodes
                // _ => (),
            };
        }
    }
}
