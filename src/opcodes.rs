#[derive(Debug)]
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
