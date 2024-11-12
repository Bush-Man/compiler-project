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
        Self { tokens, current: 0 }
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
         while let Some(operator) = self.parse_binary_operator(){
            // print!("parse_binary_expression: {:?}",operator);
            // let operator_precedence = operator.precedence();
            // if operator_precedence < precedence {
            //     break;
            // }
            let right = self.parse_binary_expression(0)?;
            
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
        let kind = match token.kind {
            TokenKind::Number(number) => AstExpressionKind::Number(AstNumberExpression{number}),
            _ => return None,
        };
        // print!("parse_primary_expression: {:?}",kind);

        Some(AstExpression { kind })
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
