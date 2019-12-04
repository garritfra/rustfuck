use crate::instructions::*;

pub fn parse(op_codes: Vec<OpCode>) -> Vec<Instruction> {
  let mut instructions: Vec<Instruction> = Vec::new();

  let mut codes = op_codes.iter();
  loop {
    if let Some(code) = codes.next() {
      match code {
        OpCode::Increment => instructions.push(Instruction::Increment),
        OpCode::Decrement => instructions.push(Instruction::Decrement),
        OpCode::MoveRight => instructions.push(Instruction::MoveRight),
        OpCode::MoveLeft => instructions.push(Instruction::MoveLeft),
        OpCode::Print => instructions.push(Instruction::Print),
        OpCode::Read => instructions.push(Instruction::Read),
        OpCode::LoopStart => {
          let codes_ref = &mut codes;
          let loop_opcodes: Vec<OpCode> = codes_ref
            .take_while(|x| **x != OpCode::LoopEnd)
            .cloned()
            .collect();
          instructions.push(Instruction::Loop(parse(loop_opcodes)));
        }
        OpCode::LoopEnd => panic!("Extra ] found"),
      }
    } else {
      break;
    }
  }

  instructions
}
