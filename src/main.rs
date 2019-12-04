mod instructions;
mod interpreter;
mod lexer;
mod parser;

use std::fs::File;
use std::io::Read;

fn main() {
    // Determine which file to execute
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: rustfuck <file.bf>");
        std::process::exit(1);
    }
    let filename = &args[1];

    // Read file
    let mut file = File::open(filename).expect("program file not found");
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("failed to read program file");

    let mut tape: Vec<u8> = vec![0; 1024];
    for _i in 0..1024 {
        tape.push(60);
    }
    let mut ptr = 512;

    interpreter::run(&parser::parse(lexer::lex(&source)), &mut tape, &mut ptr)
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;
    #[test]
    fn when_input_contains_loop_then_loop_gets_parsed() {
        let input = "+[-].";
        let tokens = super::lexer::lex(input);
        let instructions: Vec<Instruction> = super::parser::parse(tokens);
        println!("{:?}", instructions);
        assert_eq!(instructions.len(), 3);
        assert_eq!(*instructions.get(0).unwrap(), Instruction::Increment);
        assert_eq!(
            *instructions.get(1).unwrap(),
            Instruction::Loop(vec![Instruction::Decrement])
        );
        assert_eq!(*instructions.get(2).unwrap(), Instruction::Print);
    }

    #[test]
    fn when_input_contains_nested_loop_then_loops_get_parsed() {
        let input = "+[-[++]].";
        let tokens = super::lexer::lex(input);
        let instructions: Vec<Instruction> = super::parser::parse(tokens);
        println!("{:?}", instructions);
        assert_eq!(instructions.len(), 3);
        assert_eq!(*instructions.get(0).unwrap(), Instruction::Increment);
        assert_eq!(
            *instructions.get(1).unwrap(),
            Instruction::Loop(vec![
                Instruction::Decrement,
                Instruction::Loop(vec![Instruction::Increment, Instruction::Increment])
            ],)
        );
        assert_eq!(*instructions.get(2).unwrap(), Instruction::Print);
    }
}
