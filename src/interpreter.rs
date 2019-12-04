use crate::instructions::Instruction;
use std::io::Read;

pub fn run(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, ptr: &mut usize) {
  for instruction in instructions {
    match instruction {
      Instruction::Increment => tape[*ptr] += 1,
      Instruction::Decrement => tape[*ptr] -= 1,
      Instruction::MoveRight => *ptr += 1,
      Instruction::MoveLeft => *ptr -= 1,
      Instruction::Print => print!("{}", tape[*ptr] as char),
      Instruction::Loop(nested) => {
        while tape[*ptr] != 0 {
          run(&nested, tape, ptr)
        }
      }
      Instruction::Read => {
        let mut input: [u8; 1] = [0; 1];
        std::io::stdin()
          .read_exact(&mut input)
          .expect("failed to read stdin");

        tape[*ptr] = input[0];
      }
    }
  }
}
