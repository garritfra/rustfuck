mod instructions;
mod lexer;
use instructions::OpCode;

fn main() {
    let tokens: Vec<OpCode> = lexer::lex("++++++   [ > ++++++++++ < - ] > +++++ .");
    for token in tokens {
        println!("{:?}", token);
    }
}
