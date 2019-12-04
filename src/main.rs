mod lexer;
mod opcodes;
use opcodes::OpCode;

fn main() {
    let tokens: Vec<OpCode> = lexer::lex("++++++   [ > ++++++++++ < - ] > +++++ .");
    for token in tokens {
        println!("{:?}", token);
    }
}
