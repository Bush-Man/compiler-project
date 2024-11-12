use ast::{lexer::Lexer, parser::Parser, Ast};

mod ast;


fn main() {
     let input:&str = "7";
    let mut lexer =Lexer::new(input);
    let mut tokens = Vec::new();
    if let Some(token) = lexer.next_token(){

        tokens.push(token);

    }
    // print!("{:?}",tokens);

    let mut  ast = Ast::new();
    let mut parser = Parser::from_tokens(tokens);
     if let Some(stmt) = parser.next_statement(){
         ast.add_statement(stmt);
     }
     ast.visualize();
}
