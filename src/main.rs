mod instructions;
mod lexer;
mod parser;
use instructions::Instruction;
use instructions::OpCode;

fn main() {
    let tokens: Vec<OpCode> = lexer::lex("+[-[.]-].");

    let instructions: Vec<Instruction> = parser::parse(tokens);

    for instruction in &instructions {
        println!("{:?}", instruction);
    }
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
