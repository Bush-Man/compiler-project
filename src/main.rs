use ast::{evaluator::AstEvaluator, lexer::{Lexer, TokenKind}, parser::Parser, Ast};

mod ast;


fn main() {
     let input:&str = "4 * 4/6 +5-4";
    let mut lexer =Lexer::new(input);

    let mut tokens = Vec::new();
while let Some(token) = lexer.next_token() {
    if token.kind == TokenKind::Bad || token.kind == TokenKind::Eof {
        break;
    }
    tokens.push(token);
}

// let mut token = lexer.next_token().unwrap();
// while token.kind != TokenKind::Bad && token.kind != TokenKind::Eof {
//     tokens.push(token.clone());
//     token = lexer.next_token().unwrap();
// }

    

    let mut  ast = Ast::new();

    let mut parser = Parser::from_tokens(tokens);
     while let Some(stmt) = parser.next_statement(){

         ast.add_statement(stmt);

     }
ast.visualize();

let mut eval = AstEvaluator::new();
ast.visit(&mut eval);

print!("Results: {:?}",eval.last_value.unwrap());
}
