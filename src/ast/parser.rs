use crate::ast::AstNumberExpression;

use super::{lexer::{Token, TokenKind}, AstBinOperator, AstBinOperatorKind, AstExpression, AstExpressionKind, AstStatement, AstStatementKind};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { tokens:tokens.into_iter().filter(|tok| tok.kind != TokenKind::Whitespace).collect()
            , current: 0 }
    }

    // creates a statement from a token.
    pub fn next_statement(&mut self) -> Option<AstStatement> {
        
        let expr = self.parse_expression()?;
        let ast_stmt = AstStatement {
            kind: AstStatementKind::Expression(expr),
        };
        Some(ast_stmt)
    }

    // Takes a binary token and gives the equivalent AstBinOperator.
   fn parse_binary_expression(&mut self, precedence: u8) -> Option<AstExpression> {
    let mut left = self.parse_primary_expression()?;
    
    while let Some(operator) = self.parse_binary_operator() {
        let operator_precedence = operator.precedence();

        // If the operator's precedence is less than the current precedence, stop parsing.
        if operator_precedence < precedence {
            break;
        }

        // Handle the next operator with higher precedence in a recursive call
        let mut right = self.parse_primary_expression()?;
        
        // Check for right-associative operators by using precedence + 1
        while let Some(next_op) = self.parse_binary_operator() {
            let next_op_precedence = next_op.precedence();
            if next_op_precedence <= operator_precedence {
                self.current -= 1; // Backtrack one token to reprocess this operator
                break;
            }
            right = AstExpression::binary(next_op, right, self.parse_binary_expression(next_op_precedence)?);
        }

        left = AstExpression::binary(operator, left, right);
    }
    
    Some(left)
}


    fn parse_binary_operator(&mut self) -> Option<AstBinOperator> {
        let token = self.consume()?;

        let kind = match token.kind {
            TokenKind::Asterisk => AstBinOperatorKind::Multiply,
            TokenKind::Minus => AstBinOperatorKind::Minus,
            TokenKind::Plus => AstBinOperatorKind::Plus,
            TokenKind::Slash => AstBinOperatorKind::Divide,
            _ => return None, 
        };
        
         Some(AstBinOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> Option<AstExpression> {
        
       
        let token = self.consume()?;

            if token.kind == TokenKind::Eof {
            return None;
             }
             match token.kind {
            TokenKind::Number(number) =>{ 
                AstExpressionKind::Number(AstNumberExpression{number});
                return Some(AstExpression::number(number));
            },
            TokenKind::LeftParam =>{
                //  self.consume()?;
                let expr = self.parse_binary_expression(0)?;
               return Some(expr);


            },
            TokenKind::RightParam=>{
                // self.consume();
                let expr = self.parse_binary_expression(0)?;
                return Some(expr);
            }

            _ => return None,

        };
    
        
    
        
        // print!("parse_primary_expression: {:?}",kind);
    
    }

    fn parse_expression(&mut self) -> Option<AstExpression> {
        self.parse_binary_expression(0)
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<Token> {  
        let token = self.tokens.get(self.current)?.clone();  
        self.current += 1;
        Some(token)
    }
}
