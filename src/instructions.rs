#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Print,
  Read,
  LoopStart,
  LoopEnd,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Print,
  Read,
  Loop(Vec<Instruction>),
}
