use core::num;
use std::fmt::Binary;

use lexer::{TextSpan, Token};

pub mod lexer;
pub mod parser;


#[derive(Debug,Clone)]
pub struct Ast{
    pub statements: Vec<AstStatement>,
  
}
pub struct AstPrinter{
    indent: usize
}

#[derive(Debug,Clone)]
pub struct AstStatement{
    kind:AstStatementKind,
}

#[derive(Debug,Clone)]
pub enum AstStatementKind{
    Expression(AstExpression),
}
#[derive(Debug,Clone)]
pub struct AstExpression{
    kind :AstExpressionKind
}
#[derive(Debug,Clone)]
pub enum AstExpressionKind{
    Number(AstNumberExpression),
    Binary(BinaryExpr)

}
#[derive(Debug,Clone)]
pub struct AstNumberExpression{
    number: i64
}
#[derive(Debug,Clone)]
pub enum AstBinOperatorKind{
    Plus,
    Minus,
    Divide,
    Multiply,
    NotDefined
}
#[derive(Debug,Clone)]
pub struct AstBinOperator{
    kind: AstBinOperatorKind,
    token: Token
}
#[derive(Debug,Clone)]
pub struct BinaryExpr{
    left: Box<AstExpression>,
    operator:AstBinOperator,
    right: Box<AstExpression>

}

impl Ast{
    pub fn new()->Ast{
        Ast { statements: Vec::new() }
    }
    pub fn add_statement(&mut self,stmt:AstStatement){
        self.statements.push(stmt);
    }
     pub fn visit(&mut self,visitor: &mut dyn AstVisitor){
        for stmt in &self.statements{
            visitor.visit_statement(stmt);
        }
     }
     pub fn visualize(&mut self){
        let mut printer = AstPrinter{indent:0 };
        self.visit(&mut printer);
     }

    }


pub trait AstVisitor{
   fn visit_expression(&mut self,expr:&AstExpression){
    self.do_visit_expression(expr);
    }
   
   fn visit_statement(&mut self,stmt:&AstStatement){
    self.do_visit_statement(stmt);
   }
   fn visit_binary_expression(&mut self,binary:&BinaryExpr){
    self.visit_expression(&binary.left);
    self.visit_expression(&binary.right);
   }
   
  

    fn do_visit_statement(&mut self,stmt:&AstStatement){
        match &stmt.kind{
            AstStatementKind::Expression(ast_expression) => {
                self.visit_expression(ast_expression);
            }
        }
    }

    fn do_visit_expression(&mut self,expr:&AstExpression){
        match &expr.kind{
            AstExpressionKind::Number(number)=>{
                 self.visit_number(number);
            },
            AstExpressionKind::Binary(expr)=>{
                self.visit_binary_expression(expr);
            }

            _ => todo!("do visit expression not finished")
        }
    }

   fn visit_number(&mut self,number:&AstNumberExpression);
    
}

impl AstPrinter{
     pub fn print_with_indent(&self,text:&str){
        println!("{}{}"," ".repeat(self.indent),text);
        
     }
}
impl AstVisitor for AstPrinter{
    
    fn visit_statement(&mut self,stmt:&AstStatement) {
        self.print_with_indent("Statement:");
        self.indent += 2;
        self.do_visit_statement(stmt);
        self.indent -= 2;

        
    }
    fn visit_expression(&mut self,expr:&AstExpression) {
        self.print_with_indent("Expression:");
        self.indent+=2;
        self.do_visit_expression(expr);
        self.indent -= 2;
    }


    fn visit_number(&mut self,number:&AstNumberExpression) {
        self.print_with_indent(&format!("Number: {}", number.number ));
    }

    fn visit_binary_expression(&mut self,binary:&BinaryExpr) {
        self.print_with_indent("Binary Expression:");
        self.indent+=2;
        self.print_with_indent(&format!("Operator: {:?}",binary.operator));
        self.visit_expression(&binary.left);
        self.visit_expression(&binary.right);
        self.indent -= 2;
    }
}

impl AstExpression{
    pub fn new(kind:AstExpressionKind)->Self{
        Self { kind }
    }

    pub fn number(number:i64)->Self{
     AstExpression::new(AstExpressionKind::Number(AstNumberExpression { number }))
    }
   
    pub fn binary(operator:AstBinOperator,left:AstExpression,right:AstExpression)->Self{
        AstExpression::new(AstExpressionKind::Binary(BinaryExpr{ left: Box::new(left), operator,right:Box::new( right)}))
    }
}

impl AstBinOperator{
    pub fn new(kind:AstBinOperatorKind,token:Token)->AstBinOperator{
        AstBinOperator{ kind, token  }
    }

    pub fn precedence(&self)->u8{
        return match self.kind{
            AstBinOperatorKind::Divide => 2,
            AstBinOperatorKind::Minus => 1,
            AstBinOperatorKind::Multiply => 2,
            AstBinOperatorKind::Plus => 1,
            AstBinOperatorKind::NotDefined => u8::MAX
            
        };
    }
}