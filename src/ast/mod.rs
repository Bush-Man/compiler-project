use core::num;

use lexer::TextSpan;

pub mod lexer;
pub mod parser;


#[derive(Debug)]
pub struct Ast{
    pub statements: Vec<AstStatement>,
  
}
pub struct AstPrinter{
    indent: usize
}

#[derive(Debug)]
pub struct AstStatement{
    kind:AstStatementKind,
}

#[derive(Debug)]
pub enum AstStatementKind{
    Expression(AstExpression)
}
#[derive(Debug)]
pub struct AstExpression{
    kind :AstExpressionKind
}
#[derive(Debug)]
pub enum AstExpressionKind{
    Number(AstNumberExpression)
}
#[derive(Debug)]
pub struct AstNumberExpression{
    number: i64
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
            }
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
}