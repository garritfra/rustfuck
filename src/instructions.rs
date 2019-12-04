#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Instruction {
  Increment,
  Decrement,
  MoveRight,
  MoveLeft,
  Print,
  Read,
  Loop(Vec<OpCode>),
}
