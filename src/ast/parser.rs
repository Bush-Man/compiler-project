use super::{lexer::{ Token, TokenKind}, AstExpression, AstExpressionKind, AstStatement, AstStatementKind};

pub struct Parser{
   pub(crate) tokens:Vec<Token>,
   pub(crate)  current:usize
}


impl Parser{
    pub fn new()->Parser{
        Parser{
            tokens: Vec::new(),
            current:0

        }
    }

    pub fn from_tokens(tokens:Vec<Token>)->Self{
        Self{tokens,current:0}
    }

    //gets a statement from a token.
    pub fn next_statement(&mut self)->Option<AstStatement>{
        // must convert from Token to AstStatement for each TokenKind, we return the equivalent AstExpressionKind 
        let expr =  self.parse_statement()?;
        let ast_stmt = AstStatement{kind:AstStatementKind::Expression(expr)};
        return Some(ast_stmt);
    }
    fn parse_statement(&mut self)->Option<AstExpression>{
        let token = self.current()?;
    
        match token.kind{
            TokenKind::Number(number) => {
               let number_expression = AstExpressionKind::Number(super::AstNumberExpression { number });
               return Some(AstExpression{kind: number_expression});
            },
            
            _ => None
           
        }
    }

    fn peek(&self,offset:usize)->Option<&Token>{
        self.tokens.get(self.current + offset)
    }
    fn current(&self)->Option<&Token>{
        self.peek(0)
    }
}