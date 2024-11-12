use ast::{lexer::Lexer, parser::Parser, Ast};

mod ast;


fn main() {
     let input:&str = "1+7*6+3-4";
    let mut lexer =Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        
        tokens.push(token);
        
    }


    let mut  ast = Ast::new();

    let mut parser = Parser::from_tokens(tokens);
     while let Some(stmt) = parser.next_statement(){

         ast.add_statement(stmt);
     }
    // print!("{:?}",ast);

 ast.visualize();
}
