use ast::lexer::Lexer;

mod ast;


fn main() {
     let input:&str = "7";
    let mut lexer =Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);

    }
    print!("{:?}",tokens);
}
