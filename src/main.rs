mod instructions;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let mut tape: Vec<u8> = vec![0; 1024];
    for _i in 0..1024 {
        tape.push(60);
    }
    let mut ptr = 512;

    interpreter::run(
        &parser::parse(lexer::lex("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
        ")),
        &mut tape,
        &mut ptr,
    )
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
