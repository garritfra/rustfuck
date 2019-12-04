use crate::instructions::*;

pub fn parse(op_codes: Vec<OpCode>) -> Vec<Instruction> {
  let mut instructions: Vec<Instruction> = Vec::new();
  let mut loop_stack = 0;
  let mut loop_start = 0;

  for (i, code) in op_codes.iter().enumerate() {
    if loop_stack == 0 {
      let instruction = match code {
        OpCode::Increment => Some(Instruction::Increment),
        OpCode::Decrement => Some(Instruction::Decrement),
        OpCode::MoveRight => Some(Instruction::MoveRight),
        OpCode::MoveLeft => Some(Instruction::MoveLeft),
        OpCode::Print => Some(Instruction::Print),
        OpCode::Read => Some(Instruction::Read),
        OpCode::LoopStart => {
          loop_start = i;
          loop_stack += 1;
          None
        }
        OpCode::LoopEnd => panic!("Loop ending at #{} has no beginning", i),
      };
      match instruction {
        Some(instruction) => instructions.push(instruction),
        None => (),
      }
    } else {
      match code {
        OpCode::LoopStart => {
          loop_stack += 1;
        }
        OpCode::LoopEnd => {
          loop_stack -= 1;
          if loop_stack == 0 {
            instructions.push(Instruction::Loop(parse(
              op_codes[loop_start + 1..i].to_vec(),
            )))
          }
        }
        _ => (),
      }
    }
  }

  if loop_stack != 0 {
    panic!(
      "loop that starts at #{} has no matching ending!",
      loop_start
    );
  }

  instructions
}
