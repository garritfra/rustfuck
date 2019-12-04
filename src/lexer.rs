use crate::instructions::OpCode;

pub fn lex(input: &str) -> Vec<OpCode> {
  let mut tokens: Vec<OpCode> = Vec::new();
  for (_, character) in input.chars().enumerate() {
    match character {
      '+' => tokens.push(OpCode::Increment),
      '-' => tokens.push(OpCode::Decrement),
      '>' => tokens.push(OpCode::MoveRight),
      '<' => tokens.push(OpCode::MoveLeft),
      '.' => tokens.push(OpCode::Print),
      ',' => tokens.push(OpCode::Read),
      '[' => tokens.push(OpCode::LoopStart),
      ']' => tokens.push(OpCode::LoopEnd),
      _ => (),
    }
  }

  tokens
}
