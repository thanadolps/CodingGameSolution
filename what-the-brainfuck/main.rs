use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

struct ParsedInput {
    memory_size: usize,
    n_program_line: usize,
    n_input_line: usize,
}

fn parse_line_1() -> ParsedInput {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], usize);
    let s = parse_input!(inputs[1], usize);
    let i = parse_input!(inputs[2], usize);
    ParsedInput {
        memory_size: s,
        n_program_line: l,
        n_input_line: i,
    }
}

#[derive(Debug)]
enum BrainFuckError {
    SyntaxError,
    PointerOutOfBound,
    IncorrectValue,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Right,
    Left,
    Add,
    Sub,
    Output,
    Input,
    LeftBracket,
    RightBracket,
}
impl TryFrom<char> for Operation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Operation::Right),
            '<' => Ok(Operation::Left),
            '+' => Ok(Operation::Add),
            '-' => Ok(Operation::Sub),
            '.' => Ok(Operation::Output),
            ',' => Ok(Operation::Input),
            '[' => Ok(Operation::LeftBracket),
            ']' => Ok(Operation::RightBracket),
            _ => Err(()),
        }
    }
}

fn parse_code(source_code: &str) -> Vec<Operation> {
    source_code
        .chars()
        .filter_map(|c| Operation::try_from(c).ok())
        .collect()
}

#[derive(Debug)]
struct CompiledCode {
    code: Vec<Operation>,
    redirection: HashMap<usize, usize>,
}

fn compile_code(code: Vec<Operation>) -> Result<CompiledCode, BrainFuckError> {
    let cap = code
        .iter()
        .filter(|&&op| op == Operation::RightBracket || op == Operation::LeftBracket)
        .count();
    let mut redirection = HashMap::with_capacity(cap);
    let mut left_bracket_stack = Vec::new();

    for (i, &c) in code.iter().enumerate() {
        match c {
            Operation::LeftBracket => {
                left_bracket_stack.push(i);
            }
            Operation::RightBracket => {
                let li = left_bracket_stack
                    .pop()
                    .ok_or(BrainFuckError::SyntaxError)?;
                redirection.insert(li, i);
                redirection.insert(i, li);
            }
            _ => {}
        }
    }
    if !left_bracket_stack.is_empty() {
        return Err(BrainFuckError::SyntaxError);
    }
    /*
    for (&k, &v) in redirection.iter() {
        assert!(code[k] == Operation::LeftBracket || code[k] == Operation::RightBracket);
        assert!(code[v] == Operation::LeftBracket || code[v] == Operation::RightBracket);
    }*/
    Ok(CompiledCode { code, redirection })
}

fn run(
    memory_size: usize,
    compiled_code: CompiledCode,
    input: &[u8],
) -> Result<(), BrainFuckError> {
    let CompiledCode { code, redirection } = compiled_code;

    let mut m_vec = vec![0_u8; memory_size];
    let memory = m_vec.as_mut_slice();

    let mut mem_loc = 0;
    let mut code_loc = 0;

    let mut input_iter = input.iter();

    while code_loc < code.len() {
        let op = code[code_loc];
        match op {
            Operation::Right => {
                mem_loc += 1;
                if mem_loc >= memory_size {
                    return Err(BrainFuckError::PointerOutOfBound);
                }
            }
            Operation::Left => {
                mem_loc = mem_loc
                    .checked_sub(1)
                    .ok_or(BrainFuckError::PointerOutOfBound)?;
            }
            Operation::Add => {
                memory[mem_loc] = memory[mem_loc]
                    .checked_add(1)
                    .ok_or(BrainFuckError::IncorrectValue)?;
            }
            Operation::Sub => {
                memory[mem_loc] = memory[mem_loc]
                    .checked_sub(1)
                    .ok_or(BrainFuckError::IncorrectValue)?;
            }
            Operation::Output => {
                print!("{}", memory[mem_loc] as char);
            }
            Operation::Input => {
                memory[mem_loc] = *input_iter.next().expect("Input can't be missing");
            }
            Operation::LeftBracket => {
                // eprintln!("{}", mem_loc);
                if memory[mem_loc] == 0 {
                    code_loc = *redirection
                        .get(&code_loc)
                        .expect("matching right bracket must exist");
                    if mem_loc >= memory.len() - 1 {
                        break;
                    }
                }
            }
            Operation::RightBracket => {
                // eprintln!("{}", mem_loc);
                if memory[mem_loc] != 0 {
                    code_loc = *redirection
                        .get(&code_loc)
                        .expect("matching left bracket must exist");
                }
            }
        }
        code_loc += 1;
    }

    Ok(())
}

/**c
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    match inner_main() {
        Err(BrainFuckError::SyntaxError) => println!("SYNTAX ERROR"),
        Err(BrainFuckError::IncorrectValue) => println!("INCORRECT VALUE"),
        Err(BrainFuckError::PointerOutOfBound) => println!("POINTER OUT OF BOUNDS"),
        Ok(()) => (),
    }
}

fn inner_main() -> Result<(), BrainFuckError> {
    let parsed_input = parse_line_1();

    let mut source_code = String::new();
    for _ in 0..parsed_input.n_program_line {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        source_code += input_line.trim_matches('\n');
    }
    let compiled_code = compile_code(parse_code(&source_code))?;

    let mut input = Vec::new();
    for _ in 0..parsed_input.n_input_line {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input.push(parse_input!(input_line, u8));
    }
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    run(parsed_input.memory_size, compiled_code, &input)?;

    Ok(())
}
